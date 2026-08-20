use crate::models::{
    Account, ContactInfo, EmailAddress, Field, SecurityQuestion, Service, Shortcut, Username,
    Validator::{Date, Email, NonEmpty, Numeric, Url},
};
use std::clone::Clone;

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
            },
            Field {
                label: "Url".to_string(),
                value: self.url.as_ref().map_or_else(String::new, Clone::clone),
                validator: Some(Url),
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
            },
            Field {
                label: "Email".to_string(),
                value: String::from(email),
                validator: Some(Email),
            },
            Field {
                label: "Password".to_string(),
                value: self.password.clone(),
                validator: None,
            },
            Field {
                label: "Access Token".to_string(),
                value: self.access_token.clone(),
                validator: None,
            },
            Field {
                label: "PIN".to_string(),
                value: self
                    .pin
                    .map_or_else(String::new, |pin| pin.clone().to_string()),
                validator: Some(Numeric),
            },
            Field {
                label: "Passcode".to_string(),
                value: self
                    .passcode
                    .map_or_else(String::new, |passcode| passcode.clone().to_string()),
                validator: Some(Numeric),
            },
            Field {
                label: "Last Change".to_string(),
                value: self.last_change.format("%Y-%m-%d").to_string(),
                validator: Some(Date),
            },
            Field {
                label: "Account Created".to_string(),
                value: self.creation_date.format("%Y-%m-%d").to_string(),
                validator: Some(Date),
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
            },
            Field {
                label: "Answer".to_string(),
                value: self.answer.clone(),
                validator: Some(NonEmpty),
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
        }]
    }
}
