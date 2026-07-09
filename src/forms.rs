use ratatui::style::{Modifier, Style};
use ratatui::widgets::{HighlightSpacing, List, ListItem};

use crate::models::{Account, Service};

#[allow(dead_code)]
trait Form {
    fn form(&self) -> List<'_>;
}

impl Form for Service {
    fn form(&self) -> List<'_> {
        let list_items: Vec<ListItem> = vec![
            ListItem::new(self.name.clone()),
            ListItem::new(self.url.as_ref().expect("No url for this service.").clone()),
        ];

        List::new(list_items)
            .highlight_symbol(" > ")
            .highlight_style(
                Style::new()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED),
            )
            .highlight_spacing(HighlightSpacing::Always)
    }
}

impl Form for Account {
    fn form(&self) -> List<'_> {
        let list_items: Vec<ListItem> = vec![
            ListItem::new(format!("Username: {}", self.username.clone())),
            ListItem::new(format!(
                "Email: {}",
                self.email
                    .as_ref()
                    .expect("No email for this service.")
                    .clone()
            )),
            ListItem::new(format!(
                "Password: {}",
                self.password
                    .as_ref()
                    .expect("No password for this service.")
                    .clone()
            )),
            ListItem::new(format!(
                "Access Token: {}",
                self.access_token
                    .as_ref()
                    .expect("No access token for this service.")
                    .clone()
            )),
            ListItem::new(format!(
                "PIN: {}",
                self.pin
                    .as_ref()
                    .expect("No access token for this service.")
                    .clone()
            )),
            ListItem::new(format!(
                "Passcode: {}",
                self.passcode
                    .as_ref()
                    .expect("No access token for this service.")
                    .clone()
            )),
            ListItem::new(format!("Last Change: {}", self.last_change.clone())),
            ListItem::new(format!("Account Created: {}", self.creation_date.clone())),
        ];

        List::new(list_items)
            .highlight_symbol(" > ")
            .highlight_style(
                Style::new()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED),
            )
            .highlight_spacing(HighlightSpacing::Always)
    }
}
