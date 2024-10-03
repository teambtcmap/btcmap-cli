use crate::db;
use colored_json::ToColoredJson;
use reqwest::blocking::ClientBuilder;
use reqwest::blocking::Response;
use serde_json::{json, Map, Value};

pub fn call_remote_procedure(name: &str, mut params: Value) {
    let params = params.as_object_mut().unwrap();
    params.insert(
        "password".into(),
        Value::String(db::query_settings_string("password", &db::connect())),
    );
    let client = ClientBuilder::new().timeout(None).build().unwrap();
    let args = json!(
        {"jsonrpc": "2.0", "method": name, "params": params, "id": 1}
    );
    let mut api_url = db::query_settings_string("api_url", &db::connect());
    if api_url.trim().is_empty() {
        api_url = "https://api.btcmap.org/rpc".into();
    }
    let res = client
        .post(api_url)
        .body(serde_json::to_string(&args).unwrap())
        .send();
    match res {
        Ok(res) => {
            if res.status().is_success() {
                let res = res.json::<Map<String, Value>>().unwrap();
                let res = serde_json::to_string_pretty(&res).unwrap();
                println!("{}", res.to_colored_json_auto().unwrap());
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
    eprintln!("HTTP status code: {}, message: {}", status, text);
}
