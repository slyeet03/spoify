use std::io;

use crate::app::App;

use crate::ui::tui;

use std::fs::{self};

mod app;
mod enums;
mod handlers;
mod spotify;
mod ui;

fn main() -> io::Result<()> {
    // Create data folder if it doesn't exist
    let data_dir = std::path::Path::new("./data");
    if !data_dir.exists() {
        fs::create_dir(data_dir).unwrap();
    }

    //initialise the tui
    let mut terminal = tui::init()?;
    //new instance for app
    let mut app = App::default();
    //running app's main loop
    let app_result = app.run(&mut terminal);
    tui::restore()?;
    app_result
}
