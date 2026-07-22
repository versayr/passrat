use arboard::Clipboard;
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
    models::{Account, Field, Service},
};

pub struct App {
    pub exit: bool,
    pub mode: Mode,
    pub conn: Option<Connection>,
    pub services: ServiceList,
    pub clipboard: Clipboard,
}

#[derive(Debug)]
pub enum Mode {
    Lock(LockState),
    Home(HomeState),
    Help,
    Cuts,
    Edit(EditState),
    View(ViewState),
}

#[derive(Debug, Default)]
pub struct LockState {
    pub password: String,
    pub alert: String,
}

#[derive(Debug, Default)]
pub struct HomeState {
    pub filter: String,
}

#[derive(Debug, Default)]
pub struct ViewState {
    pub service: Service,
    pub accounts: AccountList,
}

#[derive(Debug)]
pub struct EditState {
    pub list: Vec<Field>,
    pub state: ListState,
}

#[derive(Debug, Default)]
pub struct ServiceList {
    pub list: Vec<Service>,
    pub state: ListState,
}

#[derive(Debug, Default, Clone)]
pub struct AccountList {
    pub list: Vec<Account>,
    pub state: ListState,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            mode: Mode::Lock(LockState::default()),
            conn: None,
            services: ServiceList::default(),
            clipboard: Clipboard::new().unwrap(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        terminal.draw(|frame| self.draw(frame))?;

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events();
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    pub fn submit_password(&mut self, password: &str) {
        let path: BaseDirectories = BaseDirectories::with_prefix("passrat");
        path.create_data_directory("")
            .expect("Failed to create data directory.");

        if let Some(path) = path.find_data_file("vault.db") {
            if let Ok(conn) = connect_database(&path, password) {
                self.conn = Some(conn);
                self.get_services()
                    .expect("Failed to get list of services.");
                self.mode = Mode::Home(HomeState::default());
                self.services.state.select(Some(0));
            } else {
                self.mode = Mode::Lock(LockState {
                    password: String::new(),
                    alert: "Incorrect input - please try again.".into(),
                });
            }
        } else {
            let _ = init_database(password);
            self.mode = Mode::Lock(LockState {
                password: String::new(),
                alert: "Database created - please enter passphrase again.".into(),
            });
        }
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

        for service in result {
            self.services.list.push(service?);
        }

        Ok(())
    }

    pub fn get_accounts(&mut self, service_id: u16) -> Result<Vec<Account>, Error> {
        let mut stmt = self
            .conn
            .as_mut()
            .expect("Failed to connect to database.")
            .prepare(&format!(
                "SELECT * FROM accounts WHERE service_id = {service_id} ORDER BY username"
            ))
            .expect("Failed to prepare statement.");

        let mut rows = stmt.query([])?;

        let mut accounts = vec![];

        while let Some(row) = rows.next()? {
            accounts.push(Account::from_row(row));
        }

        Ok(accounts)
    }

    pub fn add_service(&mut self) {
        let service = Service::default();
        let conn = self.conn.as_mut().expect("Failed to get connection.");

        let _ = conn.execute(
            "INSERT INTO services (name, url) VALUES (?1, ?2)",
            params![service.name, service.url],
        );

        self.get_services()
            .expect("Failed to refresh service list.");
    }

    //     fn update_service(&mut self) -> Result<(), Error> {
    //
    //     }
    //
    //     fn remove_service(&mut self) -> Result<(), Error> {
    //
    //     }

    pub fn add_account(&mut self) {
        let account = Account::default();
        let conn = self.conn.as_mut().expect("Failed to get connection.");

        let _ = conn.execute(
            "INSERT INTO accounts (
                service_id,
                username,
                last_change,
                creation_date,
                email,
                password,
                access_token,
                pin,
                passcode) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                &self.services.list[self
                    .services
                    .state
                    .selected()
                    .expect("No selected service.")]
                .id,
                account.username,
                account.last_change,
                account.creation_date,
                account.email,
                account.password,
                account.access_token,
                account.pin,
                account.passcode
            ],
        );

        // TODO
        // refresh Mode::View account list after adding a new account?
        // self.get_accounts()
        //     .expect("Failed to refresh accounts list.");
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
        match &self.mode {
            Mode::Lock(_) => self.render_lock_mode(area, buf),
            Mode::Home(_) => self.render_home_mode(area, buf),
            Mode::Help => self.render_help_mode(area, buf),
            Mode::Cuts => self.render_shortcut_mode(area, buf),
            Mode::Edit(_) => self.render_edit_mode(area, buf),
            Mode::View(_) => self.render_view_mode(area, buf),
        }
    }
}
