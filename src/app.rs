use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    widgets::{ListState, Widget},
};
use rusqlite::{Connection, Error, params};
use std::io;
use xdg::BaseDirectories;

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

#[derive(Debug, Default)]
pub struct ServiceList {
    pub list: Vec<Service>,
    pub state: ListState,
}

#[derive(Debug, Default)]
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
    Cuts,
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
            services: ServiceList::default(),
            accounts: AccountList::default(),
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

    fn submit_password(&mut self) -> Result<(), Error> {
        let path: BaseDirectories = BaseDirectories::with_prefix("passrat");
        path.create_data_directory("")
            .expect("Failed to create data directory.");

        match path.find_data_file("vault.db") {
            Some(path) => {
                match connect_database(&path, &self.password) {
                    Ok(conn) => {
                        self.conn = Some(conn);
                        self.password = "".into();
                        self.alert = "".into();
                        self.locked = false;
                        self.get_services()
                            .expect("Failed to get list of services.");
                        self.services.state.select(Some(0));
                    },
                    Err(_) => {
                        self.password = "".into();
                        self.alert = "Incorrect password - please try again.".into();
                    }
                }
            }
            None => {
                let _ = init_database(&self.password);
                self.password = "".into();
                self.alert = "Database created - please enter passphrase again.".into();
            }
        }

        Ok(())
    }

    pub fn get_services(&mut self) -> Result<(), Error> {
        let mut stmt = self
            .conn
            .as_mut()
            .expect("Failed to connect to database.")
            .prepare("SELECT id, name, url FROM services ORDER BY name")?;

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

        let result = stmt.query_map([], Account::from_row)?;

        self.accounts.list.clear();

        for account in result.into_iter() {
            self.accounts.list.push(account?);
        }

        self.accounts.state.select(Some(0));

        Ok(())
    }

    fn add_service(&mut self) -> Result<(), Error> {
        let service = Service::default();
        let conn = self.conn.as_mut().expect("Failed to get connection.");

        let _ = conn.execute(
            "INSERT INTO services (name, url) VALUES (?1, ?2)",
            params![service.name, service.url],
        );

        self.get_services()
            .expect("Failed to refresh service list.");
        Ok(())
    }

    //     fn update_service(&mut self) -> Result<(), Error> {
    //
    //     }
    //
    //     fn remove_service(&mut self) -> Result<(), Error> {
    //
    //     }

    fn add_account(&mut self) -> Result<(), Error> {
        let account = Account::default();
        let conn = self.conn.as_mut().expect("Failed to get connection.");

        let _ = conn.execute(
            "INSERT INTO accounts (
            service_id,
            username,
            last_change,
            account_creation_date,
            email,
            password,
            access_token,
            pin,
            passcode) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                &self
                    .selected_service
                    .as_ref()
                    .expect("No selected service.")
                    .id,
                account.username,
                account.last_change,
                account.account_creation_date,
                account.email,
                account.password,
                account.access_token,
                account.pin,
                account.passcode
            ],
        );

        self.get_accounts()
            .expect("Failed to refresh accounts list.");
        Ok(())
    }

    //     fn update_account(&mut self) -> Result<(), Error> {
    //
    //     }
    //
    //     fn remove_account(&mut self) -> Result<(), Error> {
    //
    //     }
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
                Mode::Cuts => self.render_shortcut_mode(area, buf),
            }
        }
    }
}
