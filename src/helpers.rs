use std::error::Error;

use chrono::{Datelike, NaiveDate};
use ordinal::ToOrdinal;
use ratatui::{
    style::Stylize,
    text::{Line, Span},
    widgets::ListItem,
};
use rustpass::{PassphraseConfig, PassphraseGenerator};

use crate::{
    models::{Account, ContactInfo, Field, SecurityQuestion, Service, Shortcut},
    modes::{edit::Input, view::Detail},
};

pub fn format_current_date(date: NaiveDate) -> String {
    format!(
        "{}, {} {}, {}",
        date.format("%A"),
        date.format("%B"),
        date.day().to_ordinal_string(),
        date.format("%Y")
    )
}

// pub fn construct_detail_field(label: &str, value: &str, width: usize) -> Line<'static> {
//     Line::from(vec![
//         format!("{label: <width$}").into(),
//         value.to_string().into(),
//     ])
// }

pub fn format_detail_field(detail: &Detail, width: usize) -> Line<'static> {
    Line::from(vec![
        format!("{0: <width$}", detail.label).into(),
        detail.value.clone().into(),
    ])
}

pub fn format_hidden_detail_field(detail: &Detail, width: usize) -> Line<'static> {
    Line::from(vec![
        format!("{0: <width$}", detail.label).into(),
        "[*]".into(),
    ])
}

pub fn construct_detail_list(account: &Account) -> Vec<Detail> {
    let mut detail_list = vec![];

    let (email, username) = match &account.contact {
        ContactInfo::Both(email, name) => (Some(email), Some(name)),
        ContactInfo::Email(email) => (Some(email), None),
        ContactInfo::Username(name) => (None, Some(name)),
    };

    if let Some(username) = username {
        detail_list.push(Detail {
            label: "Username".to_string(),
            value: String::from(username),
            hidden: false,
        });
    }

    if let Some(email) = email {
        detail_list.push(Detail {
            label: "Email".to_string(),
            value: String::from(email),
            hidden: false,
        });
    }

    if let Some(password) = &account.password {
        detail_list.push(Detail {
            label: "Password".to_string(),
            value: String::from(password),
            hidden: true,
        });
    }

    if let Some(access_token) = &account.access_token {
        detail_list.push(Detail {
            label: "Access Token".to_string(),
            value: String::from(access_token),
            hidden: true,
        });
    }

    if let Some(pin) = account.pin {
        detail_list.push(Detail {
            label: "PIN".to_string(),
            value: pin.to_string(),
            hidden: true,
        });
    }

    if let Some(passcode) = account.passcode {
        detail_list.push(Detail {
            label: "Passcode".to_string(),
            value: passcode.to_string(),
            hidden: true,
        });
    }

    detail_list.push(Detail {
        label: "Last Change".to_string(),
        value: format_current_date(account.last_change),
        hidden: false,
    });

    detail_list.push(Detail {
        label: "Account Created".to_string(),
        value: format_current_date(account.creation_date),
        hidden: false,
    });

    detail_list
}

// pub fn construct_detail_list(account: &Account) -> List<'_> {
//     let mut lines = vec![];
//     let width = 17;
//
//     let (email, username) = match &account.contact {
//         ContactInfo::Both(email, name) => (Some(email), Some(name)),
//         ContactInfo::Email(email) => (Some(email), None),
//         ContactInfo::Username(name) => (None, Some(name)),
//     };
//
//     if let Some(username) = username {
//         lines.push(construct_detail_field(
//             "Username",
//             &String::from(username),
//             width,
//         ));
//     }
//
//     if let Some(email) = email {
//         lines.push(construct_detail_field("Email", &String::from(email), width));
//     }
//
//     if !account.password.is_empty() {
//         lines.push(construct_detail_field("Password", "{*}", width));
//     }
//
//     if !account.access_token.is_empty() {
//         lines.push(construct_detail_field(
//             "Access Token",
//             &account.access_token,
//             width,
//         ));
//     }
//
//     if let Some(pin) = account.pin {
//         lines.push(construct_detail_field("PIN", &pin.to_string(), width));
//     }
//
//     if let Some(passcode) = account.passcode {
//         lines.push(construct_detail_field(
//             "Passcode",
//             &passcode.to_string(),
//             width,
//         ));
//     }
//
//     lines.push(construct_detail_field(
//         "Last Change",
//         &format_current_date(account.last_change),
//         width,
//     ));
//     lines.push(construct_detail_field(
//         "Account Created",
//         &format_current_date(account.creation_date),
//         width,
//     ));
//
//     List::new(lines)
// }

pub fn gen_password(config: PassphraseConfig) -> Result<String, Box<dyn Error>> {
    let generator = PassphraseGenerator::with_default_wordlist(config)?;
    let password = generator.generate()?;
    Ok(password)
}

pub fn construct_field_list<'a>(
    list: &'a [Field],
    selected: Option<usize>,
    input: Option<&'a Input>,
) -> Vec<ListItem<'a>> {
    list.iter()
        .enumerate()
        .map(|(idx, field)| {
            let value: Vec<Span> = if Some(idx) == selected {
                input.as_ref().map_or_else(
                    || vec![Span::from(format!("[ {} ]", field.value))],
                    |input| {
                        let (prefix, suffix) = input.value.split_at(input.index);
                        if suffix.is_empty() {
                            vec![
                                Span::raw(format!("[ {prefix}")),
                                Span::raw(" ").reversed(),
                                Span::raw(" ]".to_string()),
                            ]
                        } else {
                            let (selected_char, suffix) = suffix.split_at(1);
                            vec![
                                Span::raw(format!("[ {prefix}")),
                                Span::raw(selected_char.to_string()).reversed(),
                                Span::raw(format!("{suffix} ]")),
                            ]
                        }
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
        }
        ContactInfo::Username(username) => {
            if username.trim().is_empty() {
                return Err("Please submit either an email or a username.".to_string());
            }
        }
        ContactInfo::Both(email, username) => {
            if email.trim().is_empty() && username.trim().is_empty() {
                return Err("Please submit either an email or a username.".to_string());
            }
        }
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

pub fn validate_shortcut(shortcut: &Shortcut) -> Result<(), String> {
    match (
        shortcut.field.trim().is_empty(),
        shortcut.sequence.trim().is_empty(),
    ) {
        (true, true) => Err("This form is empty - please complete all fields.".to_string()),
        (true, false) => Err("Please fill out the target field.".to_string()),
        (false, true) => Err("Please fill out the sequence field.".to_string()),
        (false, false) => Ok(()),
    }
}
