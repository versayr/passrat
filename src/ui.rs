use std::vec;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::Line,
    widgets::{
        Block, BorderType, HighlightSpacing, List, ListItem, Padding, StatefulWidget, Widget,
    },
};

use crate::App;

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
