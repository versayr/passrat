use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::{Block, BorderType, Widget},
};

use crate::App;

impl App {
    #[allow(clippy::unused_self)]
    pub fn render_help_mode(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Help Mode ");
        let block = Block::bordered()
            .title(title)
            .border_type(BorderType::Rounded);

        block.render(area, buf);
    }

    #[allow(clippy::unused_self)]
    pub fn render_shortcut_mode(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Shortcut Mode ");
        let block = Block::bordered()
            .title(title)
            .border_type(BorderType::Rounded);

        block.render(area, buf);
    }
}
