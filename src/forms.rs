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
                value: self.url.clone(),
            },
        ]
    }
}

impl Fields for Account {
    fn fields(&self) -> Vec<Field> {
        vec![
            Field {
                label: "Username".to_string(),
                value: self.username.clone(),
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
                value: format!("{:?}", self.pin.clone()),
            },
            Field {
                label: "Passcode".to_string(),
                value: format!("{:?}", self.passcode.clone()),
            },
            Field {
                label: "Last Change".to_string(),
                value: self.last_change.clone(),
            },
            Field {
                label: "Account Created".to_string(),
                value: self.creation_date.clone(),
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
        vec![
            Field {
                label: "Sequence".to_string(),
                value: self.sequence.clone(),
            },
        ]
    }
}

// #[allow(dead_code)]
// trait Form {
//     fn form(&self) -> List<'_>;
// }
// 
// impl Form for Service {
//     fn form(&self) -> List<'_> {
//         let list_items: Vec<ListItem> = vec![
//             ListItem::new(self.name.clone()),
//             ListItem::new(self.url.as_ref().expect("No url for this service.").clone()),
//         ];
// 
//         List::new(list_items)
//             .highlight_symbol(" > ")
//             .highlight_style(
//                 Style::new()
//                     .add_modifier(Modifier::BOLD)
//                     .add_modifier(Modifier::REVERSED),
//             )
//             .highlight_spacing(HighlightSpacing::Always)
//     }
// }
// 
// impl Form for Account {
//     fn form(&self) -> List<'_> {
//         let list_items: Vec<ListItem> = vec![
//             ListItem::new(format!("Username: {}", self.username.clone())),
//             ListItem::new(format!(
//                 "Email: {}",
//                 self.email
//                     .as_ref()
//                     .expect("No email for this service.")
//                     .clone()
//             )),
//             ListItem::new(format!(
//                 "Password: {}",
//                 self.password
//                     .as_ref()
//                     .expect("No password for this service.")
//                     .clone()
//             )),
//             ListItem::new(format!(
//                 "Access Token: {}",
//                 self.access_token
//                     .as_ref()
//                     .expect("No access token for this service.")
//                     .clone()
//             )),
//             ListItem::new(format!(
//                 "PIN: {}",
//                 self.pin
//                     .as_ref()
//                     .expect("No access token for this service.")
//                     .clone()
//             )),
//             ListItem::new(format!(
//                 "Passcode: {}",
//                 self.passcode
//                     .as_ref()
//                     .expect("No access token for this service.")
//                     .clone()
//             )),
//             ListItem::new(format!("Last Change: {}", self.last_change.clone())),
//             ListItem::new(format!("Account Created: {}", self.creation_date.clone())),
//         ];
// 
//         List::new(list_items)
//             .highlight_symbol(" > ")
//             .highlight_style(
//                 Style::new()
//                     .add_modifier(Modifier::BOLD)
//                     .add_modifier(Modifier::REVERSED),
//             )
//             .highlight_spacing(HighlightSpacing::Always)
//     }
// }

