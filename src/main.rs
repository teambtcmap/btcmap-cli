use std::{env, error::Error};
mod rpc;
mod settings;
use clap::{Parser, Subcommand};
use command::{
    area,
    element::{
        self, GenerateElementCategoriesArgs, GenerateElementIconsArgs, GenerateElementIssuesArgs,
        GetElementsSnapshotArgs,
    },
};
mod command;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[derive(Subcommand)]
enum Commands {
    /// Set a JSON RPC API URL
    SetServer(command::setup::SetServerArgs),
    /// Try to login and save password for future calls, if successful
    Login(command::setup::LoginArgs),
    /// Show all locally cached data
    State(command::setup::StateArgs),
    /// Create a new admin user. New admins have no permissions by default, use add-admin-action to allow certain acitons
    AddAdmin(command::admin::AddAdminArgs),
    /// Allow other admin to perform a certain action. You must be super admin to use this command
    AddAdminAction(command::admin::AddAdminActionArgs),
    /// Block other admin from using a certain action. You must be super admin to use this command
    RemoveAdminAction(command::admin::RemoveAdminActionArgs),
    /// Generate invoice
    GenerateInvoice(command::admin::GenerateInvoiceArgs),
    /// Sync unpaid invoices
    SyncUnpaidInvoices(command::admin::SyncUnpaidInvoicesArgs),
    /// Return all entities matching provided search query. Currently, only areas are returned
    Search(command::common::SearchArgs),
    /// Fetch element by a numeric or OSM (node:12345) id. You can also use node=12345 format
    GetElement(command::element::GetElementArgs),
    /// Set tag to a certain element. You can use either numeric or OSM (node:12345) id. Every tag must be a valid JSON value. Nulls are not allowed and will be interpreted as deletion requests
    SetElementTag(command::element::SetElementTagArgs),
    /// Remove tag from a certain element. You can use either numeric or OSM (node:12345) id
    RemoveElementTag(command::element::RemoveElementTagArgs),
    /// Add coment to a certain element. You can use either numeric or OSM (node:12345) id
    AddElementComment(command::element::AddElementCommentArgs),
    /// Boost an element for a set number of days. You can use either numeric or OSM (node:12345) id
    BoostElement(command::element::BoostElementArgs),
    /// Get all boosted elements
    GetBoostedElements(command::element::GetBoostedElementsArgs),
    /// Fetch the latest Overpass snapshot and merge it with cached elements. It may take a long time and it's not supposed to be called manually
    SyncElements(command::element::SyncElementsArgs),
    /// Generate icon:android tags for a specific element id range
    GenerateElementIcons(GenerateElementIconsArgs),
    /// Generate category tags for a specific element id range
    GenerateElementCategories(GenerateElementCategoriesArgs),
    /// Generate issues tags for a specific element id range. This command is supposed to be called automatically by a BTC Map server internal shceduler
    GenerateElementIssues(GenerateElementIssuesArgs),
    /// Get snapshot of all visible elements. This command is supposed to be called automatically by a BTC Map server internal shceduler
    GetElementsSnapshot(GetElementsSnapshotArgs),
    /// Fetch area by either numeric id or string alias (th)
    GetArea(command::area::GetAreaArgs),
    /// Set tag to a certain area. You can use either numeric id or a string alias (th)
    SetAreaTag(command::area::SetAreaTagArgs),
    /// Remove tag from a certain area. You can use either numeric id or a string alias (th)
    RemoveAreaTag(command::area::RemoveAreaTagArgs),
    /// Set icon to a certain area. You can use either numeric id or a string alias (th). Icon needs to be base64-encoded, and you also need to provide file extension
    SetAreaIcon(command::area::SetAreaIconArgs),
    /// Ensure that elements and areas are correctly mapped to each other. You need to provide element id range in order to operate on a specific slice of elements
    GenerateAreasElementsMapping(command::area::GenerateAreasElementsMappingArgs),
    /// Fetch the latest user actions. You need to provide OSM username and the number of latest entries you are interested in
    GetUserActivity(command::user::GetUserActivityArgs),
    /// Generate daily reports. It will skip report generation if current date is already covered
    GenerateReports(command::report::GenerateReportsArgs),
    /// Find which countries were trending during a certain time period. Arguments should be valid ISO dates (example: 2024-09-10)
    GetTrendingCountries(command::report::GetTrendingCountriesArgs),
    /// Find which communities were trending during a certain time period. Arguments should be valid ISO dates (example: 2024-09-10)
    GetTrendingCommunities(command::report::GetTrendingCommunitiesArgs),
    /// Find which countries had the most comments during a certain time period. Arguemnts should be valid ISO dates (example: 2024-09-10)
    GetMostCommentedCountries(command::report::GetMostCommentedCountriesArgs),
    /// Get current element comment price in sats
    PaywallGetAddElementCommentQuote,
    /// Submit comment to receive an invoice
    PaywallAddElementComment(command::paywall::PaywallAddElementCommentArgs),
    /// Get current element boost price in sats
    PaywallGetBoostElementQuote,
    /// Submit boost request to receive an invoice
    PaywallBoostElement(command::paywall::PaywallBoostElementArgs),
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let cli = Cli::parse();

    std::env::set_var("VERBOSITY", cli.verbose.to_string());

    if let Some(Commands::SetServer(args)) = &cli.command {
        return command::setup::set_server(args);
    }

    if let Some(Commands::Login(args)) = &cli.command {
        return command::setup::login(args);
    }

    if let Some(Commands::State(args)) = &cli.command {
        return command::setup::state(args);
    }

    if settings::get_str("password")?.is_empty() {
        Err("you need to login first, run btcmap-cli login <password>")?;
    }

    let command = match &cli.command {
        Some(command) => command,
        None => return Ok(()),
    };

    match command {
        // Setup
        Commands::SetServer(_) => Err("supposed to be unreachable".into()),
        Commands::Login(_) => Err("supposed to be unreachable".into()),
        Commands::State(_) => Err("supposed to be unreachable".into()),
        // Admin
        Commands::AddAdmin(args) => command::admin::add_admin(args),
        Commands::AddAdminAction(args) => command::admin::add_admin_action(args),
        Commands::RemoveAdminAction(args) => command::admin::remove_admin_action(args),
        Commands::GenerateInvoice(args) => command::admin::generate_invoice(args),
        Commands::SyncUnpaidInvoices(args) => command::admin::sync_unpaid_invoices(args),
        // Common
        Commands::Search(args) => command::common::search(args),
        // Element
        Commands::GetElement(args) => element::get_element(args),
        Commands::SetElementTag(args) => element::set_element_tag(args),
        Commands::RemoveElementTag(args) => element::remove_element_tag(args),
        Commands::AddElementComment(args) => element::add_element_comment(args),
        Commands::BoostElement(args) => element::boost_element(args),
        Commands::GetBoostedElements(args) => element::get_boosted_elements(args),
        Commands::SyncElements(args) => element::sync_elements(args),
        Commands::GenerateElementIcons(args) => element::generate_element_icons(args),
        Commands::GenerateElementCategories(args) => element::generate_element_categories(args),
        Commands::GenerateElementIssues(args) => element::generate_element_issues(args),
        Commands::GetElementsSnapshot(args) => element::get_elements_snapshot(args),
        // Area
        Commands::GetArea(args) => area::get_area(args),
        Commands::SetAreaTag(args) => area::set_area_tag(args),
        Commands::RemoveAreaTag(args) => area::remove_area_tag(args),
        Commands::SetAreaIcon(args) => area::set_area_icon(args),
        Commands::GenerateAreasElementsMapping(args) => area::generate_areas_elements_mapping(args),
        // User
        Commands::GetUserActivity(args) => command::user::get_user_activity(args),
        // Report
        Commands::GenerateReports(args) => command::report::generate_reports(args),
        Commands::GetTrendingCountries(args) => command::report::get_trending_countries(args),
        Commands::GetTrendingCommunities(args) => command::report::get_trending_communities(args),
        Commands::GetMostCommentedCountries(args) => {
            command::report::get_most_commented_countries(args)
        }
        // Paywall
        Commands::PaywallGetAddElementCommentQuote => {
            command::paywall::paywall_get_add_element_comment_quote()
        }
        Commands::PaywallAddElementComment(args) => {
            command::paywall::paywall_add_element_comment(args)
        }
        Commands::PaywallGetBoostElementQuote => {
            command::paywall::paywall_get_boost_element_quote()
        }
        Commands::PaywallBoostElement(args) => command::paywall::paywall_boost_element(args),
    }
}

pub fn verbosity() -> i64 {
    env::var("VERBOSITY")
        .unwrap_or("".into())
        .parse()
        .unwrap_or(0)
}
