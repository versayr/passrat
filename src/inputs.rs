use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io;

use crate::app::{App, Mode};

impl App {
    pub fn handle_events(&mut self) -> io::Result<()> {
        match event::read().expect("Failed to parse input.") {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_events(key_event);
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_key_events(&mut self, event: KeyEvent) {
        if self.locked {
            self.handle_locked_inputs(event);
        } else {
            match self.mode {
                Mode::List => self.handle_list_inputs(event),
                Mode::View => self.handle_view_inputs(event),
                Mode::Edit => self.handle_edit_inputs(event),
                Mode::Help => self.handle_help_inputs(event),
                Mode::Cuts => self.handle_shortcut_inputs(event),
            }
        }
    }

    fn handle_locked_inputs(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Esc => self.exit = true,
            KeyCode::Enter => self.submit_password().expect("Failed to submit password."),
            KeyCode::Backspace => {
                self.password.pop();
            }
            KeyCode::Char(char) => self.password.push(char),
            _ => {}
        }
    }

    fn handle_list_inputs(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('h') | KeyCode::Char('?') => self.mode = Mode::Help,
            KeyCode::Char('j') | KeyCode::Down => self.services.state.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.services.state.select_previous(),
            KeyCode::Char('e') => self.mode = Mode::Edit,
            KeyCode::Char('n') => self.add_service().expect("Failed to add service."),
            KeyCode::Char('\\') => self.mode = Mode::Cuts,
            KeyCode::Enter => {
                if !self.services.list.is_empty() {
                    self.selected_service = Some(
                        self.services.list[self
                        .services
                        .state
                        .selected()
                        .expect("No service is selected.")]
                        .clone(),
                    );
                    let _ = self.get_accounts();
                    self.mode = Mode::View;
                }
            }
            _ => {}
        }
    }

    fn handle_view_inputs(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('h') | KeyCode::Char('?') => self.mode = Mode::Help,
            KeyCode::Char('j') | KeyCode::Down => self.accounts.state.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.accounts.state.select_previous(),
            KeyCode::Esc => self.mode = Mode::List,
            KeyCode::Char('e') => self.mode = Mode::Edit,
            KeyCode::Char('n') => self.add_account().expect("Failed to add account."),
            _ => {}
        }
    }

    fn handle_edit_inputs(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('h') | KeyCode::Char('?') => self.mode = Mode::Help,
            KeyCode::Esc => self.mode = Mode::List,
            _ => {}
        }
    }

    fn handle_help_inputs(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Esc => self.mode = Mode::List,
            _ => {}
        }
    }

    fn handle_shortcut_inputs(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Esc => self.mode = Mode::List,
            _ => {}
        }
    }

}
