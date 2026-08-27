use std::error::Error;

use chrono::{Datelike, NaiveDate};
use ordinal::ToOrdinal;
use ratatui::{
    style::Stylize,
    text::{Line, Span},
    widgets::{List, ListItem},
};
use rustpass::{PassphraseConfig, PassphraseGenerator};

use crate::models::{Account, ContactInfo, Field, SecurityQuestion, Service, Shortcut};

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

pub fn gen_password(config: PassphraseConfig) -> Result<String, Box<dyn Error>> {
    let generator = PassphraseGenerator::with_default_wordlist(config)?;
    let password = generator.generate()?;
    Ok(password)
}

pub fn construct_field_list<'a>(
    list: &'a [Field],
    selected: Option<usize>,
    input: Option<&'a String>,
) -> Vec<ListItem<'a>> {
    list.iter()
        .enumerate()
        .map(|(idx, field)| {
            let value: Vec<Span> = if Some(idx) == selected {
                input.as_ref().map_or_else(
                    || vec![Span::from(format!("[ {} ]", field.value))],
                    |value| {
                        vec![
                            Span::raw(format!("[ {value}")),
                            Span::raw(" ").reversed(),
                            Span::raw(" ]"),
                        ]
                    },
                )
            } else {
                vec![format!("  {}", field.value).into()]
            };

            let mut line = Line::raw(format!("[ {: <width$}] ", field.label, width = 20));
            line.extend(value);

            let mut lines: Vec<Line> = vec![line];

            if let Some(error) = &field.error {
                lines.push(Line::from(vec![
                    Span::raw(format!("> {: <width$}", "ERROR".to_string(), width = 24)).italic(),
                    Span::raw(error).red().italic(),
                ]));
            }

            ListItem::from(lines)
        })
        .collect()
}

    pub fn validate_service(service: &Service) -> Result<(), String> {
        if service.name.trim().is_empty() {
            return Err("The service must have a name.".to_string());
        }
        Ok(())
    }

    pub fn validate_account(account: &Account) -> Result<(), String> {
        match &account.contact {
            ContactInfo::Email(email) => {
                if email.trim().is_empty() {
                    return Err("Please submit either an email or a username.".to_string());
                }
            },
            ContactInfo::Username(username) => {
                if username.trim().is_empty() {
                    return Err("Please submit either an email or a username.".to_string());
                }
            },
            ContactInfo::Both(email, username) => {
                if email.trim().is_empty() && username.trim().is_empty() {
                    return Err("Please submit either an email or a username.".to_string());
                }
            },
        }
        Ok(())
    }

    pub fn validate_security_question(sq: &SecurityQuestion) -> Result<(), String> {
        let question = sq.question.trim().is_empty();
        let answer = sq.answer.trim().is_empty();
        match (question, answer) {
            (true, true) => Err("This form is empty - please complete all fields.".to_string()),
            (true, false) => Err("Please fill out the question field.".to_string()),
            (false, true) => Err("Please fill out the answer field.".to_string()),
            (false, false) => Ok(()),
        }
    }

    pub const fn validate_shortcut(_shortcut: &Shortcut) -> Result<(), String> {
        Ok(())
    }
