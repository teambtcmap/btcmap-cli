use crate::{rpc, Result};
use clap::Args;
use serde_json::json;

pub fn list(args: &ListArgs) -> Result<()> {
    let params = json!({ "include_deleted": args.include_deleted });
    rpc::call("get_wallets", params)?.print()
}

#[derive(Args)]
pub struct ListArgs {
    #[arg(long)]
    pub include_deleted: bool,
}

#[derive(Args)]
pub struct AddArgs {
    #[arg(long)]
    pub name: String,
    #[arg(long)]
    pub xpub: String,
}

pub fn add(args: &AddArgs) -> Result<()> {
    let params = json!({
        "name": args.name,
        "xpub": args.xpub,
    });
    rpc::call("add_wallet", params)?.print()
}

#[derive(Args)]
pub struct UpdateArgs {
    pub id: i64,
    #[arg(long)]
    pub name: Option<String>,
    #[arg(long)]
    pub xpub: Option<String>,
    /// Soft-delete the wallet by passing an RFC 3339 timestamp; pass the empty string to clear the soft-delete.
    #[arg(long)]
    pub deleted_at: Option<String>,
}

pub fn update(args: &UpdateArgs) -> Result<()> {
    let mut params = json!({ "id": args.id });
    if let Some(name) = &args.name {
        params["name"] = json!(name);
    }
    if let Some(xpub) = &args.xpub {
        params["xpub"] = json!(xpub);
    }
    if let Some(deleted_at) = &args.deleted_at {
        if deleted_at.is_empty() {
            params["deleted_at"] = json!(null);
        } else {
            params["deleted_at"] = json!(deleted_at);
        }
    }
    rpc::call("update_wallet", params)?.print()
}

#[derive(Args)]
pub struct RemoveArgs {
    pub id: i64,
}

pub fn remove(args: &RemoveArgs) -> Result<()> {
    let params = json!({ "id": args.id });
    rpc::call("remove_wallet", params)?.print()
}
