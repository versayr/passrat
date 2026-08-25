use chrono::NaiveDate;

use crate::models::{
    Account,
    ContactInfo::{Both, Email, Username},
    SecurityQuestion, Service, Shortcut, Target,
};

fn target_is_service(target: &mut Target) -> Result<&mut Service, String> {
    match target {
        Target::Service(service) => Ok(service),
        _ => Err("This field can only be applied to an Account.".to_string()),
    }
}

pub fn apply_service_name(target: &mut Target, name: &str) -> Result<(), String> {
    let service = target_is_service(target)?;
    name.clone_into(&mut service.name);
    Ok(())
}

pub fn apply_service_url(target: &mut Target, url: &str) -> Result<(), String> {
    let service = target_is_service(target)?;
    service.url = Some(url.to_owned());
    Ok(())
}

fn target_is_account(target: &mut Target) -> Result<&mut Account, String> {
    match target {
        Target::Account(account) => Ok(account),
        _ => Err("This field can only be applied to an Account.".to_string()),
    }
}

pub fn apply_account_username(target: &mut Target, username: &str) -> Result<(), String> {
    let account = target_is_account(target)?;

    account.contact = match &account.contact {
        Both(email, _) | Email(email) => Both(email.to_owned(), username.to_owned()),
        Username(_) => Username(username.to_owned()),
    };

    Ok(())
}

pub fn apply_account_email(target: &mut Target, email: &str) -> Result<(), String> {
    let account = target_is_account(target)?;

    account.contact = match &account.contact {
        Both(_, username) | Username(username) => Both(email.to_owned(), username.to_owned()),
        Email(_) => Email(email.to_owned()),
    };

    Ok(())
}

pub fn apply_account_password(target: &mut Target, password: &str) -> Result<(), String> {
    let account = target_is_account(target)?;
    password.clone_into(&mut account.password);
    Ok(())
}

pub fn apply_account_access_token(target: &mut Target, token: &str) -> Result<(), String> {
    let account = target_is_account(target)?;
    token.clone_into(&mut account.access_token);
    Ok(())
}

pub fn apply_account_pin(target: &mut Target, pin: &str) -> Result<(), String> {
    let account = target_is_account(target)?;
    account.pin = pin.trim().parse::<u32>().ok();
    Ok(())
}

pub fn apply_account_passcode(target: &mut Target, passcode: &str) -> Result<(), String> {
    let account = target_is_account(target)?;
    account.passcode = passcode.trim().parse::<u32>().ok();
    Ok(())
}

pub fn apply_account_last_change(target: &mut Target, date: &str) -> Result<(), String> {
    let account = target_is_account(target)?;
    account.last_change = NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map_err(|error| format!("Invalid date format: {error}"))?;
    Ok(())
}

pub fn apply_account_creation_date(target: &mut Target, date: &str) -> Result<(), String> {
    let account = target_is_account(target)?;
    account.creation_date = NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map_err(|error| format!("Invalid date format: {error}"))?;
    Ok(())
}

fn target_is_security_question(target: &mut Target) -> Result<&mut SecurityQuestion, String> {
    match target {
        Target::SecurityQuestion(security_question) => Ok(security_question),
        _ => Err("This field can only be applied to a Security Question.".to_string()),
    }
}

pub fn apply_sq_question(target: &mut Target, question: &str) -> Result<(), String> {
    let sq = target_is_security_question(target)?;
    question.clone_into(&mut sq.question);
    Ok(())
}

pub fn apply_sq_answer(target: &mut Target, answer: &str) -> Result<(), String> {
    let sq = target_is_security_question(target)?;
    answer.clone_into(&mut sq.answer);
    Ok(())
}

fn target_is_shortcut(target: &mut Target) -> Result<&mut Shortcut, String> {
    match target {
        Target::Shortcut(shortcut) => Ok(shortcut),
        _ => Err("This field can only be applied to a Shortcut.".to_string()),
    }
}

pub fn apply_shortcut_sequence(target: &mut Target, seq: &str) -> Result<(), String> {
    let shortcut = target_is_shortcut(target)?;
    seq.clone_into(&mut shortcut.sequence);
    Ok(())
}
