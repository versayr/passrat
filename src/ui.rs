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

use crate::{
    App,
    app::Mode::{Edit, Lock, View},
    helpers::{construct_detail_field, format_current_date},
};

impl App {
    pub fn render_lock_mode(&mut self, area: Rect, buf: &mut Buffer) {
        let Lock(state) = &mut self.mode else { return };
        let password = state.password.clone();
        let alert = state.alert.clone();

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
                Span::from("*".repeat(password.len())),
                Span::styled(" ", Style::reversed(Style::default())),
            ]),
            Line::from(alert),
        ])
        .block(input_block);

        input.render(input_area, buf);
        block.render(area, buf);
    }

    pub fn render_home_mode(&mut self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Home Mode ");
        let block = Block::bordered()
            .title(title)
            .padding(Padding::uniform(1))
            .border_type(BorderType::Rounded);

        if self.services.list.is_empty() {
            Widget::render(
                construct_empty_services_alert(),
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

    #[allow(clippy::unused_self)]
    pub fn render_edit_mode(&mut self, area: Rect, buf: &mut Buffer) {
        let Edit(state) = &mut self.mode else { return };
        let selected = state.state.selected();

        let title = Line::from(" Edit Mode ");
        let block = Block::bordered()
            .title(title)
            .border_type(BorderType::Rounded);

        let fields: Vec<ListItem> = state
            .list
            .iter()
            .enumerate()
            .map(|(idx, field)| {
                let value = if Some(idx) == selected {
                    format!("[ {} ]", field.value)
                } else {
                    format!("  {}  ", field.value)
                };

                ListItem::from(Line::from(vec![
                    format!("[ {: <width$}] ", field.label, width = 20).into(),
                    value.into(),
                ]))
            })
            .collect();

        let list = List::new(fields)
            .highlight_symbol(" > ")
            .highlight_style(
                Style::new()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED),
            )
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, Block::inner(&block, area), buf, &mut state.state);
        block.render(area, buf);
    }

    pub fn render_view_mode(&mut self, area: Rect, buf: &mut Buffer) {
        let View(state) = &mut self.mode else { return };
        let empty_list = state.accounts.list.is_empty();

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

        self.render_service_details(main_layout[0], buf);
        self.render_account_list(body_layout[0], buf);
        if empty_list {
            render_empty_accounts_alert(body_layout[1], buf);
        } else {
            self.render_account_details(body_layout[1], buf);
        }

        block.render(area, buf);
    }

    #[allow(clippy::unused_self)]
    pub fn render_help_mode(&mut self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Help Mode ");
        let block = Block::bordered()
            .title(title)
            .border_type(BorderType::Rounded);

        block.render(area, buf);
    }

    #[allow(clippy::unused_self)]
    pub fn render_shortcut_mode(&mut self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Shortcut Mode ");
        let block = Block::bordered()
            .title(title)
            .border_type(BorderType::Rounded);

        block.render(area, buf);
    }

    fn render_service_details(&mut self, area: Rect, buf: &mut Buffer) {
        let View(state) = &mut self.mode else { return };
        let service = state.service.clone();

        let block = Block::bordered()
            .border_type(BorderType::Double)
            .title_alignment(HorizontalAlignment::Center)
            .title("[ [ SERVICE DETAILS ] ]");

        let service_details = vec![
            Line::from(format!(" {} ", service.name.clone())),
            Line::from(format!(" {} ", service.url.clone())),
        ];

        let header = Paragraph::new(service_details).block(block);

        header.render(area, buf);
    }

    fn render_account_list(&mut self, area: Rect, buf: &mut Buffer) {
        let View(state) = &mut self.mode else { return };
        let accounts = &state.accounts.clone();

        let block = Block::bordered()
            .border_type(BorderType::Double)
            .title_alignment(HorizontalAlignment::Center)
            .title("[ [ ACCOUNTS ] ]");

        let accounts: Vec<ListItem> = accounts
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

        StatefulWidget::render(account_list, area, buf, &mut state.accounts.state);
    }

    fn render_account_details(&mut self, area: Rect, buf: &mut Buffer) {
        let View(state) = &mut self.mode else { return };
        let selected_idx = state
            .accounts
            .state
            .selected()
            .expect("No account is selected.");
        let account = state.accounts.list[selected_idx].clone();

        let details_block = Block::bordered()
            .border_type(BorderType::Double)
            .title_alignment(HorizontalAlignment::Center)
            .title("[ [ ACCOUNT DETAILS ] ]")
            .padding(Padding::left(1));

        let mut lines = vec![];

        if !account.username.is_empty() {
            lines.push(construct_detail_field("Username", &account.username, 17));
        }

        if !account.email.is_empty() {
            lines.push(construct_detail_field("Email", &account.email, 17));
        }

        if !account.password.is_empty() {
            lines.push(construct_detail_field("Password", "{*}", 17));
        }

        if !account.access_token.is_empty() {
            lines.push(construct_detail_field(
                "Access Token",
                &account.access_token,
                17,
            ));
        }

        if let Some(pin) = account.pin {
            lines.push(construct_detail_field("PIN", &pin.to_string(), 17));
        }

        if let Some(passcode) = account.passcode {
            lines.push(construct_detail_field(
                "Passcode",
                &passcode.to_string(),
                17,
            ));
        }

        lines.push(construct_detail_field(
            "Last Change",
            &format_current_date(account.last_change),
            17,
        ));
        lines.push(construct_detail_field(
            "Account Created",
            &format_current_date(account.creation_date),
            17,
        ));

        Widget::render(List::new(lines).block(details_block), area, buf);
    }

    fn construct_service_list(&self) -> List<'static> {
        let list_items: Vec<ListItem> = self
            .services
            .list
            .iter()
            // .filter(|service| service.name.contains(&self.input))
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
}

fn construct_empty_services_alert() -> List<'static> {
    let lines = vec![
        Line::from("No services found in the database"),
        Line::from("Press 'n' to add a new one"),
    ];

    List::new(lines)
}

fn render_empty_accounts_alert(area: Rect, buf: &mut Buffer) {
    let details_block = Block::bordered()
        .border_type(BorderType::Double)
        .title_alignment(HorizontalAlignment::Center)
        .title("[ [ ACCOUNT DETAILS ] ]")
        .padding(Padding::left(1));

    let lines = vec![
        Line::from("No accounts found for this service"),
        Line::from("Press 'n' to add a new one"),
    ];

    Widget::render(List::new(lines).block(details_block), area, buf);
}
