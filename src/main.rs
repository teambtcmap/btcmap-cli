use serde_json::json;
use std::env;
mod db;
mod rpc;

fn main() {
    let conn = db::connect();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("You need to specify command, run btcmap-cli help for more details");
        std::process::exit(1);
    }
    let command = args[1].as_str();
    if db::query_settings_string("token", &conn).is_empty() && command != "set-token" {
        eprintln!("You need to login first, run btcmap-cli set-token <token>");
        std::process::exit(1);
    }
    match command {
        "set-server" => {
            let mut url = args[2].clone();
            if url == "prod" {
                url = "https://api.btcmap.org/rpc".into();
            }
            if url == "dev" {
                url = "http://127.0.0.1:8000/rpc".into();
            }
            db::insert_settings_string("api_url", &url, &conn);
            println!("Saved {url} as a server for all future commands");
        }
        "set-token" => {
            let token = args[2].clone();
            db::insert_settings_string("token", &token, &conn);
            println!("Saved {token} as a token for all future commands");
        }
        "get-element" => {
            let id = args[2].clone().replace("=", ":");
            rpc::call_remote_procedure("getelement", json!({"id":id}));
        }
        "boost-element" => {
            let id = args[2].clone().replace("=", ":");
            let days: i64 = args[3].parse().unwrap();
            rpc::call_remote_procedure("boostelement", json!({"id":id,"days":days}));
        }
        "generate-reports" => rpc::call_remote_procedure("generatereports", json!({})),
        "generate-element-icons" => {
            let from_element_id: i64 = args[2].clone().parse().unwrap();
            let to_element_id: i64 = args[3].clone().parse().unwrap();
            rpc::call_remote_procedure(
                "generateelementicons",
                json!({"from_element_id":from_element_id,"to_element_id":to_element_id}),
            );
        }
        "generate-element-categories" => {
            let from_element_id: i64 = args[2].clone().parse().unwrap();
            let to_element_id: i64 = args[3].clone().parse().unwrap();
            rpc::call_remote_procedure(
                "generateelementcategories",
                json!({"from_element_id":from_element_id,"to_element_id":to_element_id}),
            );
        }
        "add-element-comment" => {
            let id = args[2].clone().replace("=", ":");
            let comment = args[3].clone();
            rpc::call_remote_procedure("addelementcomment", json!({"id":id,"comment":comment}));
        }
        "get-area" => {
            let id = args[2].clone();
            rpc::call_remote_procedure("getarea", json!({"id":id}));
        }
        "set-area-tag" => {
            let id = args[2].clone();
            let name = args[3].clone();
            let value = args[4].clone();
            rpc::call_remote_procedure("setareatag", json!({"id":id,"name":name,"value":value}));
        }
        "remove-area-tag" => {
            let id = args[2].clone();
            let tag = args[3].clone();
            rpc::call_remote_procedure("removeareatag", json!({"id":id,"tag":tag}));
        }
        "get-trending-countries" => {
            let period_start = args[2].clone();
            let period_end = args[3].clone();
            rpc::call_remote_procedure(
                "gettrendingcountries",
                json!({"period_start":period_start,"period_end":period_end}),
            );
        }
        "get-trending-communities" => {
            let period_start = args[2].clone();
            let period_end = args[3].clone();
            rpc::call_remote_procedure(
                "gettrendingcommunities",
                json!({"period_start":period_start,"period_end":period_end}),
            );
        }
        "generate-element-issues" => rpc::call_remote_procedure("generateelementissues", json!({})),
        "sync-elements" => rpc::call_remote_procedure("syncelements", json!({})),
        "get-most-commented-countries" => {
            let period_start = args[2].clone();
            let period_end = args[3].clone();
            rpc::call_remote_procedure(
                "getmostcommentedcountries",
                json!({"period_start":period_start,"period_end":period_end}),
            );
        }
        "generate-areas-elements-mapping" => {
            let from_element_id: i64 = args[2].clone().parse().unwrap();
            let to_element_id: i64 = args[3].clone().parse().unwrap();
            rpc::call_remote_procedure(
                "getmostcommentedcountries",
                json!({"from_element_id":from_element_id,"to_element_id":to_element_id}),
            );
        }
        _ => {
            eprintln!("Command does not exist, check btcmap-cli help to see all available commands")
        }
    }
}
