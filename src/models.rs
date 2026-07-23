use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub id: Option<u32>,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Option<u32>,
    pub service_id: u32,
    pub username: String,
    pub last_change: NaiveDate,
    pub creation_date: NaiveDate,
    pub email: String,
    pub password: String,
    pub access_token: String,
    pub pin: Option<u32>,
    pub passcode: Option<u32>,
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
#[allow(dead_code)]
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
}

impl Default for Service {
    fn default() -> Self {
        Self {
            id: None,
            name: "Test".into(),
            url: "https://www.test.org".into(),
        }
    }
}

impl Default for Account {
    fn default() -> Self {
        Self {
            id: None,
            service_id: 1,
            username: String::new(),
            last_change: Local::now().date_naive(),
            creation_date: Local::now().date_naive(),
            email: String::new(),
            password: String::new(),
            access_token: String::new(),
            pin: None,
            passcode: None,
        }
    }
}
