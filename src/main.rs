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

    let (tx1, rx) = mpsc::channel();
    //let (tx2, _) = mpsc::channel();
    let mut player_info_app: App = app.clone();
    //let mut queue_app: App = app.clone();

    // Spawn a new thread to update player's current playback
    let player_info_thread = thread::spawn(move || update_player_info(tx1, &mut player_info_app));

    // Spawn a new thread to update the current queue
    //let queue_thread = thread::spawn(move || update_current_queue(tx2, &mut queue_app));

    // Run the main app loop
    app.run(&mut terminal, rx)?;

    // Wait for the spawned threads to complete
    if let Err(e) = player_info_thread.join() {
        eprintln!("Error in player_info_thread: {:?}", e);
    }
    /*
        if let Err(e) = queue_thread.join() {
            eprintln!("Error in queue_thread: {:?}", e);
        }
    */
    tui::restore()?;
    Ok(())
}
