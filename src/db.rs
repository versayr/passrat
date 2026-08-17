use chrono::NaiveDate;
use rusqlite::{Connection, Error, Row, fallible_iterator::FallibleIterator, params};
use std::path::Path;
use xdg::BaseDirectories;

use crate::{
    app::{App, Mode},
    models::{
        Account, ContactInfo, EmailAddress, SecurityQuestion, Service, Shortcut, Target, Username,
    },
    modes::{home::Home, lock::Lock},
};

pub fn connect_database(path: &Path, password: &str) -> Result<Connection, Error> {
    let conn = Connection::open(path)?;
    conn.pragma_update(None, "key", password)?;

    conn.query_row("SELECT COUNT(*) FROM services", [], |r| {
        r.get::<usize, i64>(0)
    })?;

    Ok(conn)
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
            name TEXT NOT NULL UNIQUE,
            url TEXT
        )",
        [],
    )
    .expect("Failed to create service table.");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS accounts (
            id INTEGER PRIMARY KEY,
            service_id INTEGER NOT NULL,
            username TEXT UNIQUE,
            email TEXT,
            password TEXT, 
            access_token TEXT, 
            last_change TEXT NOT NULL, 
            creation_date TEXT NOT NULL, 
            pin INTEGER,
            passcode INTEGER
        )",
        [],
    )
    .expect("Failed to create accounts table.");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS security_questions (
            id INTEGER PRIMARY KEY,
            account_id INTEGER NOT NULL,
            question TEXT NOT NULL UNIQUE,
            answer TEXT NOT NULL
        )",
        [],
    )
    .expect("Failed to create security question table.");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS shortcuts (
            id INTEGER PRIMARY KEY,
            account_id INTEGER NOT NULL,
            field TEXT NOT NULL UNIQUE,
            sequence TEXT NOT NULL UNIQUE
        )",
        [],
    )
    .expect("Failed to create security question table.");

    Ok(())
}

impl Service {
    pub fn from_row(row: &Row<'_>) -> Self {
        let url: Option<String> = row.get("url").expect("Failed to get url.");
        let url = url.and_then(|s| (!s.is_empty()).then_some(s));

        Self {
            id: row.get("id").expect("Failed to get row id."),
            name: row.get("name").expect("Failed to get row name."),
            url,
        }
    }
}

impl Account {
    pub fn from_row(row: &Row<'_>) -> Self {
        let last_change_string: String = row
            .get("last_change")
            .expect("Failed to get last change date.");
        let creation_date_string: String = row
            .get("creation_date")
            .expect("Failed to get last change date.");
        let email: Option<String> = row.get("email").expect("Failed to get email.");
        let username: Option<String> = row.get("username").expect("Failed to get username.");

        let last_change = NaiveDate::parse_from_str(&last_change_string, "%Y-%m-%d")
            .expect("Failed to parse last change date (expected YYYY-MM-DD).");
        let creation_date = NaiveDate::parse_from_str(&creation_date_string, "%Y-%m-%d")
            .expect("Failed to parse account creation date (expected YYYY-MM-DD).");
        let email = email.and_then(|s| (!s.is_empty()).then_some(s));
        let username = username.and_then(|s| (!s.is_empty()).then_some(s));
        let contact = ContactInfo::from_options(email, username);

        Self {
            id: row.get("id").expect("Failed to get row id."),
            service_id: row.get("service_id").expect("Failed to get service id."),
            contact,
            password: row.get("password").expect("Failed to get password."),
            access_token: row
                .get("access_token")
                .expect("Failed to get access token."),
            last_change,
            creation_date,
            pin: row.get_unwrap("pin"),
            passcode: row.get_unwrap("passcode"),
        }
    }
}

impl App {
    pub fn submit_password(&mut self, password: &str) {
        let path: BaseDirectories = BaseDirectories::with_prefix("passrat");
        path.create_data_directory("")
            .expect("Failed to create data directory.");

        if let Some(path) = path.find_data_file("vault.db") {
            if let Ok(conn) = connect_database(&path, password) {
                self.conn = Some(conn);
                if let Ok(list) = self.get_services() {
                    self.mode = Mode::Home(Home::new(list));
                } else {
                    self.mode = Mode::Lock(Lock::new(
                        String::new(),
                        "Login error - please try again".into(),
                    ));
                }
            } else {
                self.mode = Mode::Lock(Lock::new(
                    String::new(),
                    "Incorrect password - please try again.".into(),
                ));
                // TODO fix this hacky bs
                self.should_clear = true;
            }
        } else {
            let _ = init_database(password);
            self.mode = Mode::Lock(Lock::new(
                String::new(),
                "Database created - please enter passphrase again.".into(),
            ));
        }
    }

    pub fn get_services(&mut self) -> Result<Vec<Service>, Error> {
        let mut stmt = self
            .conn
            .as_mut()
            .expect("Failed to connect to database.")
            .prepare("SELECT id, name, url FROM services ORDER BY name")?;

        let result = stmt
            .query_map([], |row| {
                Ok(Service {
                    id: row.get(0).expect("Failed to get service id."),
                    name: row.get(1).expect("Failed to get service name."),
                    url: row.get(2).expect("Failed to get service url."),
                })
            })?
            .collect::<Result<Vec<Service>, _>>()?;

        Ok(result)
    }

    pub fn get_accounts(&mut self, service_id: u32) -> Result<Vec<Account>, Error> {
        self.conn
            .as_mut()
            .expect("Failed to connect to database.")
            .prepare(&format!(
                "SELECT * FROM accounts WHERE service_id = {service_id} ORDER BY username"
            ))
            .expect("Failed to prepare statement.")
            .query([])?
            .map(|row| Ok(Account::from_row(row)))
            .collect()
    }

    pub fn handle_target(&mut self, target: &Target) {
        match &target {
            Target::Service(service) => match service.id {
                Some(_) => {
                    self.update_service(service);
                    let list = self
                        .get_services()
                        .expect("Failed to refresh service list.");
                    self.mode = Mode::Home(Home::new(list));
                }
                None => self.add_service(service),
            },
            Target::Account(account) => match account.id {
                Some(_) => self.update_account(account),
                None => self.add_account(account),
            },
            Target::Shortcut(shortcut) => match shortcut.id {
                Some(_) => self.update_shortcut(shortcut),
                None => self.add_shortcut(shortcut),
            },
            Target::SecurityQuestion(sq) => match sq.id {
                Some(_) => self.update_security_question(sq),
                None => self.add_security_question(sq),
            },
        }
    }

    pub fn get_service(&mut self, id: u32) -> Result<Service, Error> {
        self.conn
            .as_mut()
            .expect("Failed to connect to database.")
            .prepare(&format!("SELECT * FROM services WHERE id = {id} LIMIT 1"))
            .expect("Failed to prepare statement.")
            .query_one([], |row| Ok(Service::from_row(row)))
    }

    pub fn add_service(&mut self, service: &Service) {
        let conn = self
            .conn
            .as_mut()
            .expect("Failed to get database connection.");

        let _ = conn.execute(
            "INSERT INTO services (name, url) VALUES (?1, ?2)",
            params![service.name, service.url],
        );

        self.get_services()
            .expect("Failed to refresh service list.");
    }

    fn update_service(&mut self, service: &Service) {
        let conn = self
            .conn
            .as_mut()
            .expect("Failed to get database connection.");

        let _ = conn.execute(
            "UPDATE services SET name = ?1, url = ?2 WHERE id = ?3",
            params![service.name, service.url, service.id],
        );
    }

    pub fn remove_service(&mut self, service: &Service) {
        let conn = self
            .conn
            .as_mut()
            .expect("Failed to get database connection.");

        let tx = conn
            .transaction()
            .expect("Failed to initialize database transaction.");

        tx.execute("DELETE FROM accounts WHERE service_id = ?1", [&service.id])
            .expect("Failed to delete accounts of this service.");
        tx.execute("DELETE FROM services WHERE id = ?1", [&service.id])
            .expect("Failed to delete this service.");

        tx.commit().expect("Failed to commit database transaction.");
    }

    pub fn add_account(&mut self, account: &Account) {
        let conn = self.conn.as_mut().expect("Failed to get connection.");
        let contact = &account.contact;
        let (username, email) = match contact {
            ContactInfo::Both(email, username) => (username, email),
            ContactInfo::Email(email) => (&Username(String::new()), email),
            ContactInfo::Username(username) => (username, &EmailAddress(String::new())),
        };

        let _ = conn.execute(
            r"
            INSERT INTO accounts (
                service_id,
                username,
                last_change,
                creation_date,
                email,
                password,
                access_token,
                pin,
                passcode)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            ",
            params![
                account.service_id,
                username,
                account.last_change.format("%Y-%m-%d").to_string(),
                account.creation_date.format("%Y-%m-%d").to_string(),
                email,
                account.password,
                account.access_token,
                account.pin,
                account.passcode
            ],
        );
    }

    fn update_account(&mut self, account: &Account) {
        let conn = self.conn.as_mut().expect("Failed to get connection.");
        let contact = &account.contact;
        let (username, email) = match contact {
            ContactInfo::Both(email, username) => (username, email),
            ContactInfo::Email(email) => (&Username(String::new()), email),
            ContactInfo::Username(username) => (username, &EmailAddress(String::new())),
        };

        let _ = conn.execute(
            r"
            UPDATE accounts
            SET
                service_id = ?1,
                username = ?2,
                last_change = ?3,
                creation_date = ?4,
                email = ?5,
                password = ?6,
                access_token = ?7,
                pin = ?8,
                passcode = ?9
            WHERE id = ?10
            ",
            params![
                account.service_id,
                username,
                account.last_change.format("%Y-%m-%d").to_string(),
                account.creation_date.format("%Y-%m-%d").to_string(),
                email,
                account.password,
                account.access_token,
                account.pin,
                account.passcode,
                account.id
            ],
        );
    }

    pub fn remove_account(&mut self, account: &Account) {
        let conn = self.conn.as_mut().expect("Failed to get connection.");

        let _ = conn.execute("DELETE FROM accounts WHERE id = ?1", [account.id]);
    }

    pub fn add_shortcut(&mut self, shortcut: &Shortcut) {
        let conn = self.conn.as_mut().expect("Failed to get connection.");

        let _ = conn.execute(
            "INSERT INTO shortcuts (account_id, field, sequence) VALUES (?1, ?2, ?3)",
            params![shortcut.account_id, shortcut.field, shortcut.sequence],
        );

        //         self.get_shortcuts()
        //             .expect("Failed to refresh shortcut list.");
    }

    fn update_shortcut(&mut self, shortcut: &Shortcut) {
        let conn = self.conn.as_mut().expect("Failed to get connection.");

        let _ = conn.execute(
            "UPDATE shortcuts SET sequence = ?1 WHERE id = ?3",
            params![shortcut.sequence, shortcut.id],
        );

        //         self.get_shortcuts()
        //             .expect("Failed to refresh shortcut list.");
    }
    //
    //     fn remove_shortcut(&mut self) -> Result<(), Error> {
    //
    //     }

    pub fn add_security_question(&mut self, security_question: &SecurityQuestion) {
        let conn = self.conn.as_mut().expect("Failed to get connection.");

        let _ = conn.execute(
            "INSERT INTO security_questions (question, answer) VALUES (?1, ?2)",
            params![security_question.question, security_question.answer],
        );

        //         self.get_security_questions()
        //             .expect("Failed to refresh security_question list.");
    }

    fn update_security_question(&mut self, security_question: &SecurityQuestion) {
        let conn = self.conn.as_mut().expect("Failed to get connection.");

        let _ = conn.execute(
            "UPDATE security_questions SET sequence = ?1 WHERE id = ?3",
            params![
                security_question.question,
                security_question.answer,
                security_question.id
            ],
        );

        //         self.get_security_questions()
        //             .expect("Failed to refresh security_question list.");
    }
    //
    //     fn remove_security_question(&mut self) -> Result<(), Error> {
    //
    //     }
}
