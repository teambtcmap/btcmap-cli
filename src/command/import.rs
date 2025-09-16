use crate::{rpc, Result};
use clap::Args;
use serde_json::json;

#[derive(Args)]
pub struct SubmitPlaceArgs {
    #[arg(long)]
    pub origin: String,
    #[arg(long)]
    pub external_id: String,
    #[clap(allow_hyphen_values = true)]
    #[arg(long)]
    pub lat: f64,
    #[clap(allow_hyphen_values = true)]
    #[arg(long)]
    pub lon: f64,
    #[arg(long)]
    pub category: String,
    #[arg(long)]
    pub name: String,
    #[arg(long = "extra-fields")]
    pub extra_fields: Option<String>,
}

pub fn submit_place(args: &SubmitPlaceArgs) -> Result<()> {
    let params = json!({
        "origin": args.origin,
        "external_id": args.external_id,
        "lat": args.lat,
        "lon": args.lon,
        "category": args.category,
        "name": args.name,
        "extra_fields": args.extra_fields
    });
    rpc::call("submit_place", params)?.print()
}

#[derive(Args)]
pub struct GetSubmittedPlaceArgs {
    pub id: String,
}

pub fn get_submitted_place(args: &GetSubmittedPlaceArgs) -> Result<()> {
    match args.id.parse::<i64>() {
        Ok(id) => {
            let params = json!({ "id": id });
            rpc::call("get_submitted_place", params)?.print()
        }
        Err(_) => {
            let parts: Vec<&str> = args.id.split(":").collect();

            let params = json!({
                "origin": parts[0],
                "external_id": parts[1]
            });
            rpc::call("get_submitted_place", params)?.print()
        }
    }
}

#[derive(Args)]
pub struct RevokeSubmittedPlaceArgs {
    pub id: String,
}

pub fn revoke_submitted_place(args: &RevokeSubmittedPlaceArgs) -> Result<()> {
    match args.id.parse::<i64>() {
        Ok(id) => {
            let params = json!({ "id": id });
            rpc::call("revoke_submitted_place", params)?.print()
        }
        Err(_) => {
            let parts: Vec<&str> = args.id.split(":").collect();

            let params = json!({
                "origin": parts[0],
                "external_id": parts[1]
            });
            rpc::call("revoke_submitted_place", params)?.print()
        }
    }
}
