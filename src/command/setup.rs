use crate::{settings, Result};
use clap::Args;

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
    settings::put_str("api_url", &url)?;
    Ok(())
}

#[derive(Args)]
pub struct LoginArgs {
    pub password: String,
}

pub fn login(args: &LoginArgs) -> Result<()> {
    settings::put_str("password", &args.password)?;
    Ok(())
}
