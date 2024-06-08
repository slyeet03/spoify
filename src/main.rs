use std::io;
use std::sync::mpsc;
use std::thread;

use settings::creds::{read_creds, set_creds};
use ui::tui;
use util::{instruction, save_creds_to_yml, startup, update_player_info};

use crate::app::App;

mod app;
mod enums;
mod handlers;
mod settings;
mod spotify;
mod ui;
mod util;

fn main() -> io::Result<()> {
    let mut app: App = App::default();

    app.file_name = "spoify-0.2.1".to_string();

    // Set the creds from the configure files
    read_creds(&mut app);
    set_creds(&mut app);

    if app.client_id == "" {
        instruction();
        save_creds_to_yml(&mut app);
    } else {
        // Fetch user's playlists, new releases, set keybinds and themes before the main app starts
        startup(&mut app);

        let mut terminal = tui::init()?;

        let (tx1, rx1) = mpsc::channel();

        let mut player_info_app: App = app.clone();

        // Spawn a new thread to update player's current playback
        let player_info_thread =
            thread::spawn(move || update_player_info(tx1, &mut player_info_app));

        // Run the main app loop
        app.run(&mut terminal, rx1)?;

        // Wait for the spawned threads to complete
        if let Err(e) = player_info_thread.join() {
            eprintln!("Error in player_info_thread: {:?}", e);
        }

        tui::restore()?;
    }

    Ok(())
}
