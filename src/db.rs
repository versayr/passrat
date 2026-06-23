use crate::models::*;
use rusqlite::{Connection, Error};

pub fn init_databse(password: String) -> Result<Connection, Error> {
    let conn = Connection::open("vault.db").expect("Vault not found.");
    let _ = conn.execute(&format!("PRAGMA key = '{}';", password), [],);

    conn.execute(
        "CREATE TABLE IF NOT EXISTS services (
            id INTEGER PRIMARY KEY,
            service TEXT NOT NULL,
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

pub fn get_services(conn: &Connection) -> Result<Vec<Service>, Error> {
    let mut stmt = conn
        .prepare("SELECT id, service, url FROM services ORDER BY service")
        .expect("Failed to prepare statement");

    let result = stmt.query_map([], |row| {
        Ok(Service {
            id: row.get(0).expect("Failed to get service id."),
            name: row.get(1).expect("Failed to get service name."),
            url: row.get(2).expect("Failed to get service url."),
        })
    })?;

    let mut services: Vec<Service> = vec![];

    for service in result.into_iter() {
        services.push(service?);
    }

    Ok(services)
}
