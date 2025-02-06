use crate::{rpc, Result};
use clap::Args;
use serde_json::{json, Value};

#[derive(Args)]
pub struct GetElementArgs {
    pub id: i64,
}

pub fn get_element(args: &GetElementArgs) -> Result<()> {
    rpc::call("get_element", json!({"id": args.id}))?.print()
}

#[derive(Args)]
pub struct SetElementTagArgs {
    pub element_id: i64,
    pub tag_name: String,
    pub tag_value: String,
}

pub fn set_element_tag(args: &SetElementTagArgs) -> Result<()> {
    let value: Value = serde_json::from_str(&args.tag_value)?;
    rpc::call(
        "set_element_tag",
        json!({"element_id": args.element_id, "tag_name": args.tag_name, "tag_value": value}),
    )?
    .print()
}

#[derive(Args)]
pub struct RemoveElementTagArgs {
    pub element_id: i64,
    pub tag_name: String,
}

pub fn remove_element_tag(args: &RemoveElementTagArgs) -> Result<()> {
    rpc::call(
        "remove_element_tag",
        json!({"element_id": args.element_id, "tag_name": args.tag_name}),
    )?
    .print()
}

pub fn get_boosted_elements() -> Result<()> {
    rpc::call("get_boosted_elements", json!({}))?.print()
}

#[derive(Args)]
pub struct BoostElementArgs {
    pub id: String,
    pub days: i64,
}

pub fn boost_element(args: &BoostElementArgs) -> Result<()> {
    rpc::call("boost_element", json!({"id": args.id, "days": args.days}))?.print()
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

#[derive(Args)]
pub struct AddElementCommentArgs {
    pub element_id: i64,
    pub comment: String,
}

pub fn add_element_comment(args: &AddElementCommentArgs) -> Result<()> {
    rpc::call(
        "add_element_comment",
        json!({"element_id": args.element_id, "comment": args.comment}),
    )?
    .print()
}

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

#[derive(Args)]
pub struct GenerateElementIssuesArgs {}

pub fn generate_element_issues(_: &GenerateElementIssuesArgs) -> Result<()> {
    rpc::call("generate_element_issues", json!({}))?.print()
}

#[derive(Args)]
pub struct SyncElementsArgs {}

pub fn sync_elements(_: &SyncElementsArgs) -> Result<()> {
    rpc::call("sync_elements", json!({}))?.print()
}

#[derive(Args)]
pub struct GenerateElementIconsArgs {
    pub from_element_id: i64,
    pub to_element_id: i64,
}

pub fn generate_element_icons(args: &GenerateElementIconsArgs) -> Result<()> {
    rpc::call(
        "generate_element_icons",
        json!({"from_element_id": args.from_element_id,"to_element_id": args.to_element_id}),
    )?
    .print()
}

#[derive(Args)]
pub struct GenerateElementCategoriesArgs {
    pub from_element_id: i64,
    pub to_element_id: i64,
}

pub fn generate_element_categories(args: &GenerateElementCategoriesArgs) -> Result<()> {
    rpc::call(
        "generate_element_categories",
        json!({"from_element_id": args.from_element_id,"to_element_id": args.to_element_id}),
    )?
    .print()
}
