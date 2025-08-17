use crate::{rpc, Result};
use clap::Args;
use serde_json::{json, Map, Value};

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
    pub params: Option<String>,
}

pub fn rpc(args: &CustomArgs) -> Result<()> {
    let params: Value = match &args.params {
        Some(params) => serde_json::from_str(&params)?,
        None => Value::Object(Map::new()),
    };
    rpc::call(&args.method, params)?.print()
}
