use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use crate::validators::Validator;

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
            last_change: Local::now().date_naive(),
            creation_date: Local::now().date_naive(),
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
    pub validator: Option<Validator>,
    pub error: Option<String>
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

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct EmailAddress(pub String);

pub type EmailAddress = String;
pub type Username = String;

//impl From<String> for EmailAddress {
//    fn from(str: String) -> Self {
//        Self(str)
//    }
//}

//impl From<&EmailAddress> for String {
//    fn from(value: &EmailAddress) -> Self {
//        value.0.clone()
//    }
//}

//impl ToSql for EmailAddress {
//    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
//        Ok(ToSqlOutput::from(self.0.as_str()))
//    }
//}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Username(pub String);
// 
// impl From<String> for Username {
//     fn from(str: String) -> Self {
//         Self(str)
//     }
// }
// 
// impl From<&Username> for String {
//     fn from(value: &Username) -> Self {
//         value.0.clone()
//     }
// }
// 
// impl ToSql for Username {
//     fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
//         Ok(ToSqlOutput::from(self.0.as_str()))
//     }
// }
