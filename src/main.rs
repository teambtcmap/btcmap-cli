use std::{env, error::Error};
mod rpc;
mod settings;
use clap::{Parser, Subcommand};
use command::{
    area,
    element::{
        self, GenerateElementCategoriesArgs, GenerateElementIconsArgs, GenerateElementIssuesArgs,
    },
};
mod command;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Set a JSON RPC API URL
    SetServer(command::setup::SetServerArgs),
    /// Show all locally cached data
    State(command::setup::StateArgs),

    /// Fetch element by id
    GetElement(command::element::GetElementArgs),
    /// Set tag to a certain element. Every tag must be a valid JSON value. Nulls are not allowed and will be interpreted as deletion requests
    SetElementTag(command::element::SetElementTagArgs),
    /// Remove tag from a certain element
    RemoveElementTag(command::element::RemoveElementTagArgs),
    /// Get all boosted elements
    GetBoostedElements,
    /// Boost an element for a set number of days. You can use either numeric or OSM (node:12345) id
    BoostElement(command::element::BoostElementArgs),
    /// Get current element boost price in sats
    PaywallGetBoostElementQuote,
    /// Submit boost request to receive an invoice
    PaywallBoostElement(command::element::PaywallBoostElementArgs),
    /// Add coment to a certain element
    AddElementComment(command::element::AddElementCommentArgs),
    /// Get current element comment price in sats
    PaywallGetAddElementCommentQuote,
    /// Submit comment to receive an invoice
    PaywallAddElementComment(command::element::PaywallAddElementCommentArgs),
    /// Generate issues tags for a specific element id range. This command is supposed to be called automatically by a BTC Map server internal shceduler
    GenerateElementIssues(GenerateElementIssuesArgs),
    /// Fetch the latest Overpass snapshot and merge it with cached elements. It may take a long time and it's not supposed to be called manually
    SyncElements(command::element::SyncElementsArgs),
    /// Generate icon:android tags for a specific element id range
    GenerateElementIcons(GenerateElementIconsArgs),
    /// Generate category tags for a specific element id range
    GenerateElementCategories(GenerateElementCategoriesArgs),

    // Auth - https://github.com/teambtcmap/btcmap-api/blob/master/docs/rpc-api/auth.md
    /// Change admin password. Knowledge of an old password is required
    ChangePassword(command::admin::ChangePasswordArgs),
    /// Create API key. You need to provide your username and password, as well as a key label
    CreateApiKey(command::admin::CreateApiKeyArgs),
    /// Login with your username and password and get an auth token
    Login(command::admin::LoginArgs),
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
    /// Get invoice details, you only need to pass invoice ID (integer)
    GetInvoice(command::admin::GetInvoiceArgs),
    /// Return all entities matching provided search query. Currently, only areas are returned
    Search(command::common::SearchArgs),
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

    /// Custom RPC
    RPC(command::common::CustomArgs),

    CreateEvent(command::event::CreateEventArgs),
    GetEvents,
    GetEvent(command::event::GetEventArgs),
    DeleteEvent(command::event::DeleteEventArgs),
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let cli = Cli::parse();

    std::env::set_var("VERBOSITY", cli.verbose.to_string());

    if let Some(Commands::SetServer(args)) = &cli.command {
        return command::setup::set_server(args);
    }

    if let Some(Commands::ChangePassword(args)) = &cli.command {
        return command::admin::change_password(args);
    }

    if let Some(Commands::Login(args)) = &cli.command {
        return command::admin::login(args);
    }

    if let Some(Commands::CreateApiKey(args)) = &cli.command {
        return command::admin::create_api_key(args);
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
        Commands::CreateApiKey(_) => Err("supposed to be unreachable".into()),
        Commands::State(_) => Err("supposed to be unreachable".into()),
        Commands::ChangePassword(_) => Err("supposed to be unreachable".into()),
        // Element
        Commands::GetElement(args) => element::get_element(args),
        Commands::SetElementTag(args) => element::set_element_tag(args),
        Commands::RemoveElementTag(args) => element::remove_element_tag(args),
        Commands::GetBoostedElements => element::get_boosted_elements(),
        Commands::BoostElement(args) => element::boost_element(args),
        Commands::PaywallGetBoostElementQuote => {
            command::element::paywall_get_boost_element_quote()
        }
        Commands::PaywallBoostElement(args) => command::element::paywall_boost_element(args),
        Commands::AddElementComment(args) => element::add_element_comment(args),
        Commands::PaywallGetAddElementCommentQuote => {
            command::element::paywall_get_add_element_comment_quote()
        }
        Commands::PaywallAddElementComment(args) => {
            command::element::paywall_add_element_comment(args)
        }
        Commands::GenerateElementIssues(args) => element::generate_element_issues(args),
        Commands::SyncElements(args) => element::sync_elements(args),
        Commands::GenerateElementIcons(args) => element::generate_element_icons(args),
        Commands::GenerateElementCategories(args) => element::generate_element_categories(args),
        // Admin
        Commands::AddAdmin(args) => command::admin::add_admin(args),
        Commands::AddAdminAction(args) => command::admin::add_admin_action(args),
        Commands::RemoveAdminAction(args) => command::admin::remove_admin_action(args),
        Commands::GenerateInvoice(args) => command::admin::generate_invoice(args),
        Commands::SyncUnpaidInvoices(args) => command::admin::sync_unpaid_invoices(args),
        Commands::GetInvoice(args) => command::admin::get_invoice(args),
        // Common
        Commands::Search(args) => command::common::search(args),
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
        // common
        Commands::RPC(args) => command::common::rpc(args),
        Commands::CreateEvent(args) => command::event::create_event(args),
        Commands::GetEvents => command::event::get_events(),
        Commands::GetEvent(args) => command::event::get_event(args),
        Commands::DeleteEvent(args) => command::event::delete_event(args),
    }
}

pub fn verbosity() -> i64 {
    env::var("VERBOSITY")
        .unwrap_or("".into())
        .parse()
        .unwrap_or(0)
}
