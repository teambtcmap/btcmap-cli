use crate::{rpc, Result};
use clap::Args;
use serde_json::json;

#[derive(Args)]
pub struct SearchArgs {
    pub query: String,
}

pub fn search(args: &SearchArgs) -> Result<()> {
    rpc::call("search", json!({"query": args.query}))?.print()
}
