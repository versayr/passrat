use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, BorderType, Padding, Paragraph, Widget},
};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Lock {
    pub input: String,
    pub alert: String,
}

#[derive(Debug)]
pub enum LockAction {
    SubmitPassword,
    Quit,
    None,
}

impl Lock {
    pub const fn new(input: String, alert: String) -> Self {
        Self { input, alert }
    }

    pub fn handle_inputs(&mut self, event: KeyEvent) -> LockAction {
        match event.code {
            KeyCode::Esc => LockAction::Quit,
            KeyCode::Enter => LockAction::SubmitPassword,
            KeyCode::Backspace => {
                self.input.pop();
                LockAction::None
            }
            KeyCode::Char(char) => {
                self.input.push(char);
                LockAction::None
            }
            _ => LockAction::None,
        }
    }
}

impl Widget for &Lock {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
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
                Span::from("*".repeat(self.input.len())),
                " ".slow_blink().reversed(),
            ]),
            Line::from(Span::from(&self.alert)),
        ])
        .block(input_block);

        input.render(input_area, buf);
        block.render(area, buf);
    }
}
