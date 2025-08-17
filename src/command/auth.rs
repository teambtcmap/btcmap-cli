use crate::{rpc, settings, verbosity, Result};
use clap::Args;
use serde_json::json;

#[derive(Args)]
pub struct SignUpArgs {
    pub name: String,
    pub password: String,
}

pub fn sign_up(args: &SignUpArgs) -> Result<()> {
    let add_user_response = rpc::call(
        "add_user",
        json!({"name": args.name, "password": args.password}),
    )?;
    if add_user_response.error.is_some() {
        if verbosity() == 0 {
            eprintln!("Login failed, use verbose mode to see more details");
        } else {
            eprintln!("Login failed")
        }
        return Ok(());
    }
    let res = rpc::call(
        "create_api_key",
        json!({"username": args.name, "password": args.password, "label": "Created by btcmap-cli during signup"}),
    )?;
    let res = res.result.unwrap();
    let api_key = res["token"].as_str().unwrap();
    settings::put_str("password", api_key)?;
    println!("You are now logged in as {}", args.name);
    Ok(())
}
