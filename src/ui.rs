use std::vec;

use ratatui::{
    buffer::Buffer, layout::{Constraint, Direction, Layout, Rect}, style::{Modifier, Style}, text::{Line, Span}, widgets::{Block, BorderType, HighlightSpacing, List, ListItem, Padding, Paragraph, StatefulWidget, Widget},
};

use crate::App;

impl App {
    pub fn render_locked_mode(&mut self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Login Screen ");
        let block = Block::bordered()
            .title(title)
            .padding(Padding::uniform(1))
            .border_type(BorderType::Rounded);

        let input = Paragraph::new(vec![
            Line::from("Enter Password (Or Set Password If DB Doesn't Exist)"),
            Line::from(vec![
                Span::from("*".repeat(self.password.len())),
                Span::styled(" ", Style::reversed(Style::default())),
            ]),
            Line::from(self.alert.clone()),
        ]);

        input.render(Block::inner(&block, area), buf);
        block.render(area, buf);
    }

    pub fn render_list_mode(&mut self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" List Mode ");
        let block = Block::bordered()
            .title(title)
            .padding(Padding::uniform(1))
            .border_type(BorderType::Rounded);

        let list_items: Vec<ListItem> = self
            .services
            .list
            .iter()
            .map(|service| ListItem::new(Line::from(service.name.clone())))
            .collect();

        let list = List::new(list_items)
            .highlight_symbol(" > ")
            .highlight_style(Style::new().add_modifier(Modifier::BOLD))
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, Block::inner(&block, area), buf, &mut self.services.state);

        // list.render(Block::inner(&block, area), buf);
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

        let header_block = Block::bordered().border_type(BorderType::Double);

        let header = Paragraph::new(vec![
            Line::from("SERVICE NAME"),
            Line::from("service url (optional)"),
        ])
        .block(header_block);

        let account_block = Block::bordered().border_type(BorderType::Double);

        let account_list = List::new(vec![
            ListItem::from("Account 1"),
            ListItem::from("Account 2"),
        ])
        .highlight_symbol(">>")
        .highlight_spacing(HighlightSpacing::WhenSelected)
        .block(account_block);

        let details_block = Block::bordered().border_type(BorderType::Double);

        let detail_items = vec![
            Line::from("[ [ ACCOUNT DETAILS ] ]"),
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

        block.render(area, buf);
        header.render(main_layout[0], buf);
        StatefulWidget::render(account_list, body_layout[0], buf, &mut self.services.state);
        details.render(body_layout[1], buf);
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
}
