use crossterm::event::{KeyCode, KeyEvent};
use jiff::Zoned;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, HighlightSpacing, List, ListState, Padding, Paragraph,
        StatefulWidget, Widget,
    },
};
use rustpass::PassphraseConfig;

use crate::{
    helpers::{construct_field_list, gen_password},
    models::{Field, Target},
};

#[derive(Debug, PartialEq, Eq)]
pub struct Edit {
    pub target: Target,
    pub list: Vec<Field>,
    pub state: ListState,
    pub input: Option<Input>,
    pub error: Option<String>,
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct Input {
    pub index: usize,
    pub value: String,
}

impl Input {
    const fn from(value: String) -> Self {
        Self {
            index: value.len(),
            value,
        }
    }

    const fn backward(&mut self) {
        if self.index > 0 {
            self.index = self.index.saturating_sub(1);
        }
    }

    const fn forward(&mut self) {
        if self.index < self.value.len() {
            self.index = self.index.saturating_add(1);
        }
    }

    fn delete(&mut self) {
        if self.index < self.value.len() {
            self.value.remove(self.index);
        }
    }

    fn backspace(&mut self) {
        if self.index > 0 {
            let idx = self.index.saturating_sub(1);
            self.value.remove(idx);
            self.index = self.index.saturating_sub(1);
        }
    }

    fn insert(&mut self, ch: char) {
        self.value.insert(self.index, ch);
        self.index = self.index.saturating_add(1);
    }
}

#[derive(Debug)]
pub enum EditAction {
    Submit(Target),
    Copy(String),
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
            KeyCode::Char('?') => EditAction::Help,
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
                    self.input = Some(Input::from(field.value.clone()));
                }
                EditAction::None
            }
            KeyCode::Char('p') => {
                self.input = Some(Input::default());
                EditAction::Paste
            }
            KeyCode::Char('P') => {
                if let Some(idx) = self.state.selected() {
                    let password = gen_password(PassphraseConfig::default())
                        .expect("Failed to generate password.");
                    let field = self.list.get(idx).expect("Failed to index into list.");
                    if field.label == "Password" {
                        self.input = Some(Input::from(password));
                    }
                }
                EditAction::None
            }
            KeyCode::Char('y') => {
                if let Some(idx) = self.state.selected() {
                    let field = self
                        .list
                        .get(idx)
                        .expect("Failed to index into list of fields.");
                    EditAction::Copy(field.value.clone())
                } else {
                    EditAction::None
                }
            }
            KeyCode::Enter => {
                let mut target = self.target.clone();
                for field in &self.list {
                    field.apply(&mut target).expect("Failed to apply fields.");
                }
                if let Err(error) = target.validate() {
                    self.error = Some(error);
                    EditAction::None
                } else {
                    if let Target::Account(ref mut account) = target {
                        account.last_change = Zoned::now().into();
                    }
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
            KeyCode::Enter => {
                if let Some(ref mut input) = self.input {
                    input.value = input.value.trim().to_string();
                    input.index = input.value.len();
                }
                self.handle_validation();
            }
            KeyCode::Backspace => {
                self.input
                    .as_mut()
                    .expect("Input string does not exist.")
                    .backspace();
            }
            KeyCode::Delete => {
                self.input
                    .as_mut()
                    .expect("Input string does not exist.")
                    .delete();
            }
            KeyCode::Char(ch) => {
                self.input
                    .as_mut()
                    .expect("Input string does not exist.")
                    .insert(ch);
            }
            KeyCode::Left => {
                self.input
                    .as_mut()
                    .expect("Input string does not exist.")
                    .backward();
            }
            KeyCode::Right => {
                self.input
                    .as_mut()
                    .expect("Input string does not exist.")
                    .forward();
            }
            _ => {}
        }
    }

    fn handle_validation(&mut self) {
        if let Some(idx) = self.state.selected() {
            let value = self.input.as_ref().expect("No input is set.").value.clone();
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
            let block = Block::bordered()
                .border_type(BorderType::LightQuadrupleDashed)
                .borders(Borders::BOTTOM);

            let line = Line::from(vec![
                Span::raw(" > Form Submission Error: "),
                Span::raw(error),
            ])
            .red()
            .bold();

            let text = Paragraph::new(line).block(block);
            Widget::render(text, area, buf);
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

        let error_height = match &self.error.is_some() {
            true => 2,
            false => 0,
        };

        let layout = Layout::default()
            .constraints(vec![Constraint::Length(error_height), Constraint::Fill(1)]);

        let [error_area, fields_area] = Layout::areas(&layout, Block::inner(&block, area));

        let input_binding = self.input.clone();
        let list = List::new(construct_field_list(
            &self.list,
            self.state.selected(),
            input_binding.as_ref(),
        ))
        .highlight_symbol(" > ")
        .highlight_spacing(HighlightSpacing::Always)
        .repeat_highlight_symbol(true);

        self.render_error(error_area, buf);
        StatefulWidget::render(list, fields_area, buf, &mut self.state);
        block.render(area, buf);
    }
}
