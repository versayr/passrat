use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::widgets::ListState;

use crate::{
    app::{
        AccountList, App, EditState, HomeState,
        Mode::{self, Lock, View},
        ViewState,
    },
    models::{Account, Target},
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
        match &self.mode {
            Mode::Lock(_) => self.handle_lock_inputs(event),
            Mode::Home(_) => self.handle_home_inputs(event),
            Mode::Edit(_) => self.handle_edit_inputs(event),
            Mode::View(_) => self.handle_view_inputs(event),
            Mode::Help => self.handle_help_inputs(event),
            Mode::Cuts => self.handle_shortcut_inputs(event),
        }
    }

    fn handle_lock_inputs(&mut self, event: KeyEvent) {
        let Lock(state) = &mut self.mode else { return };
        let mut password: Option<String> = None;

        match event.code {
            KeyCode::Esc => self.exit = true,
            KeyCode::Enter => password = Some(state.password.clone()),
            KeyCode::Backspace => {
                state.password.pop();
            }
            KeyCode::Char(char) => state.password.push(char),
            _ => {}
        }

        if let Some(s) = password {
            self.submit_password(&s);
        }
    }

    fn handle_home_inputs(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('h' | '?') => self.mode = Mode::Help,
            KeyCode::Char('j') | KeyCode::Down => self.services.state.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.services.state.select_previous(),
            KeyCode::Char('e') => {
                let service = self.services.list[self
                    .services
                    .state
                    .selected()
                    .expect("No service is selected.")]
                .clone();

                self.mode = Mode::Edit(EditState {
                    target: Target::Service(service),
                    list: vec![], // TODO supply this vec from target.fields()
                    state: ListState::default(),
                });
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

                    self.mode = Mode::View(ViewState {
                        service: service.clone(),
                        accounts: AccountList {
                            list: self.get_accounts(service.id.unwrap()).unwrap(),
                            state: ListState::default(),
                        },
                    });

                    if let View(state) = &mut self.mode {
                        state.accounts.state.select_first();
                    }
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

    fn handle_view_inputs(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('h' | '?') => self.mode = Mode::Help,
            KeyCode::Char('j') | KeyCode::Down => {
                if let Mode::View(state) = &mut self.mode {
                    state.accounts.state.select_next();
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if let Mode::View(state) = &mut self.mode {
                    state.accounts.state.select_previous();
                }
            }
            KeyCode::Esc => self.mode = Mode::Home(HomeState::default()),
            KeyCode::Char('e') => {
                let View(state) = &self.mode else { return };
                let account = state.accounts.list[state
                    .accounts
                    .state
                    .selected()
                    .expect("No account is selected.")]
                .clone();

                self.mode = Mode::Edit(EditState {
                    target: Target::Account(account),
                    list: vec![],
                    state: ListState::default(),
                });
            }
            KeyCode::Char('n') => {
                let state = EditState {
                    target: Target::Account(Account::default()),
                    list: vec![],
                    state: ListState::default(),
                };
                self.mode = Mode::Edit(state);
            }
            _ => {}
        }
    }

    fn handle_edit_inputs(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('h' | '?') => self.mode = Mode::Help,
            KeyCode::Esc => self.mode = Mode::Home(HomeState::default()),
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
