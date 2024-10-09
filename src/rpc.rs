use crate::settings;
use crate::Result;
use colored_json::ToColoredJson;
use serde_json::{json, Value};

pub fn call_remote_procedure(name: &str, mut params: Value) -> Result<()> {
    let params = params
        .as_object_mut()
        .ok_or("params value is not a valid JSON object")?;
    params.insert("password".into(), settings::get_str("password")?.into());
    let args = json!(
        {"jsonrpc": "2.0", "method": name, "params": params, "id": 1}
    );
    let mut api_url = settings::get_str("api_url")?;
    if api_url.trim().is_empty() {
        api_url = "https://api.btcmap.org/rpc".into();
    }
    let res = ureq::post(api_url)
        .send_json(args)?
        .body_mut()
        .read_to_string()?;
    println!("{}", res.to_colored_json_auto()?);
    Ok(())
}
