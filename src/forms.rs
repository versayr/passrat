use crate::models::{Account, Field, SecurityQuestion, Service, Shortcut};

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
                value: if let Some(url) = &self.url {
                    url.clone()
                } else { 
                    String::new()
                },
            },
        ]
    }
}

impl Fields for Account {
    fn fields(&self) -> Vec<Field> {
        vec![
            Field {
                label: "Username".to_string(),
                value: self.username.clone().unwrap_or_default(),
            },
            Field {
                label: "Email".to_string(),
                value: self.email.clone(),
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
                value: if let Some(pin) = self.pin {
                    pin.clone().to_string()
                } else {
                    String::new()
                },
            },
            Field {
                label: "Passcode".to_string(),
                value: if let Some(passcode) = self.passcode {
                    passcode.clone().to_string()
                } else {
                    String::new()
                },
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
