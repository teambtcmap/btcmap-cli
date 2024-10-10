use crate::{rpc, Result};
use clap::Args;
use serde_json::json;

#[derive(Args)]
pub struct GenerateReportsArgs {}

pub fn generate_reports(_: &GenerateReportsArgs) -> Result<()> {
    rpc::call("generate_reports", json!({}))
}

#[derive(Args)]
pub struct GetTrendingCountriesArgs {
    pub period_start: String,
    pub period_end: String,
}

pub fn get_trending_countries(args: &GetTrendingCountriesArgs) -> Result<()> {
    rpc::call(
        "get_trending_countries",
        json!({"period_start": args.period_start, "period_end": args.period_end}),
    )
}

#[derive(Args)]
pub struct GetTrendingCommunitiesArgs {
    pub period_start: String,
    pub period_end: String,
}

pub fn get_trending_communities(args: &GetTrendingCommunitiesArgs) -> Result<()> {
    rpc::call(
        "get_trending_communities",
        json!({"period_start": args.period_start, "period_end": args.period_end}),
    )
}

#[derive(Args)]
pub struct GetMostCommentedCountriesArgs {
    pub period_start: String,
    pub period_end: String,
}

pub fn get_most_commented_countries(args: &GetMostCommentedCountriesArgs) -> Result<()> {
    rpc::call(
        "get_most_commented_countries",
        json!({"period_start": args.period_start, "period_end": args.period_end}),
    )
}
