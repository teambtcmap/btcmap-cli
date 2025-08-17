use crate::{rpc, settings, verbosity, Result};
use clap::Args;
use serde_json::json;

#[derive(Args)]
pub struct LoginArgs {
    pub username: String,
    pub password: String,
}

pub fn login(args: &LoginArgs) -> Result<()> {
    let response = rpc::call(
        "create_api_key",
        json!({
            "username": args.username,
            "password": args.password,
            "label": "Created by btcmap-cli"
        }),
    )?;
    if let Some(result) = response.result {
        let api_key = result["token"].as_str().unwrap();
        settings::put_str("password", api_key)?;
        println!("You are now logged in as {}", args.username);
    } else {
        if verbosity() == 0 {
            eprintln!("Login failed, use verbose mode to see more details");
        } else {
            eprintln!("Login failed")
        }
    }
    Ok(())
}

#[derive(Args)]
pub struct CreateApiKeyArgs {
    pub username: String,
    pub password: String,
    pub label: String,
}

pub fn create_api_key(args: &CreateApiKeyArgs) -> Result<()> {
    rpc::call(
        "create_api_key",
        json!({"username": args.username, "password": args.password, "label": args.label}),
    )?
    .print()
}

#[derive(Args)]
pub struct AddUserArgs {
    pub name: String,
    pub password: String,
}

pub fn add_user(args: &AddUserArgs) -> Result<()> {
    rpc::call(
        "add_user",
        json!({"name": args.name, "password": args.password}),
    )?
    .print()
}

#[derive(Args)]
pub struct ChangePasswordArgs {
    pub username: String,
    pub old_password: String,
    pub new_password: String,
}

pub fn change_password(args: &ChangePasswordArgs) -> Result<()> {
    rpc::call(
        "change_password",
        json!({"username": args.username, "old_password": args.old_password, "new_password": args.new_password}),
    )?
    .print()
}

#[derive(Args)]
pub struct AddAdminActionArgs {
    pub admin_name: String,
    pub action: String,
}

pub fn add_admin_action(args: &AddAdminActionArgs) -> Result<()> {
    rpc::call(
        "add_admin_action",
        json!({"admin": args.admin_name, "action": args.action}),
    )?
    .print()
}

#[derive(Args)]
pub struct RemoveAdminActionArgs {
    pub admin_name: String,
    pub action: String,
}

pub fn remove_admin_action(args: &RemoveAdminActionArgs) -> Result<()> {
    rpc::call(
        "remove_admin_action",
        json!({"admin": args.admin_name, "action": args.action}),
    )?
    .print()
}

#[derive(Args)]
pub struct GenerateInvoiceArgs {
    pub amount_sats: i64,
    pub description: String,
}

pub fn generate_invoice(args: &GenerateInvoiceArgs) -> Result<()> {
    rpc::call(
        "generate_invoice",
        json!({"amount_sats": args.amount_sats, "description": args.description}),
    )?
    .print()
}

#[derive(Args)]
pub struct SyncUnpaidInvoicesArgs {}

pub fn sync_unpaid_invoices(_args: &SyncUnpaidInvoicesArgs) -> Result<()> {
    rpc::call("sync_unpaid_invoices", json!({}))?.print()
}

#[derive(Args)]
pub struct GetInvoiceArgs {
    pub id: i64,
}

pub fn get_invoice(args: &GetInvoiceArgs) -> Result<()> {
    rpc::call("get_invoice", json!({"id": args.id }))?.print()
}
