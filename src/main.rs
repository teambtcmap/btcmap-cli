use serde_json::{json, Map, Value};
use std::env;

fn main() {
    let token = env::var("TOKEN").unwrap();
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "get-area" => {
            let id_or_alias = args[2].clone();
            let client = reqwest::blocking::Client::new();
            let args = json!(
                {"jsonrpc": "2.0", "method": "getarea", "params": {"token": token, "area_id_or_alias": id_or_alias}, "id": 1}
            );
            println!("{args}");
            let mut res = client
                .post("https://api.btcmap.org/rpc")
                .body(serde_json::to_string(&args).unwrap())
                .send()
                .unwrap()
                .json::<Map<String, Value>>()
                .unwrap()
                .get("result")
                .unwrap()
                .clone();
            res.get_mut("tags")
                .unwrap()
                .as_object_mut()
                .unwrap()
                .remove("geo_json");
            let res = serde_json::to_string_pretty(&res).unwrap();
            println!("{}", res);
        }
        "set-area-tag" => {
            let id_or_alias = args[2].clone();
            let tag_name = args[3].clone();
            let tag_value = args[4].clone();
            println!("{}", tag_value);
            let tag_value: Value = serde_json::from_str(&tag_value).unwrap();
            let client = reqwest::blocking::Client::new();
            let args = json!(
                {"jsonrpc": "2.0", "method": "setareatag", "params": {"token": token, "area_id_or_alias": id_or_alias, "tag_name": tag_name, "tag_value": tag_value}, "id": 1}
            );
            println!("{args}");
            let mut res = client
                .post("https://api.btcmap.org/rpc")
                .body(serde_json::to_string(&args).unwrap())
                .send()
                .unwrap()
                .json::<Map<String, Value>>()
                .unwrap()
                .get("result")
                .unwrap()
                .clone();
            res.get_mut("tags")
                .unwrap()
                .as_object_mut()
                .unwrap()
                .remove("geo_json");
            let res = serde_json::to_string_pretty(&res).unwrap();
            println!("{}", res);
        }
        "remove-area-tag" => {
            let id_or_alias = args[2].clone();
            let tag_name = args[3].clone();
            let client = reqwest::blocking::Client::new();
            let args = json!(
                {"jsonrpc": "2.0", "method": "removeareatag", "params": {"token": token, "area_id_or_alias": id_or_alias, "tag_name": tag_name}, "id": 1}
            );
            println!("{args}");
            let mut res = client
                .post("https://api.btcmap.org/rpc")
                .body(serde_json::to_string(&args).unwrap())
                .send()
                .unwrap()
                .json::<Map<String, Value>>()
                .unwrap()
                .get("result")
                .unwrap()
                .clone();
            res.get_mut("tags")
                .unwrap()
                .as_object_mut()
                .unwrap()
                .remove("geo_json");
            let res = serde_json::to_string_pretty(&res).unwrap();
            println!("{}", res);
        }
        _ => {}
    }
}
