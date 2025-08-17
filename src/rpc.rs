use crate::settings;
use crate::verbosity;
use crate::Result;
use colored_json::ToColoredJson;
use serde::Deserialize;
use serde::Serialize;
use serde_json::{json, Value};
use std::i64;

#[derive(Serialize, Deserialize)]
pub struct RpcResponse {
    pub result: Option<Value>,
    pub error: Option<Value>,
}

impl RpcResponse {
    pub fn print(&self) -> Result<()> {
        if let Some(result) = &self.result {
            println!(
                "{}",
                serde_json::to_string(&result)?.to_colored_json_auto()?
            );
        } else {
            if let Some(error) = &self.error {
                println!("{}", serde_json::to_string(&error)?.to_colored_json_auto()?)
            }
        }
        Ok(())
    }
}

pub fn call(method: &str, params: Value) -> Result<RpcResponse> {
    match verbosity() {
        i64::MIN..=0 => {}
        1..=i64::MAX => {
            println!("Calling method {method} with the following params:");
            println!(
                "{}",
                serde_json::to_string(&params)?.to_colored_json_auto()?
            );
        }
    }
    let params = params
        .as_object()
        .ok_or("params value is not a valid JSON object")?;
    let req_body = json!(
        {"jsonrpc": "2.0", "method": method, "params": params, "id": 1}
    );
    let mut api_url = settings::get_str("api_url")?;
    if api_url.trim().is_empty() {
        api_url = "https://api.btcmap.org/rpc".into();
    }
    if verbosity() >= 2 {
        println!("Full request body:");
        println!(
            "{}",
            serde_json::to_string(&req_body)?.to_colored_json_auto()?
        );
    }
    let response: RpcResponse = ureq::post(api_url)
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            format!("Bearer {}", settings::get_str("password")?),
        )
        .send_json(req_body)?
        .body_mut()
        .read_json()?;
    match verbosity() {
        i64::MIN..=0 => {}
        1..=2 => {
            if response.result.is_some() {
                println!("RPC result:");
                println!(
                    "{}",
                    serde_json::to_string(&response.result)?.to_colored_json_auto()?
                );
            }
            if response.error.is_some() {
                println!("RPC error:");
                println!(
                    "{}",
                    serde_json::to_string(&response.error)?.to_colored_json_auto()?
                );
            }
        }
        3..=i64::MAX => {
            println!("RPC response:");
            println!(
                "{}",
                serde_json::to_string(&response)?.to_colored_json_auto()?
            );
        }
    };
    Ok(response)
}
