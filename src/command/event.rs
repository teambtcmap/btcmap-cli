use crate::{
    rpc::{self},
    Result,
};
use clap::Args;
use serde_json::{json, Map, Value};

#[derive(Args)]
pub struct CreateEventArgs {
    #[clap(allow_hyphen_values = true)]
    pub lat: f64,
    #[clap(allow_hyphen_values = true)]
    pub lon: f64,
    pub name: String,
    pub website: String,
    pub starts_at: String,
    pub ends_at: Option<String>,
}

pub fn create_event(args: &CreateEventArgs) -> Result<()> {
    let params = json!({
        "lat": args.lat,
        "lon": args.lon,
        "name": args.name,
        "website": args.website,
        "starts_at": args.starts_at,
        "ends_at": args.ends_at
    });
    rpc::call("create_event", params)?.print()
}

pub fn get_events() -> Result<()> {
    rpc::call("get_events", Value::Object(Map::new()))?.print()
}

#[derive(Args)]
pub struct GetEventArgs {
    pub id: i64,
}

pub fn get_event(args: &GetEventArgs) -> Result<()> {
    rpc::call("get_event", json!({"id": args.id}))?.print()
}

#[derive(Args)]
pub struct DeleteEventArgs {
    pub id: i64,
}

pub fn delete_event(args: &DeleteEventArgs) -> Result<()> {
    rpc::call("delete_event", json!({"id": args.id}))?.print()
}
