use crate::{rpc, Result};
use clap::Args;
use serde_json::json;

#[derive(Args)]
pub struct GetAreaDashboardArgs {
    pub area_id: i64,
}

pub fn get_dashboard() -> Result<()> {
    rpc::call("dashboard", json!({}))?.print()
}

pub fn get_area_dashboard(args: &GetAreaDashboardArgs) -> Result<()> {
    rpc::call("get_area_dashboard", json!({"area_id": args.area_id}))?.print()
}
