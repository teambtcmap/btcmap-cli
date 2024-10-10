use crate::{rpc, Result};
use clap::Args;
use serde_json::json;

#[derive(Args)]
pub struct GetUserActivityArgs {
    pub id: String,
    pub limit: i64,
}

pub fn get_user_activity(args: &GetUserActivityArgs) -> Result<()> {
    rpc::call(
        "get_user_activity",
        json!({"id": args.id, "limit": args.limit}),
    )
}
