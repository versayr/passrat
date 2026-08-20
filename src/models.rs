use chrono::{Local, NaiveDate};
use rusqlite::{Result, ToSql, types::ToSqlOutput};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Service {
    pub id: Option<u32>,
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Option<u32>,
    pub service_id: u32,
    pub contact: ContactInfo,
    // pub username: Option<String>,
    pub last_change: NaiveDate,
    pub creation_date: NaiveDate,
    // pub email: String,
    pub password: String,
    pub access_token: String,
    pub pin: Option<u32>,
    pub passcode: Option<u32>,
}

impl Account {
    pub fn new(service_id: u32) -> Self {
        Self {
            id: None,
            service_id,
            contact: ContactInfo::default(),
            // username: None,
            last_change: Local::now().date_naive(),
            creation_date: Local::now().date_naive(),
            // email: String::new(),
            password: String::new(),
            access_token: String::new(),
            pin: None,
            passcode: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct SecurityQuestion {
    pub id: Option<u32>,
    pub account_id: u32,
    pub question: String,
    pub answer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Shortcut {
    pub id: Option<u32>,
    pub account_id: u32,
    pub field: String,
    pub sequence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Target {
    Service(Service),
    Account(Account),
    SecurityQuestion(SecurityQuestion),
    Shortcut(Shortcut),
}

#[derive(Debug, Clone)]
pub struct Field {
    pub label: String,
    pub value: String,
    pub validator: Option<Validator>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Validator {
    Email, 
    Url, 
    Date, 
    Numeric, 
    NonEmpty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContactInfo {
    Email(EmailAddress),
    Username(Username),
    Both(EmailAddress, Username),
}

impl Default for ContactInfo {
    fn default() -> Self {
        Self::Username(Username(String::new()))
    }
}

impl ContactInfo {
    pub fn from_options(email: Option<String>, username: Option<String>) -> Self {
        match (email, username) {
            (Some(email), Some(username)) => Self::Both(EmailAddress(email), Username(username)),
            (Some(email), None) => Self::Email(EmailAddress(email)),
            (None, Some(username)) => Self::Username(Username(username)),
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAddress(pub String);

impl From<String> for EmailAddress {
    fn from(str: String) -> Self {
        Self(str)
    }
}

impl From<&EmailAddress> for String {
    fn from(value: &EmailAddress) -> Self {
        value.0.clone()
    }
}

impl ToSql for EmailAddress {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.0.as_str()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Username(pub String);

impl From<String> for Username {
    fn from(str: String) -> Self {
        Self(str)
    }
}

impl From<&Username> for String {
    fn from(value: &Username) -> Self {
        value.0.clone()
    }
}

impl ToSql for Username {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.0.as_str()))
    }
}
