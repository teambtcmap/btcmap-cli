use crate::settings;
use crate::verbosity;
use crate::Result;
use colored_json::ToColoredJson;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
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

pub fn call(method: &str, mut params: Value) -> Result<RpcResponse> {
    if verbosity() > 0 {
        println!(
            "{}",
            serde_json::to_string(&params)?.to_colored_json_auto()?
        );
    }
    let params = params
        .as_object_mut()
        .ok_or("params value is not a valid JSON object")?;
    params.insert("password".into(), settings::get_str("password")?.into());
    let args = json!(
        {"jsonrpc": "2.0", "method": method, "params": params, "id": 1}
    );
    let mut api_url = settings::get_str("api_url")?;
    if api_url.trim().is_empty() {
        api_url = "https://api.btcmap.org/rpc".into();
    }
    if verbosity() >= 2 {
        println!("{}", serde_json::to_string(&args)?.to_colored_json_auto()?);
    }
    let response = ureq::post(api_url)
        .send_json(args)?
        .body_mut()
        .read_to_string()?;
    if verbosity() >= 2 {
        println!("{}", response);
    }
    Ok(serde_json::from_str(&response)?)
}
