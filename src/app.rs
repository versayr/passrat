use arboard::Clipboard;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    widgets::{ListState, Widget},
};
use rusqlite::Connection;
use std::io::{self};

use crate::{
    models::Service,
    modes::{edit::Edit, home::Home, lock::Lock, view::View},
};

pub struct App {
    pub exit: bool,
    pub mode: Mode,
    pub conn: Option<Connection>,
    // TODO remove duplicated ServiceLists in App & Home
    pub services: ServiceList,
    pub clipboard: Clipboard,
    // TODO remove this hack
    pub should_clear: bool,
}

#[derive(Debug)]
pub enum Mode {
    Lock(Lock),
    Home(Home),
    View(View),
    Edit(Edit),
    Help,
    Cuts,
}

#[derive(Debug, Default)]
pub struct ServiceList {
    pub list: Vec<Service>,
    pub state: ListState,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            mode: Mode::Lock(Lock::default()),
            conn: None,
            services: ServiceList::default(),
            clipboard: Clipboard::new().expect("Clipboard failed to initialize."),
            should_clear: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            // TODO remove this hack
            if self.should_clear {
                terminal.clear()?;
                self.should_clear = false;
            }
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events();
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        match &mut self.mode {
            Mode::Lock(lock) => lock.render(area, buf),
            Mode::Edit(edit) => edit.render(area, buf),
            Mode::View(view) => view.render(area, buf),
            Mode::Home(home) => home.render(area, buf),
            Mode::Help => self.render_help_mode(area, buf),
            Mode::Cuts => self.render_shortcut_mode(area, buf),
        }
    }
}
