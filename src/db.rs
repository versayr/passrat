use rusqlite::{Connection, Error, Row};
use std::path::Path;
use xdg::BaseDirectories;

use crate::models::Account;

pub fn connect_database(path: &Path, password: &str) -> Result<Connection, Error> {
    let conn = Connection::open(path).expect("Vault not found.");
    conn.pragma_update(None, "key", password)?;
    match conn.query_row("SELECT COUNT(*) FROM services", [], |r| {
        r.get::<usize, i64>(0)
    }) {
        Ok(_) => Ok(conn),
        Err(e) => Err(e),
    }
}

pub fn init_database(password: &str) -> Result<(), Error> {
    let path = BaseDirectories::with_prefix("passrat")
        .get_data_file("vault.db")
        .expect("Failed to get db.");
    let conn = Connection::open(path)?;
    conn.pragma_update(None, "key", password)?;

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
            creation_date TEXT NOT NULL, 
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

    Ok(())
}

impl Account {
    pub fn from_row(row: &Row<'_>) -> Account {
        Account {
            id: row.get("id").expect("Failed to get row id."),
            service_id: row.get("service_id").expect("Failed to get service id."),
            username: row.get("username").expect("Failed to get username."),
            email: row.get("email").expect("Failed to get email."),
            password: row.get("password").expect("Failed to get password."),
            access_token: row
                .get("access_token")
                .expect("Failed to get access token."),
            last_change: row.get("last_change").expect("Failed to get last change."),
            creation_date: row
                .get("creation_date")
                .expect("Failed to get account creation date."),
//             pin: row.get("pin").expect("Failed to get pin."),
//             passcode: row.get("passcode").expect("Failed to get passcode."),
            pin: row.get_unwrap("pin"),
            passcode: row.get_unwrap("passcode")
        }
    }
}
