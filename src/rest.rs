use crate::rpc::RpcResponse;
use crate::settings;
use crate::verbosity;
use crate::Result;
use colored_json::ToColoredJson;
use serde_json::Value;

/// Percent-encode a value for use as a single URI path segment. Aliases are
/// stored verbatim and may contain spaces, uppercase or accented characters,
/// none of which are valid in a raw URI.
pub fn encode_path_segment(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                encoded.push(byte as char);
            }
            _ => encoded.push_str(&format!("%{byte:02X}")),
        }
    }
    encoded
}

/// Resolve the REST API root from the configured RPC endpoint. The `api_url`
/// setting points at the JSON-RPC endpoint (e.g. `https://api.btcmap.org/rpc`),
/// so a trailing `/rpc` is stripped to obtain the API root.
fn api_root() -> Result<String> {
    let mut api_url = settings::get_str("api_url")?;
    if api_url.trim().is_empty() {
        api_url = "https://api.btcmap.org/rpc".into();
    }
    let api_url = api_url.trim().trim_end_matches('/');
    let root = api_url.strip_suffix("/rpc").unwrap_or(api_url);
    Ok(root.trim_end_matches('/').to_string())
}

fn endpoint(path: &str) -> Result<String> {
    Ok(format!("{}/v4{}", api_root()?, path))
}

fn auth_header() -> Result<String> {
    Ok(format!("Bearer {}", settings::get_str("password")?))
}

fn print_request(method: &str, url: &str, body: Option<&Value>) -> Result<()> {
    if verbosity() > 0 {
        match body {
            Some(body) => {
                println!("{method} {url} with the following body:");
                println!("{}", serde_json::to_string(body)?.to_colored_json_auto()?);
            }
            None => println!("{method} {url}"),
        }
    }
    Ok(())
}

/// Map a REST exchange onto an [`RpcResponse`] so commands print REST and
/// JSON-RPC results the same way: a non-2xx status is surfaced in the `error`
/// field rather than as a transport error.
fn into_response(success: bool, status: u16, value: Value) -> Result<RpcResponse> {
    let response = if success {
        RpcResponse {
            result: Some(value),
            error: None,
        }
    } else {
        RpcResponse {
            result: None,
            error: Some(value),
        }
    };
    if verbosity() >= 3 {
        println!("REST response ({status}):");
        println!(
            "{}",
            serde_json::to_string(&response)?.to_colored_json_auto()?
        );
    }
    Ok(response)
}

/// GET a v4 REST endpoint. `path` is relative to `/v4` (example: `/areas/1`).
pub fn get(path: &str) -> Result<RpcResponse> {
    let url = endpoint(path)?;
    print_request("GET", &url, None)?;
    let response = ureq::get(&url)
        .config()
        .http_status_as_error(false)
        .build()
        .header("Authorization", auth_header()?)
        .call()?;
    let status = response.status();
    let value: Value = response.into_body().read_json()?;
    into_response(status.is_success(), status.as_u16(), value)
}

/// POST a v4 REST endpoint. `path` is relative to `/v4` (example: `/areas`).
pub fn post(path: &str, body: Value) -> Result<RpcResponse> {
    let url = endpoint(path)?;
    print_request("POST", &url, Some(&body))?;
    let response = ureq::post(&url)
        .config()
        .http_status_as_error(false)
        .build()
        .header("Content-Type", "application/json")
        .header("Authorization", auth_header()?)
        .send_json(&body)?;
    let status = response.status();
    let value: Value = response.into_body().read_json()?;
    into_response(status.is_success(), status.as_u16(), value)
}

/// PATCH a v4 REST endpoint. `path` is relative to `/v4` (example: `/areas/1`).
pub fn patch(path: &str, body: Value) -> Result<RpcResponse> {
    let url = endpoint(path)?;
    print_request("PATCH", &url, Some(&body))?;
    let response = ureq::patch(&url)
        .config()
        .http_status_as_error(false)
        .build()
        .header("Content-Type", "application/json")
        .header("Authorization", auth_header()?)
        .send_json(&body)?;
    let status = response.status();
    let value: Value = response.into_body().read_json()?;
    into_response(status.is_success(), status.as_u16(), value)
}
