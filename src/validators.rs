use chrono::NaiveDate;
use email_address_parser::EmailAddress as ParsedEmailAddress;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Validator {
    Email,
    Url,
    Date,
    Numeric,
    NonEmpty,
}

impl Validator {
    pub fn validate(&self, value: &str) -> Result<(), String> {
        match self {
            Self::Email => validate_email(value),
            Self::Url => validate_url(value),
            Self::Date => validate_date(value),
            Self::Numeric => validate_numeric(value),
            Self::NonEmpty => validate_non_empty(value),
        }
    }
}

fn validate_email(s: &str) -> Result<(), String> {
    ParsedEmailAddress::parse(s.trim(), None)
        .map(|_| ())
        .ok_or_else(|| "Enter a valid email address.".to_string())
}

fn validate_url(s: &str) -> Result<(), String> {
    let parsed = Url::parse(s.trim()).map_err(|_| "Enter a valid URL.".to_string())?;

    match parsed.scheme() {
        "https" => Ok(()),
        _ => Err("Please use https for the URL.".to_string()),
    }
}

fn validate_date(s: &str) -> Result<(), String> {
    NaiveDate::parse_from_str(s.trim(), "%Y-%m-%d")
        .map(|_| ())
        .map_err(|_| "Incorrect date format - use YYYY-MM-DD.".to_string())
}

fn validate_numeric(s: &str) -> Result<(), String> {
    if s.is_empty() {
        return Ok(());
    }

    s.trim()
        .parse::<u32>()
        .map(|_| ())
        .map_err(|_| "Enter a valid string of digits.".to_string())
}

fn validate_non_empty(s: &str) -> Result<(), String> {
    if s.trim().is_empty() {
        Err("This value must not be empty.".to_string())
    } else {
        Ok(())
    }
}
