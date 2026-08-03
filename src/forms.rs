use crate::models::{
    Account, ContactInfo, EmailAddress, Field, SecurityQuestion, Service, Shortcut, Username,
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
            },
            Field {
                label: "URL".to_string(),
                value: self.url.as_ref().map_or_else(String::new, Clone::clone),
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
            },
            Field {
                label: "Email".to_string(),
                value: String::from(email),
            },
            Field {
                label: "Password".to_string(),
                value: self.password.clone(),
            },
            Field {
                label: "Access Token".to_string(),
                value: self.access_token.clone(),
            },
            Field {
                label: "PIN".to_string(),
                value: self
                    .pin
                    .map_or_else(String::new, |pin| pin.clone().to_string()),
            },
            Field {
                label: "Passcode".to_string(),
                value: self
                    .passcode
                    .map_or_else(String::new, |passcode| passcode.clone().to_string()),
            },
            Field {
                label: "Last Change".to_string(),
                value: self.last_change.format("%Y-%m-%d").to_string(),
            },
            Field {
                label: "Account Created".to_string(),
                value: self.creation_date.format("%Y-%m-%d").to_string(),
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
            },
            Field {
                label: "Answer".to_string(),
                value: self.answer.clone(),
            },
        ]
    }
}

impl Fields for Shortcut {
    fn fields(&self) -> Vec<Field> {
        vec![Field {
            label: "Sequence".to_string(),
            value: self.sequence.clone(),
        }]
    }
}
