use crate::{rpc, settings, Result};
use clap::Args;
use serde_json::json;

#[derive(Args)]
pub struct SetApiKeyArgs {
    pub api_key: String,
}

pub fn set_api_key(args: &SetApiKeyArgs) -> Result<()> {
    settings::put_str("password", &args.api_key)?;
    println!("api key has been updated");
    Ok(())
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
pub struct CreateInvoiceArgs {
    pub amount_sats: i64,
    pub description: String,
}

pub fn create_invoice(args: &CreateInvoiceArgs) -> Result<()> {
    rpc::call(
        "create_invoice",
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