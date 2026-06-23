use std::{env, error::Error};
mod command;
mod rpc;
mod settings;
use clap::{Arg, ArgAction, ArgMatches, Command, FromArgMatches, Subcommand};
use command::area;
use command::element;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

mod sections {
    use crate::command;
    use clap::Subcommand;

    #[derive(Subcommand)]
    pub enum Auth {
        /// Sign up with your username and password and get an auth token
        Signup(command::auth::SignUpArgs),
        /// Sign in with your username and password and get an auth token
        Signin(command::auth::SignInArgs),
        /// Change account password. Knowledge of the old password is required
        ChangePassword(command::auth::ChangePasswordArgs),
        /// List all API keys associated with the authorized user. Secrets are never returned
        GetApiKeys(command::auth::GetApiKeysArgs),
        /// Revoke an API key by its id. Use this to remove keys you no longer need or suspect to be exposed
        RevokeApiKey(command::auth::RevokeApiKeyArgs),
        /// Return the account name and roles of the authorized user
        Whoami(command::auth::WhoAmIArgs),
    }

    #[derive(Subcommand)]
    pub enum Element {
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
        GenerateElementIssues(command::element::GenerateElementIssuesArgs),
        /// Fetch the latest Overpass snapshot and merge it with cached elements. It may take a long time and it's not supposed to be called manually
        SyncElements(command::element::SyncElementsArgs),
        /// Generate icon:android tags for a specific element id range
        GenerateElementIcons(command::element::GenerateElementIconsArgs),
        /// Generate category tags for a specific element id range
        GenerateElementCategories(command::element::GenerateElementCategoriesArgs),
    }

    #[derive(Subcommand)]
    pub enum Area {
        /// Fetch area by either numeric id or string alias (th)
        GetArea(command::area::GetAreaArgs),
        /// Set tag to a certain area. You can use either numeric id or a string alias (th)
        SetAreaTag(command::area::SetAreaTagArgs),
        /// Remove tag from a certain area. You can use either numeric id or a string alias (th)
        RemoveAreaTag(command::area::RemoveAreaTagArgs),
        /// Set icon to a certain area. You can use either numeric id or a string alias (th). Icon needs to be base64-encoded, and you also need to provide file extension
        SetAreaIcon(command::area::SetAreaIconArgs),
        /// Ensure that elements and areas are correctly mapped to each other. You need to provide element id range in order to operate on a specific slice of elements
        GenerateAreasElementsMapping,
    }

    #[derive(Subcommand)]
    pub enum Admin {
        /// Allow other admin to perform a certain action. You must be super admin to use this command
        AddAdminAction(command::admin::AddAdminActionArgs),
        /// Block other admin from using a certain action. You must be super admin to use this command
        RemoveAdminAction(command::admin::RemoveAdminActionArgs),
        /// Create invoice
        CreateInvoice(command::admin::CreateInvoiceArgs),
        /// Sync unpaid invoices
        SyncUnpaidInvoices(command::admin::SyncUnpaidInvoicesArgs),
        /// Get invoice details, you only need to pass invoice ID (integer)
        GetInvoice(command::admin::GetInvoiceArgs),
        SetApiKey(command::admin::SetApiKeyArgs),
    }

    #[derive(Subcommand)]
    pub enum Dashboard {
        /// Fetch the admin analytics dashboard with place and log statistics
        GetDashboard,
        /// Fetch the dashboard for a specific area, including element counts and 365-day charts
        GetAreaDashboard(command::dashboard::GetAreaDashboardArgs),
    }

    #[derive(Subcommand)]
    pub enum Report {
        /// Generate daily reports. It will skip report generation if current date is already covered
        GenerateReports(command::report::GenerateReportsArgs),
        /// Find which countries were trending during a certain time period. Arguments should be valid ISO dates (example: 2024-09-10)
        GetTrendingCountries(command::report::GetTrendingCountriesArgs),
        /// Find which communities were trending during a certain time period. Arguments should be valid ISO dates (example: 2024-09-10)
        GetTrendingCommunities(command::report::GetTrendingCommunitiesArgs),
        /// Find which countries had the most comments during a certain time period. Arguemnts should be valid ISO dates (example: 2024-09-10)
        GetMostCommentedCountries(command::report::GetMostCommentedCountriesArgs),
        /// Get daily infrastructure report containing request statistics, unique IP counts, platform breakdowns, and top user agents
        GetDailyInfraReport(command::report::GetDailyInfraReportArgs),
        /// Get top clients report grouped by platform over the last 24 hours
        GetTopClients(command::report::GetTopClientsArgs),
        /// Generate montly activity report. We use it as a data source in our monthly reports
        GetReport(command::common::GetReportArgs),
    }

    #[derive(Subcommand)]
    pub enum Event {
        CreateEvent(command::event::CreateEventArgs),
        /// Get all events
        GetEvents,
        /// Get event by id
        GetEvent(command::event::GetEventArgs),
        /// Delete event by id
        DeleteEvent(command::event::DeleteEventArgs),
    }

    #[derive(Subcommand)]
    pub enum Import {
        /// Submit new place to BTC Map. Place submissions are processed manually, so use catiously and prefer direct OSM merge.
        SubmitPlace(command::import::SubmitPlaceArgs),
        /// Fetch processing/processed submission to look up all the details.
        GetSubmittedPlace(command::import::GetSubmittedPlaceArgs),
        /// Revoke previously submitted place.
        RevokeSubmittedPlace(command::import::RevokeSubmittedPlaceArgs),
    }

    #[derive(Subcommand)]
    pub enum Setup {
        /// Set a JSON RPC API URL
        SetServer(command::setup::SetServerArgs),
        /// Show all locally cached data
        State(command::setup::StateArgs),
    }

    #[derive(Subcommand)]
    pub enum User {
        /// Fetch the latest user actions. You need to provide OSM username and the number of latest entries you are interested in
        GetUserActivity(command::user::GetUserActivityArgs),
    }

    #[derive(Subcommand)]
    pub enum Matrix {
        /// Send a message to a Matrix room
        SendMatrixMessage(command::matrix::SendMatrixMessageArgs),
    }

    #[derive(Subcommand)]
    pub enum Common {
        /// Return all entities matching provided search query. Currently, only areas are returned
        Search(command::common::SearchArgs),
        /// Custom RPC
        RPC(command::common::CustomArgs),
    }
}

fn section(name: &'static str, about: &'static str) -> Command {
    Command::new(name)
        .about(about)
        .subcommand_required(true)
        .arg_required_else_help(true)
}

fn build_cli() -> Command {
    Command::new("btcmap-cli")
        .about("Manage BTC Map server via JSON RPC API")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::Count)
                .help("Increase verbosity"),
        )
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(sections::Auth::augment_subcommands(section(
            "auth",
            "Authentication and account management",
        )))
        .subcommand(sections::Element::augment_subcommands(section(
            "element",
            "Element (place) operations",
        )))
        .subcommand(sections::Area::augment_subcommands(section(
            "area",
            "Area operations",
        )))
        .subcommand(sections::Admin::augment_subcommands(section(
            "admin",
            "Admin-only operations",
        )))
        .subcommand(sections::Dashboard::augment_subcommands(section(
            "dashboard",
            "Dashboard analytics",
        )))
        .subcommand(sections::Report::augment_subcommands(section(
            "report",
            "Reports and analytics",
        )))
        .subcommand(sections::Event::augment_subcommands(section(
            "event",
            "Server events",
        )))
        .subcommand(sections::Import::augment_subcommands(section(
            "import",
            "Place submission and review",
        )))
        .subcommand(sections::Setup::augment_subcommands(section(
            "setup",
            "Local CLI setup and configuration",
        )))
        .subcommand(sections::User::augment_subcommands(section(
            "user",
            "User activity",
        )))
        .subcommand(sections::Matrix::augment_subcommands(section(
            "matrix",
            "Matrix messaging",
        )))
        .subcommand(sections::Common::augment_subcommands(section(
            "common",
            "General-purpose commands",
        )))
}

fn main() -> Result<()> {
    let matches = build_cli().get_matches();
    let verbose = matches.get_count("verbose");
    unsafe { std::env::set_var("VERBOSITY", verbose.to_string()) };

    let (section, sub_matches) = matches
        .subcommand()
        .expect("subcommand_required(true) guarantees a section");

    if let Some((cmd, cmd_matches)) = sub_matches.subcommand() {
        match (section, cmd) {
            ("setup", "set-server") => {
                let args = command::setup::SetServerArgs::from_arg_matches(cmd_matches)?;
                return command::setup::set_server(&args);
            }
            ("setup", "state") => {
                let args = command::setup::StateArgs::from_arg_matches(cmd_matches)?;
                return command::setup::state(&args);
            }
            ("auth", "signin") => {
                let args = command::auth::SignInArgs::from_arg_matches(cmd_matches)?;
                return command::auth::sign_in(&args);
            }
            ("auth", "signup") => {
                let args = command::auth::SignUpArgs::from_arg_matches(cmd_matches)?;
                return command::auth::sign_up(&args);
            }
            ("auth", "change-password") => {
                let args = command::auth::ChangePasswordArgs::from_arg_matches(cmd_matches)?;
                return command::auth::change_password(&args);
            }
            _ => {}
        }
    }

    if settings::get_str("password")?.is_empty() {
        Err("you need to sign in first, run btcmap-cli auth signin <username> <password>")?;
    }

    dispatch(section, sub_matches)
}

fn dispatch(section: &str, sub_matches: &ArgMatches) -> Result<()> {
    match section {
        "auth" => match sections::Auth::from_arg_matches(sub_matches)? {
            sections::Auth::GetApiKeys(args) => command::auth::get_api_keys(&args),
            sections::Auth::RevokeApiKey(args) => command::auth::revoke_api_key(&args),
            sections::Auth::Whoami(args) => command::auth::whoami(&args),
            sections::Auth::Signup(_)
            | sections::Auth::Signin(_)
            | sections::Auth::ChangePassword(_) => {
                unreachable!("pre-auth variants handled above")
            }
        },
        "element" => match sections::Element::from_arg_matches(sub_matches)? {
            sections::Element::GetElement(args) => element::get_element(&args),
            sections::Element::SetElementTag(args) => element::set_element_tag(&args),
            sections::Element::RemoveElementTag(args) => element::remove_element_tag(&args),
            sections::Element::GetBoostedElements => element::get_boosted_elements(),
            sections::Element::BoostElement(args) => element::boost_element(&args),
            sections::Element::PaywallGetBoostElementQuote => {
                element::paywall_get_boost_element_quote()
            }
            sections::Element::PaywallBoostElement(args) => element::paywall_boost_element(&args),
            sections::Element::AddElementComment(args) => element::add_element_comment(&args),
            sections::Element::PaywallGetAddElementCommentQuote => {
                element::paywall_get_add_element_comment_quote()
            }
            sections::Element::PaywallAddElementComment(args) => {
                element::paywall_add_element_comment(&args)
            }
            sections::Element::GenerateElementIssues(args) => element::generate_element_issues(&args),
            sections::Element::SyncElements(args) => element::sync_elements(&args),
            sections::Element::GenerateElementIcons(args) => element::generate_element_icons(&args),
            sections::Element::GenerateElementCategories(args) => {
                element::generate_element_categories(&args)
            }
        },
        "area" => match sections::Area::from_arg_matches(sub_matches)? {
            sections::Area::GetArea(args) => area::get_area(&args),
            sections::Area::SetAreaTag(args) => area::set_area_tag(&args),
            sections::Area::RemoveAreaTag(args) => area::remove_area_tag(&args),
            sections::Area::SetAreaIcon(args) => area::set_area_icon(&args),
            sections::Area::GenerateAreasElementsMapping => area::generate_areas_elements_mapping(),
        },
        "admin" => match sections::Admin::from_arg_matches(sub_matches)? {
            sections::Admin::AddAdminAction(args) => command::admin::add_admin_action(&args),
            sections::Admin::RemoveAdminAction(args) => command::admin::remove_admin_action(&args),
            sections::Admin::CreateInvoice(args) => command::admin::create_invoice(&args),
            sections::Admin::SyncUnpaidInvoices(args) => command::admin::sync_unpaid_invoices(&args),
            sections::Admin::GetInvoice(args) => command::admin::get_invoice(&args),
            sections::Admin::SetApiKey(args) => command::admin::set_api_key(&args),
        },
        "dashboard" => match sections::Dashboard::from_arg_matches(sub_matches)? {
            sections::Dashboard::GetDashboard => command::dashboard::get_dashboard(),
            sections::Dashboard::GetAreaDashboard(args) => {
                command::dashboard::get_area_dashboard(&args)
            }
        },
        "report" => match sections::Report::from_arg_matches(sub_matches)? {
            sections::Report::GenerateReports(args) => command::report::generate_reports(&args),
            sections::Report::GetTrendingCountries(args) => {
                command::report::get_trending_countries(&args)
            }
            sections::Report::GetTrendingCommunities(args) => {
                command::report::get_trending_communities(&args)
            }
            sections::Report::GetMostCommentedCountries(args) => {
                command::report::get_most_commented_countries(&args)
            }
            sections::Report::GetDailyInfraReport(args) => {
                command::report::get_daily_infra_report(&args)
            }
            sections::Report::GetTopClients(args) => command::report::get_top_clients(&args),
            sections::Report::GetReport(args) => command::common::get_report(&args),
        },
        "event" => match sections::Event::from_arg_matches(sub_matches)? {
            sections::Event::CreateEvent(args) => command::event::create_event(&args),
            sections::Event::GetEvents => command::event::get_events(),
            sections::Event::GetEvent(args) => command::event::get_event(&args),
            sections::Event::DeleteEvent(args) => command::event::delete_event(&args),
        },
        "import" => match sections::Import::from_arg_matches(sub_matches)? {
            sections::Import::SubmitPlace(args) => command::import::submit_place(&args),
            sections::Import::GetSubmittedPlace(args) => command::import::get_submitted_place(&args),
            sections::Import::RevokeSubmittedPlace(args) => {
                command::import::revoke_submitted_place(&args)
            }
        },
        "setup" => match sections::Setup::from_arg_matches(sub_matches)? {
            sections::Setup::SetServer(_) | sections::Setup::State(_) => {
                unreachable!("pre-auth variants handled above")
            }
        },
        "user" => match sections::User::from_arg_matches(sub_matches)? {
            sections::User::GetUserActivity(args) => command::user::get_user_activity(&args),
        },
        "matrix" => match sections::Matrix::from_arg_matches(sub_matches)? {
            sections::Matrix::SendMatrixMessage(args) => command::matrix::send_matrix_message(&args),
        },
        "common" => match sections::Common::from_arg_matches(sub_matches)? {
            sections::Common::Search(args) => command::common::search(&args),
            sections::Common::RPC(args) => command::common::rpc(&args),
        },
        _ => unreachable!("all sections are explicitly matched"),
    }
}

pub fn verbosity() -> i64 {
    env::var("VERBOSITY")
        .unwrap_or("".into())
        .parse()
        .unwrap_or(0)
}