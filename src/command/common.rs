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

#[derive(Args)]
pub struct CustomArgs {
    pub method: String,
    pub params: String,
}

pub fn rpc(args: &CustomArgs) -> Result<()> {
    rpc::call(&args.method, serde_json::from_str(&args.params)?)?.print()
}
