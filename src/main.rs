use std::io;

use crate::app::App;
use crate::ui::tui;

mod app;
mod enums;
mod handlers;
mod ui;

fn main() -> io::Result<()> {
    //initialise the tui
    let mut terminal = tui::init()?;
    //new instance for app
    let mut app = App::default();
    //running app's main loop
    let app_result = app.run(&mut terminal);
    tui::restore()?;
    app_result
}
