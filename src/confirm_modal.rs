use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    text::Line,
    widgets::{Block, BorderType, Padding, Paragraph, Widget},
};

#[derive(Debug, Default)]
pub struct Modal {
    label: String,
    confirm: bool,
}

impl Widget for &Modal {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from("[ [ CONFIRM ] ]");
        let block = Block::bordered()
            .title(title)
            .padding(Padding::uniform(1))
            .border_type(BorderType::Double);

        let label = Paragraph::new(self.label.clone()).block(block);

        let centered_area = area.centered(Constraint::Length(60), Constraint::Length(4));

        label.render(centered_area, buf);
    }
}
