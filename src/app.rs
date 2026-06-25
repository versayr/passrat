use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget, DefaultTerminal, Frame};
use rusqlite::Connection;
use std::io;

use crate::{
    db::init_database,
    models::{Account, Service},
};

pub struct App {
    exit: bool,
    locked: bool,
    pub password: String,
    pub alert: String,
    mode: Mode,
    pub conn: Option<Connection>,
    selected_service: Option<Service>,
    selected_account: Option<Account>,
}

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
            selected_service: None,
            selected_account: None,
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
            },
            KeyCode::Char(char) => self.password.push(char),
            _ => {}
        }
    }

    fn handle_list_inputs(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('h') | KeyCode::Char('?') => self.mode = Mode::Help,
            KeyCode::Char('e') => self.mode = Mode::Edit,
            KeyCode::Char('n') => self.new_service(),
            KeyCode::Char('\\') => self.mode = Mode::Shortcuts,
            KeyCode::Enter => self.mode = Mode::View,
            _ => {}
        }
    }

    fn handle_view_inputs(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('h') | KeyCode::Char('?') => self.mode = Mode::Help,
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
        if let Ok(conn) = init_database(self.password.clone()) {
            self.conn = Some(conn);
            self.password = "".into();
            self.alert = "".into();
            self.locked = false;
        } else {
            self.password = "".into();
            self.alert = "Incorrect password - please try again.".into();
        }
    }

    fn new_service(&mut self) {
        let new_service = Service::default();
        self.selected_service = Some(new_service);
        self.mode = Mode::Edit;
    }

    fn new_account(&mut self) {
        let new_account = Account::default();
        self.selected_account = Some(new_account);
        self.mode = Mode::Edit;
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
