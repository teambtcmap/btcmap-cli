use serde_json::json;
use std::env;
use std::error::Error;

mod db;
mod rpc;

const UNAUTHORIZED_ACTIONS: [&str; 2] = ["login", "help"];

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        Err(
            "you need to provide an action, run btcmap-cli help to see all supported actions"
        )?;
    }
    let action = args[1].as_str();
    let password = db::query_settings_string("password", &db::connect()?)?;
    if password.is_empty() && !UNAUTHORIZED_ACTIONS.contains(&action) {
        Err("you need to login first, run btcmap-cli login <password>")?;
    }
    match action {
        "help" => help(),
        "set-server" => {
            let mut url = args[2].clone();
            if url == "prod" {
                url = "https://api.btcmap.org/rpc".into();
            }
            if url == "dev" {
                url = "http://127.0.0.1:8000/rpc".into();
            }
            db::insert_settings_string("api_url", &url, &db::connect()?)?;
            println!("saved {url} as a server for all future actions");
        }
        "login" => {
            let token = args[2].clone();
            db::insert_settings_string("password", &token, &db::connect()?)?;
            println!("saved {token} as a password for all future actions");
        }
        "get-element" => {
            let id = args[2].clone().replace("=", ":");
            rpc::call_remote_procedure("getelement", json!({"id":id}))?;
        }
        "boost-element" => {
            let id = args[2].clone().replace("=", ":");
            let days: i64 = args[3].parse().unwrap();
            rpc::call_remote_procedure("boostelement", json!({"id":id,"days":days}))?;
        }
        "generate-reports" => rpc::call_remote_procedure("generatereports", json!({}))?,
        "generate-element-icons" => {
            let from_element_id: i64 = args[2].clone().parse().unwrap();
            let to_element_id: i64 = args[3].clone().parse().unwrap();
            rpc::call_remote_procedure(
                "generateelementicons",
                json!({"from_element_id":from_element_id,"to_element_id":to_element_id}),
            )?;
        }
        "generate-element-categories" => {
            let from_element_id: i64 = args[2].clone().parse().unwrap();
            let to_element_id: i64 = args[3].clone().parse().unwrap();
            rpc::call_remote_procedure(
                "generateelementcategories",
                json!({"from_element_id":from_element_id,"to_element_id":to_element_id}),
            )?;
        }
        "add-element-comment" => {
            let id = args[2].clone().replace("=", ":");
            let comment = args[3].clone();
            rpc::call_remote_procedure("addelementcomment", json!({"id":id,"comment":comment}))?;
        }
        "get-area" => {
            let id = args[2].clone();
            rpc::call_remote_procedure("getarea", json!({"id":id}))?;
        }
        "set-area-tag" => {
            let id = args[2].clone();
            let name = args[3].clone();
            let value = args[4].clone();
            rpc::call_remote_procedure("setareatag", json!({"id":id,"name":name,"value":value}))?;
        }
        "remove-area-tag" => {
            let id = args[2].clone();
            let tag = args[3].clone();
            rpc::call_remote_procedure("removeareatag", json!({"id":id,"tag":tag}))?;
        }
        "get-trending-countries" => {
            let period_start = args[2].clone();
            let period_end = args[3].clone();
            rpc::call_remote_procedure(
                "gettrendingcountries",
                json!({"period_start":period_start,"period_end":period_end}),
            )?;
        }
        "get-trending-communities" => {
            let period_start = args[2].clone();
            let period_end = args[3].clone();
            rpc::call_remote_procedure(
                "gettrendingcommunities",
                json!({"period_start":period_start,"period_end":period_end}),
            )?;
        }
        "generate-element-issues" => rpc::call_remote_procedure("generateelementissues", json!({}))?,
        "sync-elements" => rpc::call_remote_procedure("syncelements", json!({}))?,
        "get-most-commented-countries" => {
            let period_start = args[2].clone();
            let period_end = args[3].clone();
            rpc::call_remote_procedure(
                "getmostcommentedcountries",
                json!({"period_start":period_start,"period_end":period_end}),
            )?;
        }
        "generate-areas-elements-mapping" => {
            let from_element_id: i64 = args[2].clone().parse().unwrap();
            let to_element_id: i64 = args[3].clone().parse().unwrap();
            rpc::call_remote_procedure(
                "getmostcommentedcountries",
                json!({"from_element_id":from_element_id,"to_element_id":to_element_id}),
            )?;
        }
        "add-allowed-action" => {
            let admin_name = args[2].clone();
            let action = args[3].clone();
            rpc::call_remote_procedure(
                "addallowedaction",
                json!({"admin_name":admin_name,"action":action}),
            )?;
        }
        "remove-allowed-action" => {
            let admin_name = args[2].clone();
            let action = args[3].clone();
            rpc::call_remote_procedure(
                "removeallowedaction",
                json!({"admin_name":admin_name,"action":action}),
            )?;
        }
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
}
