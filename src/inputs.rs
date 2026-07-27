use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::{
    app::{
        App,
        Mode::{self, Help},
    },
    forms::Fields,
    models::Target,
    modes::{
        edit::{Edit, EditAction},
        home::{Home, HomeAction},
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
            Mode::Home(home) => match home.handle_inputs(event) {
                HomeAction::View(service) => {
                    let list = self.get_accounts(service.id.expect("Unable to get service's account list without a row id for the service.")).expect("Unable to get list of accounts.");
                    self.mode = Mode::View(View::new(&service, list));
                }
                HomeAction::Edit(service) => {
                    self.mode = Mode::Edit(Edit::new(
                        Target::Service(service.clone()),
                        service.fields(),
                    ));
                }
                HomeAction::Copy(str) => {
                    self.clipboard.set_text(str).unwrap();
                }
                HomeAction::Quit => self.exit = true,
                HomeAction::Help => self.mode = Mode::Help,
                HomeAction::None => {}
            },
            Mode::View(view) => match view.handle_inputs(event) {
                ViewAction::Edit(account) => {
                    self.mode = Mode::Edit(Edit::new(
                        Target::Account(account.clone()),
                        account.fields(),
                    ));
                }
                ViewAction::Return => {
                    let list = self.get_services().expect("Failed to get services.");
                    self.mode = Mode::Home(Home::new(list));
                }
                ViewAction::Copy(str) => self.clipboard.set_text(str).unwrap(),
                ViewAction::Help => self.mode = Help,
                ViewAction::Quit => self.exit = true,
                ViewAction::None => {}
            },
            Mode::Edit(edit) => match edit.handle_inputs(event) {
                EditAction::Return => {
                    let list = self.get_services().expect("Failed to get services.");
                    self.mode = Mode::Home(Home::new(list));
                }
                EditAction::Quit => self.exit = true,
                EditAction::Help => self.mode = Help,
                EditAction::None => {}
                EditAction::Submit(target) => {
                    // TODO perhaps confirm/discard here?
                    self.handle_target(&target);
                    // TODO send to previous mode (store on app? Option<Mode> ?)
                    let list = self.get_services().expect("Failed to get services.");
                    self.mode = Mode::Home(Home::new(list));
                }
            },
            Mode::Help => self.handle_help_inputs(event),
            Mode::Cuts => self.handle_shortcut_inputs(event),
        }
    }

    //     fn handle_home_inputs(&mut self, event: KeyEvent) {
    //         match event.code {
    //             KeyCode::Esc | KeyCode::Char('q') => self.exit = true,
    //             KeyCode::Char('h' | '?') => self.mode = Mode::Help,
    //             KeyCode::Char('j') | KeyCode::Down => self.services.state.select_next(),
    //             KeyCode::Char('k') | KeyCode::Up => self.services.state.select_previous(),
    //             KeyCode::Char('e') => {
    //                 let service = &self.services.list[self
    //                     .services
    //                     .state
    //                     .selected()
    //                     .expect("No service is selected.")];
    //                 self.mode = Mode::Edit(Edit::new(
    //                     Target::Service(service.clone()),
    //                     service.fields(),
    //                 ));
    //             }
    //             KeyCode::Char('n') => {
    //                 let service = Service::default();
    //                 self.mode = Mode::Edit(Edit::new(
    //                     Target::Service(service.clone()),
    //                     service.fields(),
    //                 ));
    //             }
    //             KeyCode::Char('\\') => self.mode = Mode::Cuts,
    //             KeyCode::Enter => {
    //                 if !self.services.list.is_empty() {
    //                     let service = self.services.list[self
    //                         .services
    //                         .state
    //                         .selected()
    //                         .expect("No service is selected.")]
    //                     .clone();
    //                     let list = self.get_accounts(service.id.expect("Unable to get service's account list without a row id for the service.")).expect("Unable to get list of accounts.");
    //                     self.mode = Mode::View(View::new(&service, list));
    //                 }
    //             }
    //             KeyCode::Char('y') => {
    //                 let service = self.services.list[self
    //                     .services
    //                     .state
    //                     .selected()
    //                     .expect("No service is selected.")]
    //                 .clone();
    //                 self.clipboard.set_text(service.name).unwrap();
    //             }
    //             _ => {}
    //         }
    //     }

    fn handle_help_inputs(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Esc => {
                let list = self.get_services().expect("Failed to get services.");
                self.mode = Mode::Home(Home::new(list));
            }
            _ => {}
        }
    }

    fn handle_shortcut_inputs(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Esc => {
                let list = self.get_services().expect("Failed to get services.");
                self.mode = Mode::Home(Home::new(list));
            }
            _ => {}
        }
    }
}
