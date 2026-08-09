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
}

#[derive(Debug)]
pub enum EditAction {
    Submit(Target),
    // Copy(String),
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
        }
    }

    pub fn handle_inputs(&mut self, event: KeyEvent) -> EditAction {
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
            .highlight_spacing(HighlightSpacing::Always)
            .block(block);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}
