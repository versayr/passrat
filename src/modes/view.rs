use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{
        Constraint,
        Direction::{self, Vertical},
        HorizontalAlignment, Layout, Rect,
    },
    style::{Modifier, Style},
    text::Line,
    widgets::{
        Block, BorderType, Borders, HighlightSpacing, List, ListItem, ListState, Padding,
        Paragraph, StatefulWidget, Widget,
    },
};

use crate::{
    helpers::{construct_detail_list, format_detail_field, format_hidden_detail_field},
    models::{Account, ContactInfo, Service},
};

#[derive(Debug, Default)]
pub struct View {
    pub service: Service,
    pub accounts: AccountList,
    pub details: DetailList,
    pub selected: Pane,
    pub hide_sensitive: ShowHiddenFields,
}

#[derive(Debug, Default, Clone)]
pub struct AccountList {
    pub list: Vec<Account>,
    pub state: ListState,
}

#[derive(Debug, Default, Clone)]
pub struct DetailList {
    pub list: Vec<Detail>,
    pub state: ListState,
}

#[derive(Debug)]
pub enum ViewAction {
    Edit(Account),
    Delete(Account),
    Copy(String),
    Return,
    Help,
    Quit,
    None,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd)]
pub enum Pane {
    #[default]
    Left,
    Right,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Clone)]
pub enum ShowHiddenFields {
    Always,
    WhenSelected,
    #[default]
    Never,
}

impl ShowHiddenFields {
    const fn next(&self) -> Self {
        match self {
            Self::Never => Self::WhenSelected,
            Self::WhenSelected => Self::Always,
            Self::Always => Self::Never,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Detail {
    pub label: String,
    pub value: String,
    pub hidden: bool,
}

impl View {
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
            selected: Pane::default(),
            hide_sensitive: ShowHiddenFields::default(),
        }
    }

    pub fn handle_inputs(&mut self, event: KeyEvent) -> ViewAction {
        if self.selected == Pane::Right {
            return self.handle_detail_inputs(event);
        }

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
            KeyCode::Tab | KeyCode::Right | KeyCode::Char('l') => {
                self.selected = Pane::Right;
                self.details.state.select_first();
                ViewAction::None
            }
            KeyCode::Char('e') => {
                if self.accounts.state.selected().is_none() {
                    ViewAction::None
                } else {
                    let account = self.get_selected_account();
                    ViewAction::Edit(account.clone())
                }
            }
            KeyCode::Char('n') => {
                let id = self.service.id.expect("No service id found");
                ViewAction::Edit(Account::new(id))
            }
            KeyCode::Delete => {
                if self.accounts.state.selected().is_none() {
                    ViewAction::None
                } else {
                    // TODO confirm this action first with confirm_modal
                    let account = self.get_selected_account();
                    ViewAction::Delete(account.clone())
                }
            }
            KeyCode::Char('y') => {
                if self.accounts.state.selected().is_none() {
                    ViewAction::None
                } else {
                    let account = self.get_selected_account().clone();
                    match account.contact {
                        ContactInfo::Email(email) => ViewAction::Copy(email),
                        ContactInfo::Both(_, username) | ContactInfo::Username(username) => {
                            ViewAction::Copy(username)
                        }
                    }
                }
            }
            KeyCode::Enter => {
                self.hide_sensitive = self.hide_sensitive.next();
                ViewAction::None
            }
            _ => ViewAction::None,
        }
    }

    fn handle_detail_inputs(&mut self, event: KeyEvent) -> ViewAction {
        match event.code {
            KeyCode::Char('q' | 'h')
            | KeyCode::Esc
            | KeyCode::Right
            | KeyCode::Left
            | KeyCode::Tab => {
                self.selected = Pane::Left;
                self.details.state.select(None);
                ViewAction::None
            }
            KeyCode::Char('?') => ViewAction::Help,
            KeyCode::Char('j') | KeyCode::Down => {
                self.details.state.select_next();
                ViewAction::None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.details.state.select_previous();
                ViewAction::None
            }
            KeyCode::Char('e') => {
                if self.accounts.state.selected().is_none() {
                    ViewAction::None
                } else {
                    let account = self.get_selected_account();
                    ViewAction::Edit(account.clone())
                }
            }
            KeyCode::Char('y') => {
                if self.details.list.is_empty() {
                    ViewAction::None
                } else {
                    let selected = self
                        .details
                        .state
                        .selected()
                        .expect("Failed to get selected detail.");
                    let detail = self
                        .details
                        .list
                        .get(selected)
                        .expect("Index out of range.");
                    ViewAction::Copy(detail.value.clone())
                }
            }
            KeyCode::Enter => {
                self.hide_sensitive = self.hide_sensitive.next();
                ViewAction::None
            }
            _ => ViewAction::None,
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
        let accounts = &self.accounts;

        let block = Block::bordered()
            .border_type(BorderType::Double)
            .title_alignment(HorizontalAlignment::Center)
            .title("[ [ ACCOUNTS ] ]");

        let account_list: Vec<ListItem> = accounts
            .list
            .iter()
            .map(|account| {
                let text = match &account.contact {
                    ContactInfo::Both(_, username) | ContactInfo::Username(username) => username,
                    ContactInfo::Email(email) => email,
                };
                ListItem::new(Line::raw(text.clone()))
            })
            .collect();

        let list = List::new(account_list)
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

    //    fn render_account_details(&mut self, area: Rect, buf: &mut Buffer) {
    //        let account = self.get_selected_account().clone();
    //
    //        let block = Block::bordered()
    //            .border_type(BorderType::Double)
    //            .title_alignment(HorizontalAlignment::Center)
    //            .title("[ [ ACCOUNT DETAILS ] ]")
    //            .padding(Padding::left(1));
    //
    //        let list = construct_detail_list(&account)
    //            .block(block)
    //            .highlight_style(
    //                Style::new()
    //                    .add_modifier(Modifier::BOLD)
    //                    .add_modifier(Modifier::REVERSED),
    //            );
    //
    //        StatefulWidget::render(list, area, buf, &mut self.details.state);
    //    }

    fn render_account_details(&mut self, area: Rect, buf: &mut Buffer) {
        let account = self.get_selected_account();
        self.details.list = construct_detail_list(account);

        let block = Block::bordered()
            .border_type(BorderType::Double)
            .title_alignment(HorizontalAlignment::Center)
            .title("[ [ ACCOUNT DETAILS ] ]")
            .padding(Padding::left(1))
            .padding(Padding::right(1));

        let layout = Layout::default()
            .direction(Vertical)
            .constraints(vec![Constraint::Fill(1), Constraint::Length(2)])
            .split(Block::inner(&block, area));
        let top = layout.first().expect("Malformed layout.");
        let bottom = layout.get(1).expect("Malformed layout.");

        let width = 19;
        let detail_list: Vec<ListItem> = self
            .details
            .list
            .iter()
            .enumerate()
            .map(|(idx, detail)| {
                if let Some(selected_idx) = self.details.state.selected() {
                    let is_selected = idx.eq(&selected_idx);
                    let field = match (&self.hide_sensitive, is_selected) {
                        (&ShowHiddenFields::Always, _)
                        | (&ShowHiddenFields::WhenSelected, true) => {
                            format_detail_field(detail, width)
                        }
                        (&ShowHiddenFields::WhenSelected, false)
                        | (&ShowHiddenFields::Never, _) => {
                            if detail.hidden {
                                format_hidden_detail_field(detail, width)
                            } else {
                                format_detail_field(detail, width)
                            }
                        }
                    };
                    ListItem::new(field)
                } else {
                    let field = match &self.hide_sensitive {
                        ShowHiddenFields::Always => format_detail_field(detail, width),
                        ShowHiddenFields::WhenSelected | ShowHiddenFields::Never => {
                            if detail.hidden {
                                format_hidden_detail_field(detail, width)
                            } else {
                                format_detail_field(detail, width)
                            }
                        }
                    };
                    ListItem::new(field)
                }
            })
            .collect();

        let list = List::new(detail_list)
            .highlight_symbol(" ")
            .highlight_style(
                Style::new()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED),
            )
            .highlight_spacing(HighlightSpacing::Always);

        let hidden_field_status = match self.hide_sensitive {
            ShowHiddenFields::Always => "Always",
            ShowHiddenFields::WhenSelected => "When Selected",
            ShowHiddenFields::Never => "Never",
        };
        let status_line = Paragraph::new(format!(" Show Hidden Fields: {hidden_field_status}"))
            .block(
                Block::bordered()
                    .border_type(BorderType::LightQuadrupleDashed)
                    .borders(Borders::TOP),
            );

        StatefulWidget::render(list, *top, buf, &mut self.details.state);
        Widget::render(status_line, *bottom, buf);
        block.render(area, buf);
    }

    fn get_selected_account(&self) -> &Account {
        let selected = self
            .accounts
            .state
            .selected()
            .expect("No account is selected.");
        self.accounts
            .list
            .get(selected)
            .expect("Index is out of range.")
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
            .constraints(vec![Constraint::Length(4), Constraint::Fill(1)])
            .split(Block::inner(&block, area));

        let header_pane = main_layout.first().expect("Malformed main layout.");
        let body = main_layout.get(1).expect("Malformed main layout.");

        let body_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(*body);

        let account_pane = body_layout.first().expect("Malformed body layout.");
        let details_pane = body_layout.get(1).expect("Malformed body layout.");

        self.render_service_details(*header_pane, buf);
        self.render_account_list(*account_pane, buf);
        if self.accounts.list.is_empty() {
            render_empty_accounts_alert(*details_pane, buf);
        } else {
            self.render_account_details(*details_pane, buf);
        }

        block.render(area, buf);
    }
}
