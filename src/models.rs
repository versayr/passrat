use crate::{
    helpers::{validate_account, validate_security_question, validate_service, validate_shortcut},
    validators::Validator,
};
use chrono::{Local, NaiveDate};
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
    pub last_change: NaiveDate,
    pub creation_date: NaiveDate,
    pub password: Option<String>,
    pub access_token: Option<String>,
    pub pin: Option<u32>,
    pub passcode: Option<u32>,
}

impl Account {
    pub fn new(service_id: u32) -> Self {
        Self {
            id: None,
            service_id,
            contact: ContactInfo::default(),
            last_change: Local::now().date_naive(),
            creation_date: Local::now().date_naive(),
            password: None,
            access_token: None,
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

impl Target {
    pub fn validate(&self) -> Result<(), String> {
        match self {
            Self::Service(service) => validate_service(service),
            Self::Account(account) => validate_account(account),
            Self::Shortcut(shortcut) => validate_shortcut(shortcut),
            Self::SecurityQuestion(security_question) => {
                validate_security_question(security_question)
            }
        }
    }
}

type ApplyFn = fn(&mut Target, &str) -> Result<(), String>;

#[derive(Debug, Clone)]
pub struct Field {
    pub label: String,
    pub value: String,
    pub error: Option<String>,
    pub validator: Option<Validator>,
    pub apply: ApplyFn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContactInfo {
    Email(EmailAddress),
    Username(Username),
    Both(EmailAddress, Username),
}

impl Default for ContactInfo {
    fn default() -> Self {
        Self::Username(String::new())
    }
}

impl ContactInfo {
    pub fn from_options(email: Option<EmailAddress>, username: Option<Username>) -> Self {
        match (email, username) {
            (Some(email), Some(username)) => Self::Both(email, username),
            (Some(email), None) => Self::Email(email),
            (None, Some(username)) => Self::Username(username),
            _ => Self::default(),
        }
    }
}

pub type EmailAddress = String;
pub type Username = String;
