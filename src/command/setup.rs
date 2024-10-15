use crate::{rpc, settings, verbosity, Result};
use clap::Args;
use colored_json::ToColoredJson;
use serde_json::json;

#[derive(Args)]
pub struct SetServerArgs {
    pub url: String,
}

pub fn set_server(args: &SetServerArgs) -> Result<()> {
    let url = match args.url.as_str() {
        "prod" => "https://api.btcmap.org/rpc",
        "dev" => "http://127.0.0.1:8000/rpc",
        _ => &args.url,
    };
    if verbosity() > 0 {
        println!("Old value: {}", settings::get_str("api_url")?);
        println!("New value: {}", url);
    }
    settings::put_str("api_url", &url)?;
    Ok(())
}

#[derive(Args)]
pub struct LoginArgs {
    pub password: String,
}

pub fn login(args: &LoginArgs) -> Result<()> {
    let old_password = settings::get_str("password")?;
    settings::put_str("password", &args.password)?;
    match rpc::call("get_area", json!({ "id": "th" })) {
        Ok(res) => match res.error {
            Some(_) => {
                res.print()?;
                settings::put_str("password", &old_password)?;
                Err("Password is likely wrong")?
            }
            None => Ok(()),
        },
        Err(e) => {
            settings::put_str("password", &old_password)?;
            Err(e)?
        }
    }
}

#[derive(Args)]
pub struct StateArgs {}

pub fn state(_: &StateArgs) -> Result<()> {
    let state = json!({ "server": settings::get_str("api_url")?, "password": settings::get_str("password")? });
    println!("{}", serde_json::to_string(&state)?.to_colored_json_auto()?);
    Ok(())
}
