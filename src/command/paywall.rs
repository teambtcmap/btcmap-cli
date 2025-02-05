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
