use std::io;
use crate::app::App;

mod app;
mod ui;
mod models;
mod db;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App::new();

    let app_result = app.run(&mut terminal);

    ratatui::restore();
    app_result
}
