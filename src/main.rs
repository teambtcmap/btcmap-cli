use dirs::data_dir;
use reqwest::blocking::ClientBuilder;
use reqwest::blocking::Response;
use rusqlite::params;
use rusqlite::Connection;
use serde_json::{json, Map, Value};
use std::path::PathBuf;
use std::{env, fs::create_dir};

fn main() {
    let conn = db_connect();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("You need to specify command, run mapctl help for more details");
    }
    let command = args[1].as_str();
    if query_settings_string("token", &conn).is_empty() && command != "set-token" {
        println!("You need to login first, run mapctl set-token <token>");
        return;
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
            conn.execute(
                "UPDATE settings SET json = json_set(json, '$.api_url', ?1);",
                params![url],
            )
            .unwrap();
            println!("Saved {url} as a server for all future commands");
        }
        "set-token" => {
            let token = args[2].clone();
            conn.execute(
                "UPDATE settings SET json = json_set(json, '$.token', ?1);",
                params![token],
            )
            .unwrap();
            println!("Saved {token} as a token for all future commands");
        }
        "get-element" => {
            let id = args[2].clone().replace("=", ":");
            call_remote_procedure("getelement", json!({"id":id}));
        }
        "boost-element" => {
            let id = args[2].clone().replace("=", ":");
            let days: i64 = args[3].parse().unwrap();
            call_remote_procedure("boostelement", json!({"id":id,"days":days}));
        }
        "generate-reports" => {
            call_remote_procedure("generatereports", json!({}));
        }
        "generate-element-icons" => {
            let from_element_id: i64 = args[2].clone().parse().unwrap();
            let to_element_id: i64 = args[3].clone().parse().unwrap();
            call_remote_procedure(
                "generateelementicons",
                json!({"from_element_id":from_element_id,"to_element_id":to_element_id}),
            );
        }
        "generate-element-categories" => {
            let from_element_id: i64 = args[2].clone().parse().unwrap();
            let to_element_id: i64 = args[3].clone().parse().unwrap();
            call_remote_procedure(
                "generateelementcategories",
                json!({"from_element_id":from_element_id,"to_element_id":to_element_id}),
            );
        }
        "add-element-comment" => {
            let id = args[2].clone().replace("=", ":");
            let comment = args[3].clone();
            call_remote_procedure("addelementcomment", json!({"id":id,"comment":comment}));
        }
        "get-area" => {
            let id = args[2].clone();
            call_remote_procedure("getarea", json!({"id":id}));
        }
        "set-area-tag" => {
            let id = args[2].clone();
            let name = args[3].clone();
            let value = args[4].clone();
            call_remote_procedure("setareatag", json!({"id":id,"name":name,"value":value}));
        }
        "remove-area-tag" => {
            let id = args[2].clone();
            let tag = args[3].clone();
            call_remote_procedure("removeareatag", json!({"id":id,"tag":tag}));
        }
        "get-trending-countries" => {
            let period_start = args[2].clone();
            let period_end = args[3].clone();
            call_remote_procedure(
                "gettrendingcountries",
                json!({"period_start":period_start,"period_end":period_end}),
            );
        }
        "get-trending-communities" => {
            let period_start = args[2].clone();
            let period_end = args[3].clone();
            call_remote_procedure(
                "gettrendingcommunities",
                json!({"period_start":period_start,"period_end":period_end}),
            );
        }
        "generate-element-issues" => {
            call_remote_procedure("generateelementissues", json!({}));
        }
        "sync-elements" => {
            call_remote_procedure("syncelements", json!({}));
        }
        "get-most-commented-countries" => {
            let period_start = args[2].clone();
            let period_end = args[3].clone();
            call_remote_procedure(
                "getmostcommentedcountries",
                json!({"period_start":period_start,"period_end":period_end}),
            );
        }
        "generate-areas-elements-mapping" => {
            let from_element_id: i64 = args[2].clone().parse().unwrap();
            let to_element_id: i64 = args[3].clone().parse().unwrap();
            call_remote_procedure(
                "getmostcommentedcountries",
                json!({"from_element_id":from_element_id,"to_element_id":to_element_id}),
            );
        }
        _ => {}
    }
}

fn call_remote_procedure(name: &str, mut params: Value) {
    let params = params.as_object_mut().unwrap();
    params.insert(
        "token".into(),
        Value::String(query_settings_string("token", &db_connect())),
    );
    let client = ClientBuilder::new().timeout(None).build().unwrap();
    let args = json!(
        {"jsonrpc": "2.0", "method": name, "params": params, "id": 1}
    );
    let mut api_url = query_settings_string("api_url", &db_connect());
    if api_url.trim().is_empty() {
        api_url = "https://api.btcmap.org/rpc".into();
    }
    let res = client
        .post(api_url)
        .body(serde_json::to_string(&args).unwrap())
        .send();
    match res {
        Ok(res) => {
            if res.status().is_success() {
                let res = res.json::<Map<String, Value>>().unwrap();
                let res = serde_json::to_string_pretty(&res).unwrap();
                println!("{}", res);
            } else {
                handle_unsuccessful_response(res);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

fn handle_unsuccessful_response(res: Response) {
    let status = res.status();
    let mut text = res.text().unwrap_or("empty".into());
    if text.trim().is_empty() {
        text = "empty".into();
    }
    eprintln!("HTTP status code: {}, message: {}", status, text);
}

fn db_path() -> PathBuf {
    let data_dir = data_dir().unwrap().join("mapctl");
    if !data_dir.exists() {
        create_dir(&data_dir).unwrap();
    }
    data_dir.join("mapctl.db")
}

fn db_connect() -> Connection {
    let conn = Connection::open(db_path()).unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (json TEXT NOT NULL);",
        (),
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
    Connection::open(db_path()).unwrap()
}

fn query_settings_string(name: &str, conn: &Connection) -> String {
    let mut stmt = conn
        .prepare(&format!(
            "SELECT json_extract(json, '$.{name}') FROM settings;"
        ))
        .unwrap();
    let mut rows = stmt.query(()).unwrap();
    match rows.next().unwrap() {
        Some(first_row) => first_row.get(0).unwrap_or("".into()),
        None => "".into(),
    }
}
