use std::vec;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, HorizontalAlignment, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, BorderType, HighlightSpacing, List, ListItem, Padding, Paragraph, StatefulWidget,
        Widget,
    },
};

use crate::App;
use crate::models::Service;

impl App {
    pub fn render_lock_mode(&mut self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Login Screen ");
        let block = Block::bordered()
            .title(title)
            .padding(Padding::uniform(1))
            .border_type(BorderType::Rounded);

        let input_area =
            Block::inner(&block, area).centered(Constraint::Length(60), Constraint::Length(6));

        let input_block = Block::bordered()
            .title(Line::from("[ [ ENTER PASSPHRASE ] ]"))
            .padding(Padding::uniform(1))
            .border_type(BorderType::Double);

        let input = Paragraph::new(vec![
            Line::from(vec![
                Span::from("*".repeat(self.password.len())),
                Span::styled(" ", Style::reversed(Style::default())),
            ]),
            Line::from(self.alert.clone()),
        ])
        .block(input_block);

        input.render(input_area, buf);
        block.render(area, buf);
    }

    pub fn render_list_mode(&mut self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" List Mode ");
        let block = Block::bordered()
            .title(title)
            .padding(Padding::uniform(1))
            .border_type(BorderType::Rounded);

        if self.services.list.is_empty() {
            Widget::render(
                self.construct_empty_services_alert(),
                Block::inner(&block, area),
                buf,
            );
        } else {
            let list = self.construct_service_list();
            StatefulWidget::render(
                list,
                Block::inner(&block, area),
                buf,
                &mut self.services.state,
            );
        }

        block.render(area, buf);
    }

    pub fn render_edit_mode(&mut self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Edit Mode ");
        let block = Block::bordered()
            .title(title)
            .border_type(BorderType::Rounded);

        block.render(area, buf);
    }

    pub fn render_view_mode(&mut self, area: Rect, buf: &mut Buffer, service: &Service) {
        let title = Line::from(" View Mode ");
        let block = Block::bordered()
            .title(title)
            .border_type(BorderType::Rounded);

        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(4), Constraint::Fill(1)])
            .split(Block::inner(&block, area));

        let body_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(main_layout[1]);

        self.render_service_details(main_layout[0], buf, service);
        self.render_account_list(body_layout[0], buf);
        self.render_account_details(body_layout[1], buf);

        block.render(area, buf);
    }

    pub fn render_help_mode(&mut self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Help Mode ");
        let block = Block::bordered()
            .title(title)
            .border_type(BorderType::Rounded);

        block.render(area, buf);
    }

    pub fn render_shortcut_mode(&mut self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Shortcut Mode ");
        let block = Block::bordered()
            .title(title)
            .border_type(BorderType::Rounded);

        block.render(area, buf);
    }

    fn render_service_details(&mut self, area: Rect, buf: &mut Buffer, service: &Service) {
        let block = Block::bordered()
            .border_type(BorderType::Double)
            .title_alignment(HorizontalAlignment::Center)
            .title("[ [ SERVICE DETAILS ] ]");

        let mut service_details = vec![Line::from(format!(" {} ", service.name.clone()))];

        if let Some(url) = &service.url {
            service_details.push(Line::from(format!(" {} ", url.clone())));
        }

        let header = Paragraph::new(service_details).block(block);

        header.render(area, buf);
    }

    fn render_account_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .border_type(BorderType::Double)
            .title_alignment(HorizontalAlignment::Center)
            .title("[ [ ACCOUNTS ] ]");

        let accounts: Vec<ListItem> = self
            .accounts
            .list
            .iter()
            .map(|account| ListItem::new(Line::from(account.username.clone())))
            .collect();

        let account_list = List::new(accounts)
            .highlight_symbol(" > ")
            .highlight_style(
                Style::new()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED),
            )
            .highlight_spacing(HighlightSpacing::Always)
            .block(block);

        StatefulWidget::render(account_list, area, buf, &mut self.accounts.state);
    }

    fn render_account_details(&self, area: Rect, buf: &mut Buffer) {
        let details_block = Block::bordered()
            .border_type(BorderType::Double)
            .title_alignment(HorizontalAlignment::Center)
            .title("[ [ ACCOUNT DETAILS ] ]")
            .padding(Padding::left(1));

        if self.accounts.list.is_empty() {
            Widget::render(
                self.construct_empty_accounts_alert().block(details_block),
                area,
                buf,
            );
        } else {
            Widget::render(
                self.construct_account_details().block(details_block),
                area,
                buf,
            );
        }
    }

    fn construct_account_details(&self) -> List<'_> {
        let mut lines = vec![];

        let account = &self.accounts.list[self
            .accounts
            .state
            .selected()
            .expect("No account selected.")];

        if !account.username.is_empty() {
            lines.push(Line::from(vec![
                Span::raw(format!("{:.<width$}", "Username", width = 15)),
                account.username.clone().into(),
            ]));
        }

        if let Some(email) = &account.email {
            lines.push(Line::from(vec![
                Span::raw(format!("{:.<width$}", "Email", width = 15)),
                email.into(),
            ]));
        }

        if let Some(_password) = &account.password {
            lines.push(Line::from(vec![
                Span::raw(format!("{:.<width$}", "Password", width = 15)),
                format!("{{{}}}", "*").into(),
            ]));
        }

        if let Some(access_token) = &account.access_token
            && !access_token.is_empty()
        {
            lines.push(Line::from(vec![
                Span::raw(format!("{:.<width$}", "Access Token", width = 15)),
                access_token.into(),
            ]));
        }

        if let Some(pin) = &account.pin {
            lines.push(Line::from(vec![
                Span::raw(format!("{:.<width$}", "PIN", width = 15)),
                pin.into(),
            ]));
        }

        if let Some(passcode) = &account.passcode {
            lines.push(Line::from(vec![
                Span::raw(format!("{:.<width$}", "Passcode", width = 15)),
                passcode.into(),
            ]));
        }

        if !account.last_change.is_empty() {
            lines.push(Line::from(vec![
                Span::raw(format!("{:.<width$}", "Last Change", width = 15)),
                Span::raw(account.last_change.clone()),
            ]));
        }

        if !account.creation_date.is_empty() {
            lines.push(Line::from(vec![
                Span::raw(format!("{:.<width$}", "Account Created", width = 15)),
                Span::raw(account.creation_date.clone()),
            ]));
        }

        List::new(lines)
    }

    fn construct_empty_accounts_alert(&self) -> List<'_> {
        let lines = vec![
            Line::from("No accounts found for this service"),
            Line::from("Press 'n' to add a new one"),
        ];

        List::new(lines)
    }

    fn construct_service_list(&self) -> List<'static> {
        let list_items: Vec<ListItem> = self
            .services
            .list
            .iter()
            .map(|service| ListItem::new(Line::from(service.name.clone())))
            .collect();

        List::new(list_items)
            .highlight_symbol(" > ")
            .highlight_style(
                Style::new()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED),
            )
            .highlight_spacing(HighlightSpacing::Always)
    }

    fn construct_empty_services_alert(&self) -> List<'_> {
        let lines = vec![
            Line::from("No services found in the database"),
            Line::from("Press 'n' to add a new one"),
        ];

        List::new(lines)
    }
}
