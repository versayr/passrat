use std::str::FromStr;

use jiff::civil::Date;

use crate::models::{
    Account, Applicator, ContactInfo::{Both, Email, Username}, SecurityQuestion, Service, Shortcut, Target,
};

impl Applicator {
    pub fn apply(self, target: &mut Target, value: &str) -> Result<(), String> {
        match (self, target) {
            (Self::ServiceName, Target::Service(service)) => {
                apply_service_name(service, value);
                Ok(())
            }
            (Self::ServiceUrl, Target::Service(service)) => {
                apply_service_url(service, value);
                Ok(())
            }
            (Self::AccountUsername, Target::Account(account)) => {
                apply_account_username(account, value);
                Ok(())
            }
            (Self::AccountEmail, Target::Account(account)) => {
                apply_account_email(account, value);
                Ok(())
            }
            (Self::AccountPassword, Target::Account(account)) => {
                apply_account_password(account, value);
                Ok(())
            }
            (Self::AccountAccessToken, Target::Account(account)) => {
                apply_account_access_token(account, value);
                Ok(())
            }
            (Self::AccountPIN, Target::Account(account)) => {
                apply_account_pin(account, value);
                Ok(())
            }
            (Self::AccountPasscode, Target::Account(account)) => {
                apply_account_passcode(account, value);
                Ok(())
            }
            (Self::AccountLastChange, Target::Account(account)) => {
                apply_account_last_change(account, value)
            }
            (Self::AccountCreationDate, Target::Account(account)) => {
                apply_account_creation_date(account, value)
            }
            (Self::SqQuestion, Target::SecurityQuestion(question)) => {
                apply_sq_question(question, value);
                Ok(())
            }
            (Self::SqAnswer, Target::SecurityQuestion(question)) => {
                apply_sq_answer(question, value);
                Ok(())
            }
            (Self::ShortcutSequence, Target::Shortcut(shortcut)) => {
                apply_shortcut_sequence(shortcut, value);
                Ok(())
            }

            (applicator, target) => Err(format!(
                "Cannot apply {applicator:?} to {target:?}"
            )),
        }
    }
}


pub fn apply_service_name(service: &mut Service, name: &str) {
    name.clone_into(&mut service.name);
}

pub fn apply_service_url(service: &mut Service, url: &str) {
    service.url = Some(url.to_owned());
}

pub fn apply_account_username(account: &mut Account, username: &str) {
    account.contact = match &account.contact {
        Both(email, _) | Email(email) => Both(email.to_owned(), username.to_owned()),
        Username(_) => Username(username.to_owned()),
    };
}

pub fn apply_account_email(account: &mut Account, email: &str) {
    account.contact = match &account.contact {
        Both(_, username) | Username(username) => Both(email.to_owned(), username.to_owned()),
        Email(_) => Email(email.to_owned()),
    };
}

pub fn apply_account_password(account: &mut Account, password: &str) {
    if password.trim().is_empty() {
        account.password = None;
    } else {
        account.password = Some(password.to_string());
    }
}

pub fn apply_account_access_token(account: &mut Account, token: &str) {
    if token.trim().is_empty() {
        account.access_token = None;
    } else {
        account.access_token = Some(token.to_string());
    }
}

pub fn apply_account_pin(account: &mut Account, pin: &str) {
    account.pin = pin.trim().parse::<u32>().ok();
}

pub fn apply_account_passcode(account: &mut Account, passcode: &str) {
    account.passcode = passcode.trim().parse::<u32>().ok();
}

pub fn apply_account_last_change(account: &mut Account, date: &str) -> Result<(), String> {
    account.last_change =
        Date::from_str(date).map_err(|error| format!("Invalid date format: {error}"))?;
    Ok(())
}

pub fn apply_account_creation_date(account: &mut Account, date: &str) -> Result<(), String> {
    account.creation_date =
        Date::from_str(date).map_err(|error| format!("Invalid date format: {error}"))?;
    Ok(())
}

pub fn apply_sq_question(sq: &mut SecurityQuestion, question: &str) {
    question.clone_into(&mut sq.question);
}

pub fn apply_sq_answer(sq: &mut SecurityQuestion, answer: &str) {
    answer.clone_into(&mut sq.answer);
}

pub fn apply_shortcut_sequence(shortcut: &mut Shortcut, seq: &str) {
    seq.clone_into(&mut shortcut.sequence);
}
