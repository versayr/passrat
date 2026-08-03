use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, HorizontalAlignment, Layout, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::{
        Block, BorderType, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
        StatefulWidget, Widget,
    },
};

use crate::{
    helpers::construct_detail_list,
    models::{Account, ContactInfo, Field, Service},
};

#[derive(Debug, Default)]
pub struct View {
    pub service: Service,
    pub accounts: AccountList,
    pub details: DetailList,
    pub selected: Pane,
}

#[derive(Debug, Default, Clone)]
pub struct AccountList {
    pub list: Vec<Account>,
    pub state: ListState,
}

#[derive(Debug, Default, Clone)]
pub struct DetailList {
    pub list: Vec<Field>,
    pub state: ListState,
}

#[derive(Debug)]
pub enum ViewAction {
    Edit(Account),
    // Delete(Account),
    // Paste(String),
    Copy(String),
    Return,
    Help,
    Quit,
    None,
}

#[derive(Debug, Default)]
pub enum Pane {
    #[default]
    Left,
    Right,
}

impl View {
    pub fn handle_inputs(&mut self, event: KeyEvent) -> ViewAction {
        match event.code {
            KeyCode::Char('q') | KeyCode::Esc => ViewAction::Return,
            KeyCode::Char('h' | '?') => ViewAction::Help,
            KeyCode::Char('j') | KeyCode::Down => {
                self.accounts.state.select_next();
                ViewAction::None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.accounts.state.select_previous();
                ViewAction::None
            }
            KeyCode::Char('e') => {
                if self.accounts.state.selected().is_none() {
                    ViewAction::None
                } else {
                    let selected = self
                        .accounts
                        .state
                        .selected()
                        .expect("No account is selected.");
                    let account = self
                        .accounts
                        .list
                        .get(selected)
                        .expect("Index is out of range.");
                    ViewAction::Edit(account.clone())
                }
            }
            KeyCode::Char('n') => {
                let id = self.service.id.expect("No service id found");
                ViewAction::Edit(Account::new(id))
            }
            _ => ViewAction::None,
        }
    }

    pub fn new(service: &Service, list: Vec<Account>) -> Self {
        let mut accounts = AccountList {
            list,
            state: ListState::default(),
        };

        accounts.state.select_first();

        Self {
            service: service.clone(),
            accounts,
            details: DetailList::default(),
            selected: Pane::Left,
        }
    }

    fn render_service_details(&self, area: Rect, buf: &mut Buffer) {
        let name = &self.service.name;
        let url = self.service.url.as_deref().unwrap_or("");

        let block = Block::bordered()
            .border_type(BorderType::Double)
            .title_alignment(HorizontalAlignment::Center)
            .title("[ [ SERVICE DETAILS ] ]");

        let service_details = vec![
            Line::from(format!(" {name} ")),
            Line::from(format!(" {url} ")),
        ];

        let header = Paragraph::new(service_details).block(block);

        header.render(area, buf);
    }

    fn render_account_list(&mut self, area: Rect, buf: &mut Buffer) {
        let accounts = &self.accounts.clone();

        let block = Block::bordered()
            .border_type(BorderType::Double)
            .title_alignment(HorizontalAlignment::Center)
            .title("[ [ ACCOUNTS ] ]");

        let accounts: Vec<ListItem> = accounts
            .list
            .iter()
            .map(|account| {
                let text = match &account.contact {
                    ContactInfo::Both(_, username) | ContactInfo::Username(username) => {
                        String::from(username)
                    }
                    ContactInfo::Email(email) => String::from(email),
                };
                ListItem::new(Line::from(text))
            })
            .collect();

        let list = List::new(accounts)
            .highlight_symbol(" > ")
            .highlight_style(
                Style::new()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED),
            )
            .highlight_spacing(HighlightSpacing::Always)
            .block(block);

        StatefulWidget::render(list, area, buf, &mut self.accounts.state);
    }

    fn render_account_details(&mut self, area: Rect, buf: &mut Buffer) {
        let selected_idx = self
            .accounts
            .state
            .selected()
            .expect("No account is selected.");
        let account = self
            .accounts
            .list
            .get(selected_idx)
            .expect("Index out of range.");

        let block = Block::bordered()
            .border_type(BorderType::Double)
            .title_alignment(HorizontalAlignment::Center)
            .title("[ [ ACCOUNT DETAILS ] ]")
            .padding(Padding::left(1));

        let list = construct_detail_list(account).block(block);

        StatefulWidget::render(list, area, buf, &mut self.details.state);
    }
}

impl Widget for &mut View {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from(" View Mode ");
        let block = Block::bordered()
            .title(title)
            .border_type(BorderType::Rounded);

        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(4), Constraint::Fill(1)])
            .split(Block::inner(&block, area));

        let header = main_layout.first().expect("Malformed main layout.");
        let body = main_layout.get(1).expect("Malformed main layout.");

        let body_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(*body);

        let account_pane = body_layout.first().expect("Malformed body layout.");
        let details_pane = body_layout.get(1).expect("Malformed body layout.");

        self.render_service_details(*header, buf);
        self.render_account_list(*account_pane, buf);
        if self.accounts.list.is_empty() {
            render_empty_accounts_alert(*details_pane, buf);
        } else {
            self.render_account_details(*details_pane, buf);
        }

        block.render(area, buf);
    }
}

fn render_empty_accounts_alert(area: Rect, buf: &mut Buffer) {
    let block = Block::bordered()
        .border_type(BorderType::Double)
        .title_alignment(HorizontalAlignment::Center)
        .title("[ [ ACCOUNT DETAILS ] ]")
        .padding(Padding::left(1));

    let lines = vec![
        Line::from("No accounts found for this service"),
        Line::from("Press 'n' to add a new one"),
    ];

    Widget::render(List::new(lines).block(block), area, buf);
}
