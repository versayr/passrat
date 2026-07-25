use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::{
    app::{
        App, HomeState,
        Mode::{self, Help, Home},
    },
    models::Service,
    modes::{
        edit::{Edit, EditAction},
        lock::LockAction::{self},
        view::{View, ViewAction},
    },
};

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
        match &mut self.mode {
            Mode::Lock(lock) => {
                let mut password = None;

                match lock.handle_inputs(event) {
                    LockAction::Quit => self.exit = true,
                    LockAction::SubmitPassword => password = Some(lock.input.clone()),
                    LockAction::None => {}
                }

                if let Some(password) = password {
                    self.submit_password(&password);
                }
            }
            Mode::Home(_) => self.handle_home_inputs(event),
            Mode::View(view) => match view.handle_inputs(event) {
                ViewAction::Edit(account) => self.mode = Mode::Edit(Edit::new(&account)),
                ViewAction::Return => self.mode = Home(HomeState::default()),
                ViewAction::Help => self.mode = Help,
                ViewAction::Quit => self.exit = true,
                ViewAction::None => {}
            },
            Mode::Edit(edit) => match edit.handle_inputs(event) {
                EditAction::Return => self.mode = Home(HomeState::default()),
                EditAction::Quit => self.exit = true,
                EditAction::Help => self.mode = Help,
                EditAction::None => {}
            },
            Mode::Help => self.handle_help_inputs(event),
            Mode::Cuts => self.handle_shortcut_inputs(event),
        }
    }

    fn handle_home_inputs(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('h' | '?') => self.mode = Mode::Help,
            KeyCode::Char('j') | KeyCode::Down => self.services.state.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.services.state.select_previous(),
            KeyCode::Char('e') => {
                let service = &self.services.list[self
                    .services
                    .state
                    .selected()
                    .expect("No service is selected.")];
                self.mode = Mode::Edit(Edit::new(service));
            }
            KeyCode::Char('n') => {
                self.mode = Mode::Edit(Edit::new(&Service::default()));
            }
            KeyCode::Char('\\') => self.mode = Mode::Cuts,
            KeyCode::Enter => {
                if !self.services.list.is_empty() {
                    let service = self.services.list[self
                        .services
                        .state
                        .selected()
                        .expect("No service is selected.")]
                    .clone();
                    let list = self.get_accounts(service.id.expect("Unable to get service's account list without a row id for the service.")).expect("Unable to get list of accounts.");
                    self.mode = Mode::View(View::new(&service, list));
                }
            }
            KeyCode::Char('y') => {
                let service = self.services.list[self
                    .services
                    .state
                    .selected()
                    .expect("No service is selected.")]
                .clone();
                self.clipboard.set_text(service.name).unwrap();
            }
            _ => {}
        }
    }

    fn handle_help_inputs(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Esc => self.mode = Mode::Home(HomeState::default()),
            _ => {}
        }
    }

    fn handle_shortcut_inputs(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Esc => self.mode = Mode::Home(HomeState::default()),
            _ => {}
        }
    }
}
