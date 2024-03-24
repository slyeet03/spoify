use std::io;

use crate::app::App;
use crate::ui::tui;

mod app;
mod enums;
mod handlers;
mod ui;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let mut app = App::default();
    let app_result = app.run(&mut terminal);
    tui::restore()?;
    app_result
}
