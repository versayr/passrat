use std::vec;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::Line,
    widgets::{
        Block, BorderType, HighlightSpacing, List, ListItem, Padding, StatefulWidget,
        Widget,
    },
};

use crate::{
    app::Mode::Edit,
    App,
};

impl App {
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
            .padding(Padding::uniform(1))
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

//     pub fn render_view_mode(&mut self, area: Rect, buf: &mut Buffer) {
//         let View(state) = &mut self.mode else { return };
//         let empty_list = state.accounts.list.is_empty();
// 
//         let title = Line::from(" View Mode ");
//         let block = Block::bordered()
//             .title(title)
//             .border_type(BorderType::Rounded);
// 
//         let main_layout = Layout::default()
//             .direction(Direction::Vertical)
//             .constraints(vec![Constraint::Length(4), Constraint::Fill(1)])
//             .split(Block::inner(&block, area));
// 
//         let body_layout = Layout::default()
//             .direction(Direction::Horizontal)
//             .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
//             .split(main_layout[1]);
// 
//         self.render_service_details(main_layout[0], buf);
//         self.render_account_list(body_layout[0], buf);
//         if empty_list {
//             render_empty_accounts_alert(body_layout[1], buf);
//         } else {
//             self.render_account_details(body_layout[1], buf);
//         }
// 
//         block.render(area, buf);
//     }

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
