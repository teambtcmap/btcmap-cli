use crate::{
    rpc::{self},
    Result,
};
use clap::Args;
use serde_json::{json, Map, Value};

#[derive(Args)]
pub struct CreateEventArgs {
    /// Numeric id of the community area to link the event to
    #[arg(long)]
    pub area_id: Option<i64>,
    #[clap(allow_hyphen_values = true)]
    #[arg(long)]
    pub lat: f64,
    #[clap(allow_hyphen_values = true)]
    #[arg(long)]
    pub lon: f64,
    /// Display name of the event
    #[arg(long)]
    pub name: String,
    /// URL with up-to-date event details
    #[arg(long)]
    pub website: String,
    /// Start time. Required. Use RFC 3339 with an offset
    /// (e.g. 2025-08-28T19:00:00+07:00), or a local time without an offset
    /// together with --timezone
    #[arg(long = "starts-at")]
    pub starts_at: String,
    /// End time, same formats as --starts-at
    #[arg(long = "ends-at")]
    pub ends_at: Option<String>,
    /// Timezone for timestamps given without an offset: "auto" infers it from
    /// lat/lon, or pass an IANA name like Europe/Berlin
    #[arg(long)]
    pub timezone: Option<String>,
}

pub fn create_event(args: &CreateEventArgs) -> Result<()> {
    let params = json!({
        "area_id": args.area_id,
        "lat": args.lat,
        "lon": args.lon,
        "name": args.name,
        "website": args.website,
        "starts_at": args.starts_at,
        "ends_at": args.ends_at,
        "timezone": args.timezone,
    });
    rpc::call("create_event", params)?.print()
}

#[derive(Args)]
pub struct GetEventsArgs {
    /// Include events whose starts_at is already in the past
    #[arg(long)]
    pub include_past: bool,
    /// Include soft-deleted events
    #[arg(long)]
    pub include_deleted: bool,
}

pub fn get_events(args: &GetEventsArgs) -> Result<()> {
    let mut params = Map::new();
    params.insert("include_past".into(), Value::Bool(args.include_past));
    params.insert("include_deleted".into(), Value::Bool(args.include_deleted));
    rpc::call("get_events", Value::Object(params))?.print()
}

#[derive(Args)]
pub struct GetEventArgs {
    /// Event id
    pub id: i64,
}

pub fn get_event(args: &GetEventArgs) -> Result<()> {
    rpc::call("get_event", json!({"id": args.id}))?.print()
}

#[derive(Args)]
pub struct UpdateEventArgs {
    /// Event id
    pub id: i64,
    /// Link the event to a community area
    #[arg(long)]
    pub area_id: Option<i64>,
    /// Unlink the event from its community area
    #[arg(long, conflicts_with = "area_id")]
    pub clear_area_id: bool,
    #[clap(allow_hyphen_values = true)]
    #[arg(long)]
    pub lat: Option<f64>,
    #[clap(allow_hyphen_values = true)]
    #[arg(long)]
    pub lon: Option<f64>,
    /// Display name of the event
    #[arg(long)]
    pub name: Option<String>,
    /// URL with up-to-date event details
    #[arg(long)]
    pub website: Option<String>,
    /// Start time. Use RFC 3339 with an offset, or a local time without an
    /// offset together with --timezone. Required field: cannot be cleared once set
    #[arg(long = "starts-at")]
    pub starts_at: Option<String>,
    /// End time, same formats as --starts-at
    #[arg(long = "ends-at")]
    pub ends_at: Option<String>,
    /// Clear the end time
    #[arg(long, conflicts_with = "ends_at")]
    pub clear_ends_at: bool,
    /// Timezone for timestamps given without an offset: "auto" infers it from
    /// lat/lon, or pass an IANA name like Europe/Berlin
    #[arg(long)]
    pub timezone: Option<String>,
}

fn put_nullable<T: Into<Value>>(
    params: &mut Map<String, Value>,
    key: &str,
    value: Option<T>,
    clear: bool,
) {
    match value {
        Some(value) => {
            params.insert(key.into(), value.into());
        }
        None if clear => {
            params.insert(key.into(), Value::Null);
        }
        None => {}
    }
}

pub fn update_event(args: &UpdateEventArgs) -> Result<()> {
    let mut params = Map::new();
    params.insert("id".into(), json!(args.id));
    put_nullable(&mut params, "area_id", args.area_id, args.clear_area_id);
    put_nullable(&mut params, "lat", args.lat, false);
    put_nullable(&mut params, "lon", args.lon, false);
    put_nullable(&mut params, "name", args.name.as_deref(), false);
    put_nullable(&mut params, "website", args.website.as_deref(), false);
    put_nullable(
        &mut params,
        "starts_at",
        args.starts_at.as_deref(),
        false,
    );
    put_nullable(
        &mut params,
        "ends_at",
        args.ends_at.as_deref(),
        args.clear_ends_at,
    );
    if let Some(timezone) = args.timezone.as_deref() {
        params.insert("timezone".into(), json!(timezone));
    }
    rpc::call("update_event", Value::Object(params))?.print()
}

#[derive(Args)]
pub struct DeleteEventArgs {
    /// Event id
    pub id: i64,
}

pub fn delete_event(args: &DeleteEventArgs) -> Result<()> {
    rpc::call("delete_event", json!({"id": args.id}))?.print()
}
