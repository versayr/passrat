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

    #[allow(clippy::too_many_lines)]
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
                    self.clipboard
                        .set_text(str)
                        .expect("Failed to copy to system clipboard.");
                }
                HomeAction::Quit => self.exit = true,
                HomeAction::Help => self.mode = Mode::Help,
                HomeAction::Delete(service) => {
                    let _ = self.remove_service(&service);
                    let list = self
                        .get_services()
                        .expect("Failed to refresh service list.");
                    self.mode = Mode::Home(Home::new(list));
                }
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
                ViewAction::Copy(str) => self
                    .clipboard
                    .set_text(str)
                    .expect("Failed to copy to system clipboard."),
                ViewAction::Help => self.mode = Help,
                ViewAction::Quit => self.exit = true,
                ViewAction::Delete(account) => {
                    let _ = self.remove_account(&account);
                    let service = self
                        .get_service(account.service_id)
                        .expect("Failed to get service.");
                    let list = self.get_accounts(service.id.expect("Unable to get service's account list without a row id for the service.")).expect("Unable to get list of accounts.");
                    self.mode = Mode::View(View::new(&service, list));
                }
                ViewAction::None => {}
            },
            Mode::Edit(edit) => match edit.handle_inputs(event) {
                EditAction::Return => {
                    let list = self.get_services().expect("Failed to get services.");
                    // TODO if user was editing an account, return to view/service
                    self.mode = Mode::Home(Home::new(list));
                }
                EditAction::Paste => {
                    if let Some(ref mut input) = edit.input {
                        input.value = self
                            .clipboard
                            .get_text()
                            .expect("Failed to paste from clipboard.");
                    }
                }
                EditAction::Quit => self.exit = true,
                EditAction::Help => self.mode = Help,
                EditAction::Copy(text) => self
                    .clipboard
                    .set_text(text)
                    .expect("Failed to copy to clipboard."),
                EditAction::None => {}
                EditAction::Submit(target) => {
                    if let Err(error) = self.handle_target(&target)
                        && let Mode::Edit(edit) = &mut self.mode
                    {
                        edit.error = Some(error.to_string());
                    } else {
                        self.mode = match target {
                            Target::Shortcut(_) | Target::SecurityQuestion(_) => {
                                let list = self.get_services().expect("Failed to get services.");
                                Mode::Home(Home::new(list))
                            }
                            Target::Service(service) => {
                                if let Some(id) = service.id {
                                    let list = self.get_accounts(id).expect("Failed to get accounts.");
                                    Mode::View(View::new(&service, list))
                                } else {
                                    let list = self.get_services().expect("Failed to get services.");
                                    Mode::Home(Home::new(list))
                                }
                            }
                            Target::Account(account) => {
                                let service_id = account.service_id;
                                let list = self
                                    .get_accounts(service_id)
                                    .expect("Failed to get accounts.");
                                let service = self
                                    .get_service(service_id)
                                    .expect("Failed to get service.");
                                Mode::View(View::new(&service, list))
                            }
                        }
                    }
                }
            },
            Mode::Help => self.handle_help_inputs(event),
            Mode::Cuts => self.handle_shortcut_inputs(event),
        }
    }

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
