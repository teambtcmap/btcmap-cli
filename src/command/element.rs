use crate::{rpc, Result};
use clap::Args;
use serde_json::{json, Value};

#[derive(Args)]
pub struct GetElementArgs {
    pub id: String,
}

pub fn get_element(args: &GetElementArgs) -> Result<()> {
    rpc::call("get_element", json!({"id": args.id}))
}

#[derive(Args)]
pub struct SetElementTagArgs {
    pub id: String,
    pub name: String,
    pub value: String,
}

pub fn set_element_tag(args: &SetElementTagArgs) -> Result<()> {
    let value: Value = serde_json::from_str(&args.value)?;
    rpc::call(
        "set_element_tag",
        json!({"id": args.id,"name": args.name, "value": value}),
    )
}

#[derive(Args)]
pub struct RemoveElementTagArgs {
    pub id: String,
    pub tag: String,
}

pub fn remove_element_tag(args: &RemoveElementTagArgs) -> Result<()> {
    rpc::call("remove_element_tag", json!({"id": args.id,"tag": args.tag}))
}

#[derive(Args)]
pub struct AddElementCommentArgs {
    pub id: String,
    pub comment: String,
}

pub fn add_element_comment(args: &AddElementCommentArgs) -> Result<()> {
    rpc::call(
        "add_element_comment",
        json!({"id": args.id,"comment": args.comment}),
    )
}

#[derive(Args)]
pub struct BoostElementArgs {
    pub id: String,
    pub days: i64,
}

pub fn boost_element(args: &BoostElementArgs) -> Result<()> {
    rpc::call("boost_element", json!({"id": args.id,"days": args.days}))
}

#[derive(Args)]
pub struct GetBoostsArgs {}

pub fn get_boosts(_: &GetBoostsArgs) -> Result<()> {
    rpc::call("get_boosts", json!({}))
}

#[derive(Args)]
pub struct SyncElementsArgs {}

pub fn sync_elements(_: &SyncElementsArgs) -> Result<()> {
    rpc::call("sync_elements", json!({}))
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
    )
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
    )
}

#[derive(Args)]
pub struct GenerateElementIssuesArgs {}

pub fn generate_element_issues(_: &GenerateElementIssuesArgs) -> Result<()> {
    rpc::call("generate_element_issues", json!({}))
}
