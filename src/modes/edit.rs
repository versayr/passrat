use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::Line,
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
    input: Option<String>,
}

#[derive(Debug)]
pub enum EditAction {
    Submit(Target),
    // Paste(String),
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
            match event.code {
                KeyCode::Esc => self.input = None,
                KeyCode::Enter => {
                    if let Some(idx) = self.state.selected() {
                        let value = self.input.as_ref().expect("No input is set.").clone();
                        self.list
                            .get_mut(idx)
                            .expect("Failed to write new value to field list.")
                            .value = value;
                    }
                    self.input = None;
                }
                KeyCode::Backspace => {
                    self.input
                        .as_mut()
                        .expect("Input string does not exist.")
                        .pop();
                }
                KeyCode::Char(c) => {
                    self.input
                        .as_mut()
                        .expect("Input string does not exist.")
                        .push(c);
                }
                _ => {}
            }

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
            KeyCode::Char('e' | 'i') => {
                if let Some(idx) = self.state.selected() {
                    let field = self.list.get(idx).expect("Failed to index into list.");
                    self.input = Some(field.value.clone());
                }
                EditAction::None
            }
            KeyCode::Enter => EditAction::Submit(self.target.clone()),
            _ => EditAction::None,
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
                let value = if Some(idx) == selected {
                    self.input.as_ref().map_or_else(
                        || format!("[ {} ]", field.value),
                        |value| format!("[ {value}| ]"),
                    )
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
            .highlight_spacing(HighlightSpacing::Always)
            .block(block);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}
