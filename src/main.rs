use dirs::data_dir;
use rusqlite::params;
use rusqlite::Connection;
use serde_json::{json, Map, Value};
use std::{env, fs::create_dir};

fn main() {
    let data_dir = data_dir().unwrap().join("mapctl");
    println!("Data dir: {}", data_dir.to_str().unwrap());

    if !data_dir.exists() {
        print!("Data dir did not exist, creating...");
        create_dir(&data_dir).unwrap();
    }

    let conn = Connection::open(data_dir.join("mapctl.db")).unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (json TEXT NOT NULL);",
        (), // empty list of parameters.
    )
    .unwrap();

    let mut stmt = conn.prepare("SELECT COUNT (*) FROM settings;").unwrap();
    let mut rows = stmt.query(()).unwrap();
    let first_row = rows.next().unwrap().unwrap();
    let rows: i64 = first_row.get(0).unwrap();

    if rows == 0 {
        conn.execute("INSERT INTO settings (json) VALUES (json('{}'))", ())
            .unwrap();
    }

    let mut stmt = conn
        .prepare("SELECT json_extract(json, '$.token') FROM settings;")
        .unwrap();
    let mut rows = stmt.query(()).unwrap();

    let token: String = match rows.next().unwrap() {
        Some(first_row) => first_row.get(0).unwrap_or("".into()),
        None => "".into(),
    };

    let args: Vec<String> = env::args().collect();
    let command = args[1].as_str();

    if token.is_empty() && command != "set-token" {
        println!("You're not autorized. Please run mapctl set-token <token>");
        return;
    }

    match command {
        "set-token" => {
            let token = args[2].clone();
            conn.execute(
                "UPDATE settings SET json = json_set(json, '$.token', ?1);",
                params![token],
            )
            .unwrap();
            println!("Saved {token} as a token for all future calls");
        }
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