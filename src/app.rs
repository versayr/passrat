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
    pub exit: bool,
    pub mode: Mode,
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
pub enum Mode {
    Lock,
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
            password: "".into(),
            alert: "".into(),
            mode: Mode::Lock,
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

    pub fn submit_password(&mut self) -> Result<(), Error> {
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
                        self.get_services()
                            .expect("Failed to get list of services.");
                        self.mode = Mode::List;
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

    pub fn add_service(&mut self) -> Result<(), Error> {
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

    pub fn add_account(&mut self) -> Result<(), Error> {
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
        match self.mode {
            Mode::Lock => self.render_lock_mode(area, buf),
            Mode::List => self.render_list_mode(area, buf),
            Mode::View => self.render_view_mode(area, buf),
            Mode::Edit => self.render_edit_mode(area, buf),
            Mode::Help => self.render_help_mode(area, buf),
            Mode::Cuts => self.render_shortcut_mode(area, buf),
        }
    }
}
