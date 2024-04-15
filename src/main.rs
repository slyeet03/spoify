use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::app::App;
use crate::spotify::player::player::{currently_playing, process_currently_playing};
use crate::ui::tui;
use handlers::keybindings::{process_keybindings, read_keybindings, set_keybindings};
use handlers::theme::{read_theme,set_theme};
use spotify::user_playlist::user_playlist::{get_playlists, process_user_playlists};

mod app;
mod enums;
mod handlers;
mod spotify;
mod ui;

fn main() -> io::Result<()> {
    let mut app = App::default();

    // Fetch user's playlist
    startup(&mut app);

    let mut terminal = tui::init()?;

    let (tx, rx) = mpsc::channel();
    let mut player_info_app = app.clone();

    // Spawn a new thread to update player's current playback
    thread::spawn(move || update_player_info(tx, &mut player_info_app));

    // Run the main app loop
    app.run(&mut terminal, rx)?;

    tui::restore()?;
    Ok(())
}

/// Function to update the player information in a seperate thread
fn update_player_info(tx: mpsc::Sender<()>, app: &mut App) {
    loop {
        // Get the user's current playback
        currently_playing().unwrap();
        process_currently_playing(app);

        // Send a message to the main thread to update the UI
        if tx.send(()).is_err() {
            break;
        }

        // Wait one second before fetching playback again
        thread::sleep(Duration::from_millis(1000));
    }
}

/// Function to fetch user's playlists
fn startup(app: &mut App) {
    get_playlists();
    process_user_playlists(app);
    read_keybindings();
    set_keybindings(app);
    process_keybindings(app);
    read_theme();
    set_theme(app);
}
