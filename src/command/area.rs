use crate::{rpc, Result};
use clap::Args;
use serde_json::{json, Value};

#[derive(Args)]
pub struct GetAreaArgs {
    pub id: String,
}

pub fn get_area(args: &GetAreaArgs) -> Result<()> {
    rpc::call("get_area", json!({"id": args.id}))
}

#[derive(Args)]
pub struct SetAreaTagArgs {
    pub id: String,
    pub name: String,
    pub value: String,
}

pub fn set_area_tag(args: &SetAreaTagArgs) -> Result<()> {
    let value: Value = serde_json::from_str(&args.value)?;
    rpc::call(
        "set_area_tag",
        json!({"id": args.id,"name": args.name, "value": value}),
    )
}

#[derive(Args)]
pub struct RemoveAreaTagArgs {
    pub id: String,
    pub tag: String,
}

pub fn remove_area_tag(args: &RemoveAreaTagArgs) -> Result<()> {
    rpc::call("remove_area_tag", json!({"id": args.id,"tag": args.tag}))
}

#[derive(Args)]
pub struct SetAreaIconArgs {
    pub id: String,
    pub icon_base64: String,
    pub icon_ext: String,
}

pub fn set_area_icon(args: &SetAreaIconArgs) -> Result<()> {
    rpc::call(
        "set_area_icon",
        json!({"id": args.id,"icon_base64": args.icon_base64,"icon_ext": args.icon_ext}),
    )
}

#[derive(Args)]
pub struct GenerateAreasElementsMappingArgs {
    pub from_element_id: i64,
    pub to_element_id: i64,
}

pub fn generate_areas_elements_mapping(args: &GenerateAreasElementsMappingArgs) -> Result<()> {
    rpc::call(
        "generate_areas_elements_mapping",
        json!({"from_element_id": args.from_element_id,"to_element_id": args.to_element_id}),
    )
}
