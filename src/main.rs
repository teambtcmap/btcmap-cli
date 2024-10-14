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
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[derive(Subcommand)]
enum Commands {
    // Setup
    SetServer(command::setup::SetServerArgs),
    Login(command::setup::LoginArgs),
    State(command::setup::StateArgs),
    // Admin
    AddAdmin(command::admin::AddAdminArgs),
    AddAllowedAction(command::admin::AddAllowedActionArgs),
    RemoveAllowedAction(command::admin::RemoveAllowedActionArgs),
    // Common
    Search(command::common::SearchArgs),
    // Element
    GetElement(command::element::GetElementArgs),
    SetElementTag(command::element::SetElementTagArgs),
    RemoveElementTag(command::element::RemoveElementTagArgs),
    AddElementComment(command::element::AddElementCommentArgs),
    BoostElement(command::element::BoostElementArgs),
    GetBoosts(command::element::GetBoostsArgs),
    SyncElements(command::element::SyncElementsArgs),
    GenerateElementIcons(GenerateElementIconsArgs),
    GenerateElementCategories(GenerateElementCategoriesArgs),
    GenerateElementIssues(GenerateElementIssuesArgs),
    // Area
    GetArea(command::area::GetAreaArgs),
    SetAreaTag(command::area::SetAreaTagArgs),
    RemoveAreaTag(command::area::RemoveAreaTagArgs),
    SetAreaIcon(command::area::SetAreaIconArgs),
    GenerateAreasElementsMapping(command::area::GenerateAreasElementsMappingArgs),
    // User
    GetUserActivity(command::user::GetUserActivityArgs),
    // Report
    GenerateReports(command::report::GenerateReportsArgs),
    GetTrendingCountries(command::report::GetTrendingCountriesArgs),
    GetTrendingCommunities(command::report::GetTrendingCommunitiesArgs),
    GetMostCommentedCountries(command::report::GetMostCommentedCountriesArgs),
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
        Commands::AddAllowedAction(args) => command::admin::add_allowed_action(args),
        Commands::RemoveAllowedAction(args) => command::admin::remove_allowed_action(args),
        // Common
        Commands::Search(args) => command::common::search(args),
        // Element
        Commands::GetElement(args) => element::get_element(args),
        Commands::SetElementTag(args) => element::set_element_tag(args),
        Commands::RemoveElementTag(args) => element::remove_element_tag(args),
        Commands::AddElementComment(args) => element::add_element_comment(args),
        Commands::BoostElement(args) => element::boost_element(args),
        Commands::GetBoosts(args) => element::get_boosts(args),
        Commands::SyncElements(args) => element::sync_elements(args),
        Commands::GenerateElementIcons(args) => element::generate_element_icons(args),
        Commands::GenerateElementCategories(args) => element::generate_element_categories(args),
        Commands::GenerateElementIssues(args) => element::generate_element_issues(args),
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
    }
}

pub fn verbosity() -> i64 {
    env::var("VERBOSITY")
        .unwrap_or("".into())
        .parse()
        .unwrap_or(0)
}
