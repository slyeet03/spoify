use std::io;
use std::sync::mpsc;
use std::thread;

use ui::tui;
use util::{startup, update_player_info};

use crate::app::App;

mod app;
mod enums;
mod handlers;
mod spotify;
mod ui;
mod util;

fn main() -> io::Result<()> {
    let mut app: App = App::default();

    // Fetch user's playlist
    startup(&mut app);

    let mut terminal = tui::init()?;

    let (tx, rx) = mpsc::channel();
    let mut player_info_app: App = app.clone();

    // Spawn a new thread to update player's current playback
    thread::spawn(move || update_player_info(tx, &mut player_info_app));

    // Run the main app loop
    app.run(&mut terminal, rx)?;

    tui::restore()?;
    Ok(())
}
