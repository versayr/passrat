use crate::app::App;

mod app;
mod db;
mod forms;
mod helpers;
mod inputs;
mod models;
mod ui;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();

    let mut app = App::new();

    let app_result = app.run(&mut terminal);

    ratatui::restore();
    Ok(app_result?)
}
