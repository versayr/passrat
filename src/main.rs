use std::panic::{self, AssertUnwindSafe};

use crate::app::App;

mod app;
mod db;
mod ui;
mod helpers;
mod models;
mod inputs;
mod forms;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();

    let mut app = App::new();

    // let app_result = app.run(&mut terminal);

    let app_result = panic::catch_unwind(AssertUnwindSafe(|| {
        let _ = app.run(&mut terminal);
    }));

    ratatui::restore();
    // Ok(app_result?)

    match app_result {
        Ok(result) => {
            eprintln!("App State: {app:#?}");
            Ok(result)
        }
        Err(err) => {
            eprintln!("App State: {app:#?}");
            std::panic::resume_unwind(err)
        }
    }
}
