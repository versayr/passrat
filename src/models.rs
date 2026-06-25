use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub id: Option<u16>,
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Option<u16>,
    pub service_id: u16,
    pub username: String,
    pub last_change: String,
    pub account_creation_date: String,
    pub email: Option<String>,
    pub password: Option<String>,
    pub access_token: Option<String>,
    pub pin: Option<String>,
    pub passcode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct SecurityQuestion {
    pub id: Option<u16>,
    pub account_id: u16,
    pub question: String,
    pub answer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Shortcut {
    pub id: Option<u16>,
    pub account_id: u16,
    pub field: String,
    pub sequence: String,
}

impl Default for Service {
    fn default() -> Self {
        Self {
            id: None,
            name: "".into(),
            url: Some("".into()),
        }
    }
}

impl Default for Account {
    fn default() -> Self {
        Self {
            id: None,
            service_id: 1,
            username: "Bruce".into(),
            last_change: "".into(),
            account_creation_date: "".into(),
            email: None,
            password: None,
            access_token: None,
            pin: None,
            passcode: None,
        }
    }
}
