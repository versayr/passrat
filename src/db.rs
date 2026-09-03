use jiff::civil::Date;
use rusqlite::{fallible_iterator::FallibleIterator, params, types::Type, Connection, Error, Row};
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
    conn.pragma_update(None, "foreign_keys", true)?;

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
    conn.pragma_update(None, "foreign_keys", true)?;

    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS services (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            url TEXT
        );

        CREATE TABLE IF NOT EXISTS accounts (
            id INTEGER PRIMARY KEY,
            service_id INTEGER NOT NULL,
            username TEXT,
            email TEXT,
            password TEXT, 
            access_token TEXT, 
            last_change TEXT NOT NULL, 
            creation_date TEXT NOT NULL, 
            pin INTEGER,
            passcode INTEGER,

            UNIQUE (service_id, username),

            FOREIGN KEY (service_id)
                REFERENCES services(id)
                ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS security_questions (
            id INTEGER PRIMARY KEY,
            account_id INTEGER NOT NULL,
            question TEXT NOT NULL,
            answer TEXT NOT NULL,

            UNIQUE (account_id, question),

            FOREIGN KEY (account_id)
                REFERENCES accounts(id)
                ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS shortcuts (
            id INTEGER PRIMARY KEY,
            account_id INTEGER NOT NULL,
            field TEXT NOT NULL,
            sequence TEXT NOT NULL UNIQUE,

            UNIQUE (account_id, field),

            FOREIGN KEY (account_id)
                REFERENCES accounts(id)
                ON DELETE CASCADE
        );
        ",
    )?;

    Ok(())
}

impl Service {
    pub fn from_row(row: &Row<'_>) -> Result<Self, Error> {
        let url: Option<String> = row.get("url")?;
        let url = url.filter(|url| !url.is_empty());

        Ok(Self {
            id: row.get("id")?,
            name: row.get("name")?,
            url,
        })
    }
}

impl Account {
    pub fn from_row(row: &Row<'_>) -> Result<Self, Error> {
        let last_change_string: String = row.get("last_change")?;
        let creation_date_string: String = row.get("creation_date")?;
        let last_change = Date::strptime("%Y-%m-%d", &last_change_string)
            .map_err(|error| Error::FromSqlConversionFailure(6, Type::Text, Box::new(error)))?;
        let creation_date = Date::strptime("%Y-%m-%d", &creation_date_string)
            .map_err(|error| Error::FromSqlConversionFailure(7, Type::Text, Box::new(error)))?;

        let email: Option<EmailAddress> = row.get("email")?;
        let username: Option<Username> = row.get("username")?;
        let contact = ContactInfo::from_options(
            email.filter(|email| !email.is_empty()),
            username.filter(|username| !username.is_empty()),
        );

        let password: Option<String> = row.get("password")?;
        let access_token: Option<String> = row.get("access_token")?;

        Ok(Self {
            id: row.get("id")?,
            service_id: row.get("service_id")?,
            contact,
            password: password.filter(|password| !password.is_empty()),
            access_token: access_token.filter(|access_token| !access_token.is_empty()),
            last_change,
            creation_date,
            pin: row.get("pin")?,
            passcode: row.get("passcode")?,
        })
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
            .query_map([], Service::from_row)?
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
            .map(Account::from_row)
            .collect()
    }

    pub fn handle_target(&mut self, target: &Target) -> Result<(), Error> {
        match &target {
            Target::Service(service) => {
                if service.id.is_some() {
                    self.update_service(service)?;
                } else {
                    self.add_service(service)?;
                }
                let list = self.get_services()?;
                self.mode = Mode::Home(Home::new(list));
                Ok(())
            }
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
            .query_one([], Service::from_row)
    }

    pub fn add_service(&mut self, service: &Service) -> Result<(), Error> {
        let conn = self
            .conn
            .as_mut()
            .expect("Failed to get database connection.");

        conn.execute(
            "INSERT INTO services (name, url) VALUES (?1, ?2)",
            params![service.name, service.url],
        )?;

        Ok(())
    }

    fn update_service(&mut self, service: &Service) -> Result<(), Error> {
        let conn = self
            .conn
            .as_mut()
            .expect("Failed to get database connection.");

        conn.execute(
            "UPDATE services SET name = ?1, url = ?2 WHERE id = ?3",
            params![service.name, service.url, service.id],
        )?;

        Ok(())
    }

    pub fn remove_service(&mut self, service: &Service) -> Result<(), Error> {
        let conn = self
            .conn
            .as_mut()
            .expect("Failed to get database connection.");

        conn.execute("DELETE FROM services WHERE id = ?1", [&service.id])?;

        Ok(())
    }

    pub fn add_account(&mut self, account: &Account) -> Result<(), Error> {
        let conn = self.conn.as_mut().expect("Failed to get connection.");
        let contact = &account.contact;
        let (username, email) = match contact {
            ContactInfo::Both(email, username) => (username, email),
            ContactInfo::Email(email) => (&String::new(), email),
            ContactInfo::Username(username) => (username, &String::new()),
        };

        conn.execute(
            "
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
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                account.service_id,
                username,
                account.last_change.to_string(),
                account.creation_date.to_string(),
                email,
                account.password,
                account.access_token,
                account.pin,
                account.passcode
            ],
        )?;

        Ok(())
    }

    fn update_account(&mut self, account: &Account) -> Result<(), Error> {
        let conn = self.conn.as_mut().expect("Failed to get connection.");
        let contact = &account.contact;
        let (username, email) = match contact {
            ContactInfo::Both(email, username) => (username, email),
            ContactInfo::Email(email) => (&String::new(), email),
            ContactInfo::Username(username) => (username, &String::new()),
        };

        conn.execute(
            "
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
                account.last_change.to_string(),
                account.creation_date.to_string(),
                email,
                account.password,
                account.access_token,
                account.pin,
                account.passcode,
                account.id
            ],
        )?;

        Ok(())
    }

    pub fn remove_account(&mut self, account: &Account) -> Result<(), Error> {
        let conn = self.conn.as_mut().expect("Failed to get connection.");

        conn.execute("DELETE FROM accounts WHERE id = ?1", [account.id])?;
        Ok(())
    }

    pub fn add_shortcut(&mut self, shortcut: &Shortcut) -> Result<(), Error> {
        let conn = self.conn.as_mut().expect("Failed to get connection.");

        conn.execute(
            "INSERT INTO shortcuts (account_id, field, sequence) VALUES (?1, ?2, ?3)",
            params![shortcut.account_id, shortcut.field, shortcut.sequence],
        )?;

        Ok(())
    }

    fn update_shortcut(&mut self, shortcut: &Shortcut) -> Result<(), Error> {
        let conn = self.conn.as_mut().expect("Failed to get connection.");

        conn.execute(
            "UPDATE shortcuts SET sequence = ?1 WHERE id = ?3",
            params![shortcut.sequence, shortcut.id],
        )?;

        Ok(())
    }
    //
    //     fn remove_shortcut(&mut self) -> Result<(), Error> {
    //
    //     }

    pub fn add_security_question(
        &mut self,
        security_question: &SecurityQuestion,
    ) -> Result<(), Error> {
        let conn = self.conn.as_mut().expect("Failed to get connection.");

        conn.execute(
            "INSERT INTO security_questions (question, answer) VALUES (?1, ?2)",
            params![security_question.question, security_question.answer],
        )?;

        Ok(())
    }

    fn update_security_question(
        &mut self,
        security_question: &SecurityQuestion,
    ) -> Result<(), Error> {
        let conn = self.conn.as_mut().expect("Failed to get connection.");

        conn.execute(
            "UPDATE security_questions SET sequence = ?1 WHERE id = ?3",
            params![
                security_question.question,
                security_question.answer,
                security_question.id
            ],
        )?;

        Ok(())
    }
    //
    //     fn remove_security_question(&mut self) -> Result<(), Error> {
    //
    //     }
}
