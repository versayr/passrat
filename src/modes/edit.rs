use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer, layout::{Constraint, Layout, Rect}, style::Stylize, text::{Line, Span}, widgets::{
        Block, BorderType, HighlightSpacing, List, ListState, Padding, StatefulWidget, Widget,
    },
};
use rustpass::PassphraseConfig;

use crate::{
    helpers::{construct_field_list, gen_password},
    models::{Field, Target},
};

#[derive(Debug)]
pub struct Edit {
    pub target: Target,
    pub list: Vec<Field>,
    pub state: ListState,
    pub input: Option<String>,
    error: Option<String>,
}

#[derive(Debug)]
pub enum EditAction {
    Submit(Target),
    Paste,
    Return,
    Help,
    Quit,
    None,
}

impl Edit {
    pub fn new(target: Target, list: Vec<Field>) -> Self {
        let mut state = ListState::default();
        state.select_first();
        Self {
            target,
            list,
            state,
            input: None,
            error: None,
        }
    }

    pub fn handle_inputs(&mut self, event: KeyEvent) -> EditAction {
        if self.input.is_some() {
            self.handle_edit_inputs(event);
            return EditAction::None;
        }

        match event.code {
            KeyCode::Char('h' | '?') => EditAction::Help,
            KeyCode::Esc | KeyCode::Char('q') => EditAction::Return,
            KeyCode::Char('j') | KeyCode::Down => {
                self.state.select_next();
                EditAction::None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.state.select_previous();
                EditAction::None
            }
            KeyCode::Char('e' | 'i' | 'c') => {
                if let Some(idx) = self.state.selected() {
                    let field = self.list.get(idx).expect("Failed to index into list.");
                    self.input = Some(field.value.clone());
                }
                EditAction::None
            }
            KeyCode::Char('p') => {
                self.input = Some(String::new());
                EditAction::Paste
            }
            KeyCode::Char('P') => {
                if let Some(idx) = self.state.selected() {
                    let field = self.list.get(idx).expect("Failed to index into list.");
                    if field.label == "Password" {
                        self.input = Some(
                            gen_password(PassphraseConfig::default())
                                .expect("Failed to generate password."),
                        );
                    }
                }
                EditAction::None
            }
            KeyCode::Enter => {
                let mut target = self.target.clone();
                for field in &self.list {
                    (field.apply)(&mut target, &field.value).expect("Failed to apply fields.");
                }
                if let Err(error) = target.validate() {
                    self.error = Some(error);
                    EditAction::None
                } else {
                    EditAction::Submit(target)
                }
            }
            _ => EditAction::None,
        }
    }

    fn handle_edit_inputs(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Esc => {
                if let Some(idx) = self.state.selected() {
                    let field = self.list.get_mut(idx).expect("Failed to index into list.");
                    field.error = None;
                }
                self.input = None;
            }
            KeyCode::Enter => self.handle_validation(),
            KeyCode::Backspace => {
                self.input
                    .as_mut()
                    .expect("Input string does not exist.")
                    .pop();
            }
            KeyCode::Char(ch) => {
                self.input
                    .as_mut()
                    .expect("Input string does not exist.")
                    .push(ch);
            }
            _ => {}
        }
    }

    fn handle_validation(&mut self) {
        if let Some(idx) = self.state.selected() {
            let value = self.input.as_ref().expect("No input is set.").clone();
            let is_valid = self
                .list
                .get(idx)
                .expect("Failed to access field.")
                .validator
                .as_ref()
                .map_or(Ok(()), |validator| validator.validate(&value));

            match is_valid {
                Ok(()) => {
                    let field = self.list.get_mut(idx).expect("Failed to get field.");
                    field.value = value;
                    field.error = None;
                    self.input = None;
                }
                Err(error) => {
                    let field = self.list.get_mut(idx).expect("Failed to get field.");
                    field.error = Some(error);
                }
            }
        }
    }

    fn render_error(&self, area: Rect, buf: &mut Buffer) {
        if let Some(error) = &self.error {
            let line = Line::from(vec![
                Span::raw("     Error: "),
                Span::raw(error),
            ]).red().bold();

            Widget::render(line, area, buf);
        }
    }
}

impl Widget for &mut Edit {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from(" Edit Mode ");
        let block = Block::bordered()
            .title(title)
            .padding(Padding::uniform(1))
            .border_type(BorderType::Rounded);

        let error_height = u16::from(self.error.is_some());

        let layout = Layout::default()
            .constraints(vec![Constraint::Length(error_height), Constraint::Fill(1)])
            .split(Block::inner(&block, area));

        let error_area = layout.first().expect("Malformed layout.");
        let fields_area = layout.get(1).expect("Malformed layout.");

        let input_binding = self.input.clone();
        let list = List::new(construct_field_list(
            &self.list,
            self.state.selected(),
            input_binding.as_ref(),
        ))
        .highlight_symbol(" > ")
        .highlight_spacing(HighlightSpacing::Always)
        .repeat_highlight_symbol(true);

        self.render_error(*error_area, buf);
        StatefulWidget::render(list, *fields_area, buf, &mut self.state);
        block.render(area, buf);
    }
}
