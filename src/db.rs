use std::path::Path;
use rusqlite::{Connection, Error};

pub fn connect_database(path: &Path, password: &str) -> Result<Connection, Error> {
    let conn = Connection::open(path).expect("Vault not found.");
    conn.pragma_update(None, "key", password)?;
    // conn.execute("SELECT COUNT(*) FROM sqlite_master", [])?;
    Ok(conn)
}

pub fn init_database(path: &Path, password: &str) -> Result<Connection, Error> {
    let conn = Connection::open(path).expect("Vault not found.");
    conn.pragma_update(None, "key", password)?;

    // for some reason, this line will cause the db to not be created if it doesn't exist
    // however, without it, entering the wrong password causes a crash if the db does exist
    // also, entering the wrong password causes the correct one to fail??
    // conn.execute("SELECT COUNT(*) FROM sqlite_master", [])?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS services (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            url TEXT
        )",
        [],
    )
    .expect("Failed to create service table.");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS accounts (
            id INTEGER PRIMARY KEY,
            service_id INTEGER NOT NULL,
            username TEXT,
            email TEXT,
            password TEXT, 
            access_token TEXT, 
            last_change TEXT NOT NULL, 
            account_creation_date TEXT NOT NULL, 
            pin INTEGER,
            passcode TEXT
        )",
        [],
    )
    .expect("Failed to create accounts table.");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS security_questions (
            id INTEGER PRIMARY KEY,
            account_id INTEGER NOT NULL,
            question TEXT NOT NULL,
            answer TEXT NOT NULL
        )",
        [],
    )
    .expect("Failed to create security question table.");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS shortcuts (
            id INTEGER PRIMARY KEY,
            account_id INTEGER NOT NULL,
            field TEXT NOT NULL,
            sequence TEXT NOT NULL
        )",
        [],
    )
    .expect("Failed to create security question table.");

    Ok(conn)
}
