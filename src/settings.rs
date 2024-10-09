use crate::Result;
use dirs::data_dir;
use rusqlite::{params, Connection};
use std::{fs::create_dir, path::PathBuf};

pub fn put_str(name: &str, value: &str) -> Result<()> {
    connect()?.execute(
        &format!("UPDATE settings SET json = json_set(json, '$.{name}', ?1);"),
        params![value],
    )?;
    Ok(())
}

pub fn get_str(name: &str) -> Result<String> {
    let conn = connect()?;
    let mut stmt = conn.prepare(&format!(
        "SELECT json_extract(json, '$.{name}') FROM settings;"
    ))?;
    let mut rows = stmt.query(())?;
    let res = match rows.next()? {
        Some(first_row) => first_row.get(0).unwrap_or("".into()),
        None => "".into(),
    };
    Ok(res)
}

fn connect() -> Result<Connection> {
    let conn = Connection::open(path()?)?;
    init(&conn)?;
    Ok(conn)
}

fn path() -> Result<PathBuf> {
    let data_dir = data_dir().ok_or("failed to locate system app data directory")?;
    let data_dir = data_dir.join("btcmap-cli");
    if !data_dir.exists() {
        create_dir(&data_dir)?;
    }
    Ok(data_dir.join("btcmap-cli.db"))
}

fn init(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (json TEXT NOT NULL);",
        (),
    )?;
    let mut stmt = conn.prepare("SELECT COUNT (*) FROM settings;")?;
    let mut rows = stmt.query(())?;
    let first_row = rows.next()?.ok_or("COUNT query returned no rows")?;
    let rows: i64 = first_row.get(0)?;
    if rows == 0 {
        conn.execute("INSERT INTO settings (json) VALUES (json('{}'))", ())?;
    }
    if rows > 1 {
        Err("settings database has been corrupted")?;
    }
    Ok(())
}
