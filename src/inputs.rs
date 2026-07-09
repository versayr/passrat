use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::app::{App, Mode};

impl App {
    pub fn handle_events(&mut self) {
        match event::read().expect("Failed to parse input.") {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_events(key_event);
            }
            _ => {}
        }
    }

    fn handle_key_events(&mut self, event: KeyEvent) {
        match self.mode {
            Mode::Lock => self.handle_lock_inputs(event),
            Mode::List => self.handle_list_inputs(event),
            Mode::Help => self.handle_help_inputs(event),
            Mode::Cuts => self.handle_shortcut_inputs(event),
            Mode::Edit(_) => self.handle_edit_inputs(event),
            Mode::View(_) => self.handle_view_inputs(event),
        }
    }

    fn handle_lock_inputs(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Esc => self.exit = true,
            KeyCode::Enter => self.submit_password(),
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
            KeyCode::Char('h' | '?') => self.mode = Mode::Help,
            KeyCode::Char('j') | KeyCode::Down => self.services.state.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.services.state.select_previous(),
            KeyCode::Char('e') => {
                // let target = self.services.list[self
                //     .services
                //     .state
                //     .selected()
                //     .expect("No service is selected.")]
                // .clone();
                // self.mode = Mode::Edit(target);
                todo!("Set edit target: Service.");
            }
            KeyCode::Char('n') => self.add_service(),
            KeyCode::Char('\\') => self.mode = Mode::Cuts,
            KeyCode::Enter => {
                if !self.services.list.is_empty() {
                    let service = self.services.list[self
                        .services
                        .state
                        .selected()
                        .expect("No service is selected.")]
                    .clone();
                    let _ = self.get_accounts();
                    self.mode = Mode::View(service);
                }
            }
            _ => {}
        }
    }

    fn handle_view_inputs(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('h' | '?') => self.mode = Mode::Help,
            KeyCode::Char('j') | KeyCode::Down => self.accounts.state.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.accounts.state.select_previous(),
            KeyCode::Esc => self.mode = Mode::List,
            KeyCode::Char('e') => {
                // self.mode = Mode::Edit
                todo!("Set edit target: Account or Service.")
            }
            KeyCode::Char('n') => self.add_account(),
            _ => {}
        }
    }

    fn handle_edit_inputs(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('h' | '?') => self.mode = Mode::Help,
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
