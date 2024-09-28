use dirs::data_dir;
use reqwest::blocking::ClientBuilder;
use reqwest::blocking::Response;
use rusqlite::params;
use rusqlite::Connection;
use serde_json::{json, Map, Value};
use std::path::PathBuf;
use std::{env, fs::create_dir};

fn main() {
    let conn = Connection::open(db_path()).unwrap();
    init_db(&conn);
    let token = get_token(&conn);
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
        "get-element" => {
            let id = args[2].clone().replace("=", ":");
            call_remote_procedure("getelement", json!({"token":token,"id":id}));
        }
        "boost-element" => {
            let id = args[2].clone().replace("=", ":");
            let days: i64 = args[3].parse().unwrap();
            call_remote_procedure("boostelement", json!({"token":token,"id":id,"days":days}));
        }
        "generate-reports" => {
            call_remote_procedure("generatereports", json!({"token":token}));
        }
        "generate-element-icons" => {
            let from_element_id: i64 = args[2].clone().parse().unwrap();
            let to_element_id: i64 = args[3].clone().parse().unwrap();
            call_remote_procedure(
                "generateelementicons",
                json!({"token":token,"from_element_id":from_element_id,"to_element_id":to_element_id}),
            );
        }
        "generate-element-categories" => {
            let from_element_id: i64 = args[2].clone().parse().unwrap();
            let to_element_id: i64 = args[3].clone().parse().unwrap();
            call_remote_procedure(
                "generateelementcategories",
                json!({"token":token,"from_element_id":from_element_id,"to_element_id":to_element_id}),
            );
        }
        "add-element-comment" => {
            let id = args[2].clone().replace("=", ":");
            let comment = args[3].clone();
            call_remote_procedure(
                "addelementcomment",
                json!({"token":token,"id":id,"comment":comment}),
            );
        }
        "get-area" => {
            let id = args[2].clone();
            call_remote_procedure("getarea", json!({"token":token,"id":id}));
        }
        "set-area-tag" => {
            let id = args[2].clone();
            let name = args[3].clone();
            let value = args[4].clone();
            call_remote_procedure(
                "setareatag",
                json!({"token":token,"id":id,"name":name,"value":value}),
            );
        }
        "remove-area-tag" => {
            let id = args[2].clone();
            let tag = args[3].clone();
            call_remote_procedure("removeareatag", json!({"token":token,"id":id,"tag":tag}));
        }
        "get-trending-countries" => {
            let period_start = args[2].clone();
            let period_end = args[3].clone();
            call_remote_procedure(
                "gettrendingcountries",
                json!({"token":token,"period_start":period_start,"period_end":period_end}),
            );
        }
        "get-trending-communities" => {
            let period_start = args[2].clone();
            let period_end = args[3].clone();
            call_remote_procedure(
                "gettrendingcommunities",
                json!({"token":token,"period_start":period_start,"period_end":period_end}),
            );
        }
        "generate-element-issues" => {
            call_remote_procedure("generateelementissues", json!({"token":token}));
        }
        "sync-elements" => {
            call_remote_procedure("syncelements", json!({"token":token}));
        }
        "get-most-commented-countries" => {
            let period_start = args[2].clone();
            let period_end = args[3].clone();
            call_remote_procedure(
                "getmostcommentedcountries",
                json!({"token":token,"period_start":period_start,"period_end":period_end}),
            );
        }
        "generate-areas-elements-mapping" => {
            let from_element_id: i64 = args[2].clone().parse().unwrap();
            let to_element_id: i64 = args[3].clone().parse().unwrap();
            call_remote_procedure(
                "getmostcommentedcountries",
                json!({"token":token,"from_element_id":from_element_id,"to_element_id":to_element_id}),
            );
        }
        _ => {}
    }
}

fn call_remote_procedure(name: &str, params: Value) {
    let client = ClientBuilder::new().timeout(None).build().unwrap();
    let args = json!(
        {"jsonrpc": "2.0", "method": name, "params": params, "id": 1}
    );
    let api_url = "https://api.btcmap.org/rpc";
    //let api_url = "http://127.0.0.1:8000/rpc";
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

fn init_db(conn: &Connection) {
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
}

fn get_token(conn: &Connection) -> String {
    let mut stmt = conn
        .prepare("SELECT json_extract(json, '$.token') FROM settings;")
        .unwrap();
    let mut rows = stmt.query(()).unwrap();
    match rows.next().unwrap() {
        Some(first_row) => first_row.get(0).unwrap_or("".into()),
        None => "".into(),
    }
}
