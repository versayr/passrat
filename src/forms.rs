use crate::{
    models::{
        Account, Applicator, ContactInfo, EmailAddress, Field, SecurityQuestion, Service, Shortcut, Username,
    }, validators::Validator::{Date, Email, NoWhitespace, NonEmpty, Numeric, Url},
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
                applicator: Applicator::ServiceName,
            },
            Field {
                label: "Url".to_string(),
                value: self.url.as_ref().map_or_else(String::new, Clone::clone),
                validator: Some(Url),
                error: None,
                applicator: Applicator::ServiceUrl,
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
                applicator: Applicator::AccountUsername,
            },
            Field {
                label: "Email".to_string(),
                value: String::from(email),
                validator: Some(Email),
                error: None,
                applicator: Applicator::AccountEmail,
            },
            Field {
                label: "Password".to_string(),
                value: self
                    .password
                    .as_ref()
                    .map_or_else(String::new, ToString::to_string),
                validator: Some(NoWhitespace),
                error: None,
                applicator: Applicator::AccountPassword,
            },
            Field {
                label: "Access Token".to_string(),
                value: self
                    .access_token
                    .as_ref()
                    .map_or_else(String::new, ToString::to_string),
                validator: None,
                error: None,
                applicator: Applicator::AccountAccessToken,
            },
            Field {
                label: "PIN".to_string(),
                value: self
                    .pin
                    .map_or_else(String::new, |pin| pin.clone().to_string()),
                validator: Some(Numeric),
                error: None,
                applicator: Applicator::AccountPIN,
            },
            Field {
                label: "Passcode".to_string(),
                value: self
                    .passcode
                    .map_or_else(String::new, |passcode| passcode.clone().to_string()),
                validator: Some(Numeric),
                error: None,
                applicator: Applicator::AccountPasscode,
            },
            Field {
                label: "Last Change".to_string(),
                value: self.last_change.to_string(),
                validator: Some(Date),
                error: None,
                applicator: Applicator::AccountLastChange,
            },
            Field {
                label: "Account Created".to_string(),
                value: self.creation_date.to_string(),
                validator: Some(Date),
                error: None,
                applicator: Applicator::AccountCreationDate,
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
                applicator: Applicator::SqQuestion,
            },
            Field {
                label: "Answer".to_string(),
                value: self.answer.clone(),
                validator: Some(NonEmpty),
                error: None,
                applicator: Applicator::SqAnswer,
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
            applicator: Applicator::ShortcutSequence,
        }]
    }
}
