use crate::db;
use colored_json::ToColoredJson;
use reqwest::blocking::ClientBuilder;
use reqwest::blocking::Response;
use serde_json::{json, Map, Value};
use crate::Result;

pub fn call_remote_procedure(name: &str, mut params: Value) -> Result<()> {
    let params = params.as_object_mut()
        .ok_or("params value is not a valid JSON object")?;
    params.insert(
        "password".into(),
        db::query_settings_string("password", &db::connect()?)?.into(),
    );
    let http_client = ClientBuilder::new()
        .timeout(None)
        .build()
        .map_err(|e| {
            eprintln!("failed to initialize HTTP client: {e}");
            e
        })?;
    let args = json!(
        {"jsonrpc": "2.0", "method": name, "params": params, "id": 1}
    );
    let args = serde_json::to_string(&args).map_err(|e| {
        eprintln!("failed to convert args to string: {e}");
        e
    })?;
    let mut api_url = db::query_settings_string("api_url", &db::connect()?)?;
    if api_url.trim().is_empty() {
        api_url = "https://api.btcmap.org/rpc".into();
    }
    let res = http_client.post(api_url).body(args).send().map_err(|e| {
        eprintln!("request failed: {e}");
        e
    })?;
    if res.status().is_success() {
        let res = res.json::<Map<String, Value>>().map_err(|e| {
            eprintln!("failed to convert response body to JSON object: {e}");
            e
        })?;
        let res = serde_json::to_string_pretty(&res).map_err(|e| {
            eprintln!("failed to convert JSON object to string: {e}");
            e
        })?;
        println!(
            "{}",
            res.to_colored_json_auto().map_err(|e| {
                eprintln!("failed to create colored JSON: {e}");
                e
            })?
        );
    } else {
        handle_unsuccessful_response(res);
    }
    Ok(())
}

fn handle_unsuccessful_response(res: Response) {
    let status = res.status();
    let mut text = res.text().unwrap_or("empty".into());
    if text.trim().is_empty() {
        text = "empty".into();
    }
    eprintln!("got HTTP status code {}, message: {}", status, text);
}
