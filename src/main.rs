use serde_json::json;
use std::env;
use std::env::Args;
use std::error::Error;
mod rpc;
mod settings;

const UNAUTHORIZED_ACTIONS: [&str; 2] = ["login", "help"];

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn get_arg(args: &mut Args) -> Result<String> {
    Ok(args
        .next()
        .ok_or("You need to provide one more command line argument")?)
}

fn main() -> Result<()> {
    let mut args = env::args();
    drop(args.next());
    let action = args
        .next()
        .ok_or("you need to provide an action, run btcmap-cli help to see all supported actions")?;
    let action = action.replace("-", "_");
    let action = action.as_str();
    if settings::get_str("password")?.is_empty() && !UNAUTHORIZED_ACTIONS.contains(&action) {
        Err("you need to login first, run btcmap-cli login <password>")?;
    }
    match action {
        "help" => help(),
        "set_server" => {
            let url = get_arg(&mut args)?;
            let url = url.as_str();
            let url = match url {
                "prod" => "https://api.btcmap.org/rpc",
                "dev" => "http://127.0.0.1:8000/rpc",
                _ => url,
            };
            settings::put_str("api_url", &url)?;
            println!("saved {url} as a server for all future actions");
        }
        "login" => {
            let token = get_arg(&mut args)?;
            settings::put_str("password", &token)?;
            println!("saved {token} as a password for all future actions");
        }
        "get_element" => {
            let id = get_arg(&mut args)?.replace("=", ":");
            rpc::call(action, json!({"id":id}))?;
        }
        "boost_element" => {
            let id = get_arg(&mut args)?.replace("=", ":");
            let days = get_arg(&mut args)?.parse::<i64>()?;
            rpc::call(action, json!({"id":id,"days":days}))?;
        }
        "generate_reports" => rpc::call(action, json!({}))?,
        "generate_element_icons" => {
            let from_element_id = get_arg(&mut args)?.parse::<i64>()?;
            let to_element_id = get_arg(&mut args)?.parse::<i64>()?;
            rpc::call(
                action,
                json!({"from_element_id":from_element_id,"to_element_id":to_element_id}),
            )?;
        }
        "generate_element_categories" => {
            let from_element_id = get_arg(&mut args)?.parse::<i64>()?;
            let to_element_id = get_arg(&mut args)?.parse::<i64>()?;
            rpc::call(
                action,
                json!({"from_element_id":from_element_id,"to_element_id":to_element_id}),
            )?;
        }
        "add_element_comment" => {
            let id = get_arg(&mut args)?.replace("=", ":");
            let comment = get_arg(&mut args)?;
            rpc::call(action, json!({"id":id,"comment":comment}))?;
        }
        "get_area" => {
            let id = get_arg(&mut args)?;
            rpc::call(action, json!({"id":id}))?;
        }
        "set_area_tag" => {
            let id = get_arg(&mut args)?;
            let name = get_arg(&mut args)?;
            let value = get_arg(&mut args)?;
            rpc::call(action, json!({"id":id,"name":name,"value":value}))?;
        }
        "remove_area_tag" => {
            let id = get_arg(&mut args)?;
            let tag = get_arg(&mut args)?;
            rpc::call(action, json!({"id":id,"tag":tag}))?;
        }
        "get_trending_countries" => {
            let period_start = get_arg(&mut args)?;
            let period_end = get_arg(&mut args)?;
            rpc::call(
                action,
                json!({"period_start":period_start,"period_end":period_end}),
            )?;
        }
        "get_trending_communities" => {
            let period_start = get_arg(&mut args)?;
            let period_end = get_arg(&mut args)?;
            rpc::call(
                action,
                json!({"period_start":period_start,"period_end":period_end}),
            )?;
        }
        "generate_element_issues" => rpc::call(action, json!({}))?,
        "sync_elements" => rpc::call(action, json!({}))?,
        "get_most_commented_countries" => {
            let period_start = get_arg(&mut args)?;
            let period_end = get_arg(&mut args)?;
            rpc::call(
                action,
                json!({"period_start":period_start,"period_end":period_end}),
            )?;
        }
        "generate_areas_elements_mapping" => {
            let from_element_id = get_arg(&mut args)?.parse::<i64>()?;
            let to_element_id = get_arg(&mut args)?.parse::<i64>()?;
            rpc::call(
                action,
                json!({"from_element_id":from_element_id,"to_element_id":to_element_id}),
            )?;
        }
        "add_allowed_action" => {
            let admin_name = get_arg(&mut args)?;
            let allowed_action = get_arg(&mut args)?;
            rpc::call(
                action,
                json!({"admin_name":admin_name,"action":allowed_action}),
            )?;
        }
        "remove_allowed_action" => {
            let admin_name = get_arg(&mut args)?;
            let allowed_action = get_arg(&mut args)?;
            rpc::call(
                action,
                json!({"admin_name":admin_name,"action":allowed_action}),
            )?;
        }
        "get_user_activity" => {
            let id = get_arg(&mut args)?;
            let limit = get_arg(&mut args)
                .unwrap_or(100000.to_string())
                .parse::<i64>()?;
            rpc::call(action, json!({"id":id,"limit":limit}))?;
        }
        "search" => {
            let query = get_arg(&mut args)?;
            rpc::call(action, json!({"query":query}))?;
        }
        "set_area_icon" => {
            let id = get_arg(&mut args)?;
            let icon_base64 = get_arg(&mut args)?;
            let icon_ext = get_arg(&mut args)?;
            rpc::call(
                action,
                json!({"id":id,"icon_base64":icon_base64,"icon_ext":icon_ext}),
            )?;
        }
        "get-boosts" => rpc::call(action, json!({}))?,
        _ => {
            eprintln!("action {action} does not exist, check btcmap-cli help to see all available actions")
        }
    }
    Ok(())
}

fn help() {
    println!("add-admin <name:string> <password:string>");
    println!("add-area <tags:json>");
    println!("add-element-comment <element_id:string> <comment:string>");
    println!("boost-element <id:string> <days:integer>");
    println!("generate-areas-elements-mapping <from_element_id:integer> <to_element_id:integer>");
    println!("generate-element-categories <from_element_id:integer> <to_element_id:integer>");
    println!("generate-element-icons <from_element_id:integer> <to_element_id:integer>");
    println!("generate-element-issues");
    println!("generate-reports");
    println!("get-area <id:string>");
    println!("get-element <id:string>");
    println!("get-most-commented-countries <period_start:date> <period_end:date>");
    println!("get-trending-communities <period_start:date> <period_end:date>");
    println!("get-trending-countries <period_start:date> <period_end:date>");
    println!("remove-area <id:string>");
    println!("remove-area-tag <area_id:string> <tag:string>");
    println!("remove-element-tag <element_id:string> <tag:string>");
    println!("set-area-tag <area_id:string> <tag_name:string> <tag_value:string>");
    println!("set-element-tag <element_id:string> <tag_name:string> <tag_value:string>");
    println!("sync-elements");
    println!("add-allowed-action <admin_name:string> <action:string>");
    println!("remove-allowed-action <admin_name:string> <action:string>");
    println!("get-user-activity <id:string> [limit:int]");
    println!("search <query:string>");
    println!("set-area-icon <id:string> <icon_base64:string> <icon_ext:string>");
    println!("get-boosts");
}
