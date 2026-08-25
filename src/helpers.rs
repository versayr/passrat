use chrono::{Datelike, NaiveDate};
use ordinal::ToOrdinal;
use ratatui::{text::Line, widgets::List};

use crate::models::{Account, ContactInfo};

pub fn format_current_date(date: NaiveDate) -> String {
    format!(
        "{}, {} {}, {}",
        date.format("%A"),
        date.format("%B"),
        date.day().to_ordinal_string(),
        date.format("%Y")
    )
}

pub fn construct_detail_field(label: &str, value: &str, width: usize) -> Line<'static> {
    Line::from(vec![
        format!("{label: <width$}").into(),
        value.to_string().into(),
    ])
}

pub fn construct_detail_list(account: &Account) -> List<'_> {
    let mut lines = vec![];

    let (email, username) = match &account.contact {
        ContactInfo::Both(email, name) => (Some(email), Some(name)),
        ContactInfo::Email(email) => (Some(email), None),
        ContactInfo::Username(name) => (None, Some(name)),
    };

    if let Some(username) = username {
        lines.push(construct_detail_field(
            "Username",
            &String::from(username),
            17,
        ));
    }

    if let Some(email) = email {
        lines.push(construct_detail_field("Email", &String::from(email), 17));
    }

    if !account.password.is_empty() {
        lines.push(construct_detail_field("Password", "{*}", 17));
    }

    if !account.access_token.is_empty() {
        lines.push(construct_detail_field(
            "Access Token",
            &account.access_token,
            17,
        ));
    }

    if let Some(pin) = account.pin {
        lines.push(construct_detail_field("PIN", &pin.to_string(), 17));
    }

    if let Some(passcode) = account.passcode {
        lines.push(construct_detail_field(
            "Passcode",
            &passcode.to_string(),
            17,
        ));
    }

    lines.push(construct_detail_field(
        "Last Change",
        &format_current_date(account.last_change),
        17,
    ));
    lines.push(construct_detail_field(
        "Account Created",
        &format_current_date(account.creation_date),
        17,
    ));

    List::new(lines)
}

// pub fn apply_fields
