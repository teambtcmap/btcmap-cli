use crate::{rest, rpc, Result};
use clap::Args;
use serde_json::{json, Map, Value};

#[derive(Args)]
pub struct GetAreaArgs {
    pub id: String,
}

/// Fetch an area by numeric id or url alias through the v4 REST API.
pub fn get_area(args: &GetAreaArgs) -> Result<()> {
    rest::get(&format!("/areas/{}", rest::encode_path_segment(&args.id)))?.print()
}

#[derive(Args)]
pub struct AddAreaArgs {
    /// URL-friendly identifier for the area, used in btcmap.org links (must be
    /// unique)
    #[arg(long)]
    pub alias: String,
    /// Human readable area name. Defaults to the alias when omitted
    #[arg(long)]
    pub name: Option<String>,
    /// Area type, e.g. `community` or `country`
    #[arg(long = "type")]
    pub r#type: String,
    /// Area geometry as a GeoJSON Feature, geometry or FeatureCollection
    #[arg(long = "geojson")]
    pub geojson: String,
}

pub fn add_area(args: &AddAreaArgs) -> Result<()> {
    let geo_json: Value = serde_json::from_str(&args.geojson)
        .map_err(|e| format!("invalid --geojson: not a valid JSON value ({e})"))?;
    let name = args.name.clone().unwrap_or_else(|| args.alias.clone());
    rest::post(
        "/areas",
        json!({
            "name": name,
            "type": args.r#type,
            "url_alias": args.alias,
            "geo_json": geo_json,
        }),
    )?
    .print()
}

#[derive(Args)]
pub struct UpdateAreaArgs {
    /// Area id or url alias
    pub id: String,
    /// New area name
    #[arg(long)]
    pub name: Option<String>,
    /// New area type, e.g. `community` or `country`
    #[arg(long = "type")]
    pub r#type: Option<String>,
    /// New description
    #[arg(long, conflicts_with = "clear_description")]
    pub description: Option<String>,
    /// Remove the description
    #[arg(long)]
    pub clear_description: bool,
    /// New geometry as a GeoJSON Feature, geometry or FeatureCollection
    #[arg(long)]
    pub geojson: Option<String>,
    /// Set a contact channel, as CHANNEL=VALUE. Repeatable
    #[arg(long = "contact", value_name = "CHANNEL=VALUE")]
    pub contact: Vec<String>,
    /// Remove a contact channel. Repeatable
    #[arg(long = "clear-contact", value_name = "CHANNEL")]
    pub clear_contact: Vec<String>,
}

/// Partially update an area through the v4 REST API. Only the fields you pass
/// are changed; `--clear-description` and `--clear-contact` remove values.
pub fn update_area(args: &UpdateAreaArgs) -> Result<()> {
    let mut body = Map::new();

    if let Some(name) = &args.name {
        body.insert("name".into(), json!(name));
    }
    if let Some(r#type) = &args.r#type {
        body.insert("type".into(), json!(r#type));
    }
    if let Some(description) = &args.description {
        body.insert("description".into(), json!(description));
    }
    if args.clear_description {
        body.insert("description".into(), Value::Null);
    }
    if let Some(geojson) = &args.geojson {
        let geo_json: Value = serde_json::from_str(geojson)
            .map_err(|e| format!("invalid --geojson: not a valid JSON value ({e})"))?;
        body.insert("geo_json".into(), geo_json);
    }

    let mut contact = Map::new();
    for entry in &args.contact {
        let (channel, value) = entry
            .split_once('=')
            .ok_or_else(|| format!("invalid --contact '{entry}': expected CHANNEL=VALUE"))?;
        contact.insert(channel.to_string(), json!(value));
    }
    for channel in &args.clear_contact {
        contact.insert(channel.to_string(), Value::Null);
    }
    if !contact.is_empty() {
        body.insert("contact".into(), Value::Object(contact));
    }

    if body.is_empty() {
        return Err("nothing to update: pass at least one field".into());
    }

    rest::patch(
        &format!("/areas/{}", rest::encode_path_segment(&args.id)),
        Value::Object(body),
    )?
    .print()
}

#[derive(Args)]
pub struct RemoveAreaTagArgs {
    pub id: String,
    pub tag: String,
}

pub fn remove_area_tag(args: &RemoveAreaTagArgs) -> Result<()> {
    rpc::call("remove_area_tag", json!({"id": args.id,"tag": args.tag}))?.print()
}

#[derive(Args)]
pub struct SetAreaImageArgs {
    #[arg(long)]
    pub area: String,
    #[arg(long, default_value = "square")]
    pub r#type: String,
    #[arg(long)]
    pub image: String,
}

pub fn set_area_image(args: &SetAreaImageArgs) -> Result<()> {
    let bytes = std::fs::read(&args.image)?;
    let image_base64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bytes);
    rpc::call(
        "set_area_image",
        json!({
            "area_id": args.area,
            "image_base64": image_base64,
            "image_type": args.r#type,
        }),
    )?
    .print()
}

pub fn generate_element_mapping() -> Result<()> {
    rpc::call("generate_areas_elements_mapping", json!({}))?.print()
}

pub fn generate_bboxes() -> Result<()> {
    rpc::call("generate_area_bboxes", json!({}))?.print()
}

pub fn generate_area_icons() -> Result<()> {
    rpc::call("generate_area_icons", json!({}))?.print()
}
