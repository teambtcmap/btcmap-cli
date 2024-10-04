use crate::db;
use colored_json::ToColoredJson;
use reqwest::blocking::ClientBuilder;
use reqwest::blocking::Response;
use serde_json::{json, Map, Value};
use std::process::exit;

pub fn call_remote_procedure(name: &str, mut params: Value) {
    let params = params.as_object_mut().unwrap_or_else(|| {
        eprintln!("params value is not a valid JSON object");
        exit(1);
    });
    params.insert(
        "password".into(),
        db::query_settings_string("password", &db::connect()).into(),
    );
    let http_client = ClientBuilder::new()
        .timeout(None)
        .build()
        .unwrap_or_else(|e| {
            eprintln!("failed to initialize HTTP client: {e}");
            exit(1);
        });
    let args = json!(
        {"jsonrpc": "2.0", "method": name, "params": params, "id": 1}
    );
    let args = serde_json::to_string(&args).unwrap_or_else(|e| {
        eprintln!("failed to convert args to string: {e}");
        exit(1);
    });
    let mut api_url = db::query_settings_string("api_url", &db::connect());
    if api_url.trim().is_empty() {
        api_url = "https://api.btcmap.org/rpc".into();
    }
    let res = http_client.post(api_url).body(args).send();
    match res {
        Ok(res) => {
            if res.status().is_success() {
                let res = res.json::<Map<String, Value>>().unwrap_or_else(|e| {
                    eprintln!("failed to convert response body to JSON object: {e}");
                    exit(1);
                });
                let res = serde_json::to_string_pretty(&res).unwrap_or_else(|e| {
                    eprintln!("failed to convert JSON object to string: {e}");
                    exit(1);
                });
                println!(
                    "{}",
                    res.to_colored_json_auto().unwrap_or_else(|e| {
                        eprintln!("failed to create colored JSON: {e}");
                        exit(1);
                    })
                );
            } else {
                handle_unsuccessful_response(res);
            }
        }
        Err(_) => {}
    }
}

fn handle_unsuccessful_response(res: Response) {
    let status = res.status();
    let mut text = res.text().unwrap_or("empty".into());
    if text.trim().is_empty() {
        text = "empty".into();
    }
    eprintln!("got HTTP status code {}, message: {}", status, text);
}
