use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{ListState, Widget},
    DefaultTerminal, Frame,
};
use rusqlite::{Connection, Error};
use std::{io, path::Path};

use crate::{
    db::{connect_database, init_database},
    models::{Account, Service},
};

#[derive(Debug)]
pub struct App {
    exit: bool,
    locked: bool,
    mode: Mode,
    pub password: String,
    pub alert: String,
    pub conn: Option<Connection>,
    pub services: ServiceList,
    pub accounts: AccountList,
    pub selected_service: Option<Service>,
}

#[derive(Debug)]
pub struct ServiceList {
    pub list: Vec<Service>,
    pub state: ListState,
}

#[derive(Debug)]
pub struct AccountList {
    pub list: Vec<Account>,
    pub state: ListState,
}

#[derive(Debug)]
enum Mode {
    List,
    View,
    Edit,
    Help,
    Shortcuts,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            locked: true,
            password: "".into(),
            alert: "".into(),
            mode: Mode::List,
            conn: None,
            services: ServiceList {
                list: vec![],
                state: ListState::default(),
            },
            accounts: AccountList {
                list: vec![],
                state: ListState::default(),
            },
            selected_service: None,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        terminal.draw(|frame| self.draw(frame))?;

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
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
                Mode::Shortcuts => self.handle_shortcut_inputs(event),
            }
        }
    }

    fn handle_locked_inputs(&mut self, event: KeyEvent) {
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
            KeyCode::Char('h') | KeyCode::Char('?') => self.mode = Mode::Help,
            KeyCode::Char('j') | KeyCode::Down => self.services.state.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.services.state.select_previous(),
            KeyCode::Char('e') => self.mode = Mode::Edit,
            KeyCode::Char('n') => self.new_service(),
            KeyCode::Char('\\') => self.mode = Mode::Shortcuts,
            KeyCode::Enter => {
                self.selected_service = Some(
                    self.services.list[self
                        .services
                        .state
                        .selected()
                        .expect("No service is selected.")].clone(),
                );
                let _ = self.get_accounts();
                self.mode = Mode::View;
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
            KeyCode::Char('n') => self.new_account(),
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

    fn submit_password(&mut self) {
        // TODO
        // add a check that to see if the db exists
        // if yes, then:
        //   get a connection using the password, or
        //   prompt user again for the correct password
        // if no, then:
        //   init the db and set the password
        let path = Path::new("vault.db");

        if path.exists() {
            if let Ok(conn) = connect_database(path, &self.password) {
                self.conn = Some(conn);
                self.password = "".into();
                self.alert = "".into();
                self.locked = false;
                self.get_services()
                    .expect("Failed to get list of services.");
                // TODO
                // handle empty services list?
                self.services.state.select(Some(0));
            } else {
                self.password = "".into();
                self.alert = "Incorrect password - please try again.".into();
            }
        } else if let Ok(conn) = init_database(path, &self.password) {
            self.conn = Some(conn);
            self.password = "".into();
            self.alert = "".into();
            self.locked = false;
        }
    }

    fn new_service(&mut self) {
        self.mode = Mode::Edit;
    }

    fn new_account(&mut self) {
        self.mode = Mode::Edit;
    }

    pub fn get_services(&mut self) -> Result<(), Error> {
        let mut stmt = self
            .conn
            .as_mut()
            .expect("Failed to connect to database.")
            .prepare("SELECT id, name, url FROM services ORDER BY name")
            .expect("Failed to prepare statement.");

        let result = stmt.query_map([], |row| {
            Ok(Service {
                id: row.get(0).expect("Failed to get service id."),
                name: row.get(1).expect("Failed to get service name."),
                url: row.get(2).expect("Failed to get service url."),
            })
        })?;

        self.services.list.clear();

        for service in result.into_iter() {
            self.services.list.push(service?);
        }

        Ok(())
    }

    pub fn get_accounts(&mut self) -> Result<(), Error> {
        let service_id = self.services.list[self
            .services
            .state
            .selected()
            .expect("No service selected.")]
        .id
        .expect("Failed to get service id.");

        let mut stmt = self
            .conn
            .as_mut()
            .expect("Failed to connect to database.")
            .prepare(&format!(
                "SELECT * FROM accounts WHERE service_id = {} ORDER BY username",
                service_id
            ))
            .expect("Failed to prepare statement.");

        let result = stmt.query_map([], |row| {
            Ok(Account {
                id: row.get(0).expect("Failed to get id."),
                service_id: row.get(1).expect("Failed to get service id."),
                username: row.get(2).expect("Failed to get username."),
                last_change: row.get(3).expect("Failed to get last change."),
                account_creation_date: row.get(4).expect("Failed to get account creation date."),
                email: row.get(5).expect("Failed to get email."),
                password: row.get(6).expect("Failed to get password."),
                access_token: row.get(7).expect("Failed to get access token."),
                pin: row.get(8).expect("Failed to get pin."),
                passcode: row.get(9).expect("Failed to get passcode."),
            })
        })?;

        self.accounts.list.clear();

        for account in result.into_iter() {
            self.accounts.list.push(account?);
        }

        self.accounts.state.select(Some(0));

        Ok(())
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        if self.locked {
            self.render_locked_mode(area, buf);
        } else {
            match self.mode {
                Mode::List => self.render_list_mode(area, buf),
                Mode::View => self.render_view_mode(area, buf),
                Mode::Edit => self.render_edit_mode(area, buf),
                Mode::Help => self.render_help_mode(area, buf),
                Mode::Shortcuts => self.render_shortcut_mode(area, buf),
            }
        }
    }
}
