use crate::{rpc, Result};
use clap::Args;
use serde_json::json;

#[derive(Args)]
pub struct AddAdminArgs {
    pub new_admin_name: String,
    pub new_admin_password: String,
}

pub fn add_admin(args: &AddAdminArgs) -> Result<()> {
    rpc::call(
        "add_admin",
        json!({"new_admin_name": args.new_admin_name, "new_admin_password": args.new_admin_password}),
    )?.print()
}

#[derive(Args)]
pub struct AddAllowedActionArgs {
    pub admin_name: String,
    pub action: String,
}

pub fn add_allowed_action(args: &AddAllowedActionArgs) -> Result<()> {
    rpc::call(
        "add_allowed_action",
        json!({"admin_name": args.admin_name, "action": args.action}),
    )?
    .print()
}

#[derive(Args)]
pub struct RemoveAllowedActionArgs {
    pub admin_name: String,
    pub action: String,
}

pub fn remove_allowed_action(args: &RemoveAllowedActionArgs) -> Result<()> {
    rpc::call(
        "remove_allowed_action",
        json!({"admin_name": args.admin_name, "action": args.action}),
    )?
    .print()
}
