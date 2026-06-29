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

impl App {
    pub fn render_locked_mode(&mut self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Login Screen ");
        let block = Block::bordered()
            .title(title)
            .padding(Padding::uniform(1))
            .border_type(BorderType::Rounded);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Length(6),
                Constraint::Fill(4),
            ])
            .split(Block::inner(&block, area));

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

        input.render(layout[1], buf);
        block.render(area, buf);
    }

    pub fn render_list_mode(&mut self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" List Mode ");
        let block = Block::bordered()
            .title(title)
            .padding(Padding::uniform(1))
            .border_type(BorderType::Rounded);

        self.render_service_list(Block::inner(&block, area), buf);

        block.render(area, buf);
    }

    pub fn render_edit_mode(&mut self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Edit Mode ");
        let block = Block::bordered()
            .title(title)
            .border_type(BorderType::Rounded);

        block.render(area, buf);
    }

    pub fn render_view_mode(&mut self, area: Rect, buf: &mut Buffer) {
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

    fn render_service_list(&mut self, area: Rect, buf: &mut Buffer) {
        let list_items: Vec<ListItem> = self
            .services
            .list
            .iter()
            .map(|service| ListItem::new(Line::from(service.name.clone())))
            .collect();

        let list = List::new(list_items)
            .highlight_symbol(" > ")
            .highlight_style(
                Style::new()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED),
            )
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.services.state);
    }

    fn render_service_details(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .border_type(BorderType::Double)
            .title_alignment(HorizontalAlignment::Center)
            .title("[ [ SERVICE DETAILS ] ]");

        let service = self
            .selected_service
            .as_ref()
            .expect("No service is selected.");

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

    fn render_account_details(&mut self, area: Rect, buf: &mut Buffer) {
        let details_block = Block::bordered()
            .border_type(BorderType::Double)
            .title_alignment(HorizontalAlignment::Center)
            .title("[ [ ACCOUNT DETAILS ] ]")
            .padding(Padding::left(1));

        let detail_items = vec![
            Line::from(vec![
                Span::raw(format!("{:.<width$}", "Username", width = 30)),
                Span::raw("NAME"),
            ]),
            Line::from(vec![
                Span::raw(format!("{:.<width$}", "Email", width = 30)),
                Span::raw("EMAIL"),
            ]),
            Line::from(vec![
                Span::raw(format!("{:.<width$}", "Password", width = 30)),
                Span::raw("{*}"),
            ]),
            Line::from(vec![
                Span::raw(format!("{:.<width$}", "Access Token", width = 30)),
                Span::raw("{*}"),
            ]),
            Line::from(vec![
                Span::raw(format!("{:.<width$}", "Security Questions", width = 30)),
                Span::raw("{*}"),
            ]),
            Line::from(vec![
                Span::raw(format!("{:.<width$}", "PIN", width = 30)),
                Span::raw("{*}"),
            ]),
            Line::from(vec![
                Span::raw(format!("{:.<width$}", "Passcode", width = 30)),
                Span::raw("{*}"),
            ]),
            Line::from(vec![
                Span::raw(format!("{:.<width$}", "Account Created", width = 30)),
                Span::raw("DATE"),
            ]),
            Line::from(vec![
                Span::raw(format!("{:.<width$}", "Last Change", width = 30)),
                Span::raw("DATE"),
            ]),
            Line::from("[ [ SHORTCUTS ] ]"),
            Line::from(vec![
                Span::raw(format!("{:width$}", "Password", width = 30)),
                Span::raw("xk"),
            ]),
            Line::from(vec![
                Span::raw(format!("{:width$}", "Access Token", width = 30)),
                Span::raw("xl"),
            ]),
        ];

        let details = Paragraph::new(detail_items).block(details_block);

        details.render(area, buf);
    }
}
