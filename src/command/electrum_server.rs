use crate::{rpc, Result};
use clap::Args;
use serde_json::json;

pub fn list(args: &ListArgs) -> Result<()> {
    let params = json!({ "include_deleted": args.include_deleted });
    rpc::call("get_electrum_servers", params)?.print()
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
    pub url: String,
    #[arg(long, default_value_t = 0)]
    pub priority: i64,
    #[arg(long)]
    pub spki_pin: Option<String>,
}

pub fn add(args: &AddArgs) -> Result<()> {
    let params = json!({
        "name": args.name,
        "url": args.url,
        "priority": args.priority,
        "spki_pin": args.spki_pin,
    });
    rpc::call("add_electrum_server", params)?.print()
}

#[derive(Args)]
pub struct UpdateArgs {
    pub id: i64,
    #[arg(long)]
    pub name: Option<String>,
    #[arg(long)]
    pub url: Option<String>,
    #[arg(long)]
    pub priority: Option<i64>,
    /// Pass an empty string to clear the pin.
    #[arg(long)]
    pub spki_pin: Option<String>,
}

pub fn update(args: &UpdateArgs) -> Result<()> {
    let mut params = json!({ "id": args.id });
    if let Some(name) = &args.name {
        params["name"] = json!(name);
    }
    if let Some(url) = &args.url {
        params["url"] = json!(url);
    }
    if let Some(priority) = args.priority {
        params["priority"] = json!(priority);
    }
    if let Some(spki_pin) = &args.spki_pin {
        params["spki_pin"] = json!(spki_pin);
    }
    rpc::call("update_electrum_server", params)?.print()
}

#[derive(Args)]
pub struct RemoveArgs {
    pub id: i64,
}

pub fn remove(args: &RemoveArgs) -> Result<()> {
    let params = json!({ "id": args.id });
    rpc::call("remove_electrum_server", params)?.print()
}
