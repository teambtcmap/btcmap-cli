use dirs::data_dir;
use rusqlite::{params, Connection};
use std::{fs::create_dir, path::PathBuf, process::exit};

pub fn connect() -> Connection {
    let conn = Connection::open(path()).unwrap_or_else(|e| {
        eprintln!("failed to connect to database: {e}");
        exit(1);
    });
    init(&conn);
    conn
}

pub fn insert_settings_string(name: &str, value: &str, conn: &Connection) {
    conn.execute(
        &format!("UPDATE settings SET json = json_set(json, '$.{name}', ?1);"),
        params![value],
    )
    .unwrap_or_else(|e| {
        eprintln!("failed to set {name} to {value}: {e}");
        exit(1);
    });
}

pub fn query_settings_string(name: &str, conn: &Connection) -> String {
    let mut stmt = conn
        .prepare(&format!(
            "SELECT json_extract(json, '$.{name}') FROM settings;"
        ))
        .unwrap_or_else(|_| {
            eprintln!("failed to query {name} from settings");
            exit(1);
        });
    let mut rows = stmt.query(()).unwrap_or_else(|_| {
        eprintln!("failed to query {name} from settings");
        exit(1);
    });
    match rows.next().unwrap_or_else(|_| {
        eprintln!("failed to query {name} from settings");
        exit(1);
    }) {
        Some(first_row) => first_row.get(0).unwrap_or("".into()),
        None => "".into(),
    }
}

fn path() -> PathBuf {
    let data_dir = data_dir().unwrap_or_else(|| {
        eprintln!("failed to locate system app data directory");
        exit(1);
    });
    let data_dir = data_dir.join("btcmap-cli");
    if !data_dir.exists() {
        create_dir(&data_dir).unwrap_or_else(|_| {
            eprintln!("failed to create database directory");
            exit(1);
        });
    }
    data_dir.join("btcmap-cli.db")
}

fn init(conn: &Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (json TEXT NOT NULL);",
        (),
    )
    .unwrap_or_else(|e| {
        eprintln!("failed to create settings table: {e}");
        exit(1);
    });
    let mut stmt = conn
        .prepare("SELECT COUNT (*) FROM settings;")
        .unwrap_or_else(|e| {
            eprintln!("failed to query settings table: {e}");
            exit(1);
        });
    let mut rows = stmt.query(()).unwrap_or_else(|e| {
        eprintln!("failed to query settings table: {e}");
        exit(1);
    });
    let first_row = rows
        .next()
        .unwrap_or_else(|e| {
            eprintln!("failed to query settings table: {e}");
            exit(1);
        })
        .unwrap_or_else(|| {
            eprintln!("COUNT query returned no rows");
            exit(1);
        });
    let rows: i64 = first_row.get(0).unwrap_or_else(|e| {
        eprintln!("failed to query settings table {e}");
        exit(1);
    });
    if rows == 0 {
        conn.execute("INSERT INTO settings (json) VALUES (json('{}'))", ())
            .unwrap_or_else(|e| {
                eprintln!("Failed to initialize settings table: {e}");
                exit(1);
            });
    }
    if rows > 1 {
        eprintln!("settings database has been corrupted");
        exit(1);
    }
}
