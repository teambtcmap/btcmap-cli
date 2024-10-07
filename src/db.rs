use crate::Result;
use dirs::data_dir;
use rusqlite::{params, Connection};
use std::{fs::create_dir, path::PathBuf};

pub fn connect() -> Result<Connection> {
    let conn = Connection::open(path()?)?;
    init(&conn)?;
    Ok(conn)
}

pub fn insert_settings_string(name: &str, value: &str, conn: &Connection) -> Result<()> {
    conn.execute(
        &format!("UPDATE settings SET json = json_set(json, '$.{name}', ?1);"),
        params![value],
    )
    .map_err(|e| {
        eprintln!("failed to set {name} to {value}: {e}");
        e
    })?;
    Ok(())
}

pub fn query_settings_string(name: &str, conn: &Connection) -> Result<String> {
    let mut stmt = conn
        .prepare(&format!(
            "SELECT json_extract(json, '$.{name}') FROM settings;"
        ))
        .map_err(|e| {
            eprintln!("failed to query {name} from settings");
            e
        })?;
    let mut rows = stmt.query(()).map_err(|e| {
        eprintln!("failed to query {name} from settings");
        e
    })?;
    Ok(
        match rows.next().map_err(|e| {
            eprintln!("failed to query {name} from settings");
            e
        })? {
            Some(first_row) => first_row.get(0).unwrap_or("".into()),
            None => "".into(),
        },
    )
}

fn path() -> Result<PathBuf> {
    let data_dir = data_dir().ok_or("failed to locate system app data directory")?;
    let data_dir = data_dir.join("btcmap-cli");
    if !data_dir.exists() {
        create_dir(&data_dir).map_err(|e| {
            eprintln!("failed to create database directory");
            e
        })?;
    }
    Ok(data_dir.join("btcmap-cli.db"))
}

fn init(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (json TEXT NOT NULL);",
        (),
    )
    .map_err(|e| {
        eprintln!("failed to create settings table: {e}");
        e
    })?;
    let mut stmt = conn
        .prepare("SELECT COUNT (*) FROM settings;")
        .map_err(|e| {
            eprintln!("failed to query settings table: {e}");
            e
        })?;
    let mut rows = stmt.query(()).map_err(|e| {
        eprintln!("failed to query settings table: {e}");
        e
    })?;
    let first_row = rows
        .next()
        .map_err(|e| {
            eprintln!("failed to query settings table: {e}");
            e
        })?
        .ok_or("COUNT query returned no rows")?;
    let rows: i64 = first_row.get(0).map_err(|e| {
        eprintln!("failed to query settings table {e}");
        e
    })?;
    if rows == 0 {
        conn.execute("INSERT INTO settings (json) VALUES (json('{}'))", ())
            .map_err(|e| {
                eprintln!("Failed to initialize settings table: {e}");
                e
            })?;
    }
    if rows > 1 {
        Err("settings database has been corrupted")?;
    }
    Ok(())
}
