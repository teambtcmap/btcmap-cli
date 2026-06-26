use crate::{rpc, settings, verbosity, Result};
use clap::Args;
use serde_json::json;

#[derive(Args)]
pub struct SignUpArgs {
    #[arg(long)]
    pub user: String,
    #[arg(long)]
    pub password: String,
    #[arg(long)]
    pub label: Option<String>,
}

pub fn sign_up(args: &SignUpArgs) -> Result<()> {
    let mut params = json!({
        "username": args.user,
        "password": args.password,
    });
    if let Some(label) = &args.label {
        params["label"] = json!(label);
    }
    let response = rpc::call_anon("signup", params)?;
    if let Some(result) = response.result {
        let api_key = result["api_key"].as_str().unwrap_or_default().to_string();
        settings::put_str("password", &api_key)?;
        println!("You are now logged in as {}", args.user);
    } else if verbosity() == 0 {
        eprintln!("Signup failed, use verbose mode to see more details");
    } else {
        eprintln!("Signup failed")
    }
    Ok(())
}

#[derive(Args)]
pub struct SignInArgs {
    pub username: String,
    pub password: String,
    #[arg(long)]
    pub label: Option<String>,
    #[arg(long = "role", value_name = "ROLE", action = clap::ArgAction::Append)]
    pub roles: Vec<String>,
}

pub fn sign_in(args: &SignInArgs) -> Result<()> {
    let label = args
        .label
        .clone()
        .unwrap_or_else(|| format!("btcmap-cli ({})", hostname()));
    let mut params = json!({
        "username": args.username,
        "password": args.password,
        "label": label,
    });
    if !args.roles.is_empty() {
        params["roles"] = json!(args.roles);
    }
    let response = rpc::call_anon("signin", params)?;
    if let Some(result) = response.result {
        let api_key = result["api_key"].as_str().unwrap().to_string();
        settings::put_str("password", &api_key)?;
        println!("You are now logged in as {}", args.username);
        if !args.roles.is_empty() {
            println!("Token scope: {}", args.roles.join(", "));
        }
    } else if verbosity() == 0 {
        eprintln!("Signin failed, use verbose mode to see more details");
    } else {
        eprintln!("Signin failed")
    }
    Ok(())
}

#[derive(Args)]
pub struct ChangePasswordArgs {
    #[arg(long)]
    pub user: String,
    #[arg(long)]
    pub old: String,
    #[arg(long)]
    pub new: String,
}

pub fn change_password(args: &ChangePasswordArgs) -> Result<()> {
    rpc::call(
        "change_password",
        json!({
            "username": args.user,
            "old_password": args.old,
            "new_password": args.new,
        }),
    )?
    .print()
}

#[derive(Args)]
pub struct GetApiKeysArgs {}

pub fn get_api_keys(_args: &GetApiKeysArgs) -> Result<()> {
    rpc::call("get_api_keys", json!({}))?.print()
}

#[derive(Args)]
pub struct RevokeApiKeyArgs {
    pub id: i64,
}

pub fn revoke_api_key(args: &RevokeApiKeyArgs) -> Result<()> {
    rpc::call("revoke_api_key", json!({"id": args.id}))?.print()
}

#[derive(Args)]
pub struct SignOutArgs {}

pub fn sign_out(_args: &SignOutArgs) -> Result<()> {
    let response = rpc::call("signout", json!({}))?;
    response.print()?;
    settings::put_str("password", "")?;
    println!("Signed out");
    Ok(())
}

#[derive(Args)]
pub struct WhoAmIArgs {}

pub fn whoami(_args: &WhoAmIArgs) -> Result<()> {
    rpc::call("whoami", json!({}))?.print()
}

fn hostname() -> String {
    #[cfg(unix)]
    {
        if let Ok(buf) = std::fs::read_to_string("/proc/sys/kernel/hostname") {
            let trimmed = buf.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }
    }
    std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("COMPUTERNAME"))
        .unwrap_or_else(|_| "unknown".to_string())
}
