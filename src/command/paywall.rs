use crate::{rpc, Result};
use clap::Args;
use serde_json::json;

pub fn paywall_get_add_element_comment_quote() -> Result<()> {
    rpc::call("paywall_get_add_element_comment_quote", json!({}))?.print()
}

#[derive(Args)]
pub struct PaywallAddElementCommentArgs {
    pub element_id: String,
    pub comment: String,
}

pub fn paywall_add_element_comment(args: &PaywallAddElementCommentArgs) -> Result<()> {
    rpc::call(
        "paywall_add_element_comment",
        json!({"element_id": args.element_id,"comment": args.comment}),
    )?
    .print()
}

pub fn paywall_get_boost_element_quote() -> Result<()> {
    rpc::call("paywall_get_boost_element_quote", json!({}))?.print()
}

#[derive(Args)]
pub struct PaywallBoostElementArgs {
    pub element_id: String,
    pub days: i64,
}

pub fn paywall_boost_element(args: &PaywallBoostElementArgs) -> Result<()> {
    rpc::call(
        "paywall_boost_element",
        json!({"element_id": args.element_id, "days": args.days}),
    )?
    .print()
}
