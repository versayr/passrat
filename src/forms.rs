use crate::{
    applicators::{
        apply_account_access_token, apply_account_creation_date, apply_account_email,
        apply_account_last_change, apply_account_passcode, apply_account_password,
        apply_account_pin, apply_account_username, apply_service_name, apply_service_url,
        apply_shortcut_sequence, apply_sq_answer, apply_sq_question,
    },
    models::{
        Account, ContactInfo, EmailAddress, Field, SecurityQuestion, Service, Shortcut, Username,
    },
    validators::Validator::{Date, Email, NonEmpty, Numeric, Url},
};
use std::{clone::Clone, string::ToString};

pub trait Fields {
    fn fields(&self) -> Vec<Field>;
}

impl Fields for Service {
    fn fields(&self) -> Vec<Field> {
        vec![
            Field {
                label: "Service Name".to_string(),
                value: self.name.clone(),
                validator: Some(NonEmpty),
                error: None,
                apply: apply_service_name,
            },
            Field {
                label: "Url".to_string(),
                value: self.url.as_ref().map_or_else(String::new, Clone::clone),
                validator: Some(Url),
                error: None,
                apply: apply_service_url,
            },
        ]
    }
}

impl Fields for Account {
    fn fields(&self) -> Vec<Field> {
        let contact: &ContactInfo = &self.contact;
        let (email, username) = match contact {
            ContactInfo::Both(email, name) => (email, name),
            ContactInfo::Email(email) => (email, &Username::from(String::new())),
            ContactInfo::Username(name) => (&EmailAddress::from(String::new()), name),
        };

        vec![
            Field {
                label: "Username".to_string(),
                value: String::from(username),
                validator: None,
                error: None,
                apply: apply_account_username,
            },
            Field {
                label: "Email".to_string(),
                value: String::from(email),
                validator: Some(Email),
                error: None,
                apply: apply_account_email,
            },
            Field {
                label: "Password".to_string(),
                value: self
                    .password
                    .as_ref()
                    .map_or_else(String::new, ToString::to_string),
                validator: None,
                error: None,
                apply: apply_account_password,
            },
            Field {
                label: "Access Token".to_string(),
                value: self
                    .access_token
                    .as_ref()
                    .map_or_else(String::new, ToString::to_string),
                validator: None,
                error: None,
                apply: apply_account_access_token,
            },
            Field {
                label: "PIN".to_string(),
                value: self
                    .pin
                    .map_or_else(String::new, |pin| pin.clone().to_string()),
                validator: Some(Numeric),
                error: None,
                apply: apply_account_pin,
            },
            Field {
                label: "Passcode".to_string(),
                value: self
                    .passcode
                    .map_or_else(String::new, |passcode| passcode.clone().to_string()),
                validator: Some(Numeric),
                error: None,
                apply: apply_account_passcode,
            },
            Field {
                label: "Last Change".to_string(),
                value: self.last_change.format("%Y-%m-%d").to_string(),
                validator: Some(Date),
                error: None,
                apply: apply_account_last_change,
            },
            Field {
                label: "Account Created".to_string(),
                value: self.creation_date.format("%Y-%m-%d").to_string(),
                validator: Some(Date),
                error: None,
                apply: apply_account_creation_date,
            },
        ]
    }
}

impl Fields for SecurityQuestion {
    fn fields(&self) -> Vec<Field> {
        vec![
            Field {
                label: "Question".to_string(),
                value: self.question.clone(),
                validator: Some(NonEmpty),
                error: None,
                apply: apply_sq_question,
            },
            Field {
                label: "Answer".to_string(),
                value: self.answer.clone(),
                validator: Some(NonEmpty),
                error: None,
                apply: apply_sq_answer,
            },
        ]
    }
}

impl Fields for Shortcut {
    fn fields(&self) -> Vec<Field> {
        vec![Field {
            label: "Sequence".to_string(),
            value: self.sequence.clone(),
            validator: Some(NonEmpty),
            error: None,
            apply: apply_shortcut_sequence,
        }]
    }
}
