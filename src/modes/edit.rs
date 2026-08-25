use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::{Line, Span},
    widgets::{
        Block, BorderType, HighlightSpacing, List, ListItem, ListState, Padding, StatefulWidget,
        Widget,
    },
};

use crate::models::{Field, Target};

#[derive(Debug)]
pub struct Edit {
    pub target: Target,
    pub list: Vec<Field>,
    pub state: ListState,
    pub input: Option<String>,
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
            KeyCode::Enter => {
                // generate new target from fields & old target struct 
                // can fail if missing fields
                // submit, but if this fails due to duplicate entry in db we need to reset somehow
                let mut target = self.target.clone();
                for field in &self.list {
                    (field.apply)(&mut target, &field.value).expect("Failed to apply fields.");
                }
                EditAction::Submit(target)
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
}

impl Widget for &mut Edit {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let selected = self.state.selected();

        let title = Line::from(" Edit Mode ");
        let block = Block::bordered()
            .title(title)
            .padding(Padding::uniform(1))
            .border_type(BorderType::Rounded);

        let fields: Vec<ListItem> = self
            .list
            .iter()
            .enumerate()
            .map(|(idx, field)| {
                let value: Vec<Span> = if Some(idx) == selected {
                    self.input.as_ref().map_or_else(
                        || vec![Span::from(format!("[ {} ]", field.value))],
                        |value| {
                            vec![
                                Span::raw(format!("[ {value}")),
                                Span::raw(" ").reversed(),
                                Span::raw(" ]"),
                            ]
                        },
                    )
                } else {
                    vec![format!("  {}", field.value).into()]
                };

                let mut line = Line::raw(format!("[ {: <width$}] ", field.label, width = 20));
                line.extend(value);

                let mut lines: Vec<Line> = vec![line];

                if let Some(error) = &field.error {
                    lines.push(Line::from(vec![
                        Span::raw(format!("> {: <width$}", "ERROR".to_string(), width = 24))
                            .italic(),
                        Span::raw(error).red().italic(),
                    ]));
                }

                ListItem::from(lines)
            })
            .collect();

        let list = List::new(fields)
            .highlight_symbol(" > ")
            .highlight_spacing(HighlightSpacing::Always)
            .repeat_highlight_symbol(true)
            .block(block);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}
