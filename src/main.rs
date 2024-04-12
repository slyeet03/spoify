use crate::app::App;
use crate::spotify::player::player::{currently_playing, process_currently_playing};
use crate::ui::tui;
use spotify::user_playlist::user_playlist::{get_playlists, process_user_playlists};
use std::fs::OpenOptions;
use std::io;
use std::path::PathBuf;

mod app;
mod enums;
mod handlers;
mod spotify;
mod ui;

/*
fn main() -> io::Result<()> {
    //new instance for app
    let mut app = App::default();
    startup(&mut app);

    //initialise the tui
    let mut terminal = tui::init()?;
    //running app's main loop
    let app_result = app.run(&mut terminal);
    tui::restore()?;
    app_result
}
*/
// main.rs
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    // ...
    let mut app = App::default();
    startup(&mut app);
    let mut terminal = tui::init()?;
    // Create a channel for communication between the main thread and the progress bar update thread
    let (tx, rx) = mpsc::channel();

    // Start the progress bar update thread
    let mut thread_app = app.clone();
    thread::spawn(move || update_progress_bar(&mut thread_app, rx));

    // Running app's main loop

    let app_result = app.run(&mut terminal);
    tui::restore()?;
    app_result
}

fn update_progress_bar(app: &mut App, rx: mpsc::Receiver<()>) {
    loop {
        // Check if the main thread has requested to stop the progress bar update
        if let Ok(_) = rx.try_recv() {
            break;
        }

        // Fetch the current playback position from the Spotify API
        let _ = currently_playing();
        process_currently_playing(app);
        let current_progress = app.currrent_timestamp;

        // Update the progress bar ratio
        let total_duration = app.ending_timestamp;
        app.progress_bar_ratio = current_progress as f64 / total_duration as f64;

        // Sleep for a short duration before updating the progress bar again
        thread::sleep(Duration::from_millis(1000));
    }
}

pub fn init_logger() -> std::io::Result<()> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("..");
    path.push("spoify-tui");
    path.push("spotify_cache");
    std::fs::create_dir_all(&path)?;
    path.push("app.log");

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;

    env_logger::builder()
        .format_timestamp_millis()
        .format_module_path(false)
        .format_level(true)
        .format_indent(Some(4))
        .write_style(env_logger::WriteStyle::Never)
        .target(env_logger::Target::Pipe(Box::new(file)))
        .init();

    Ok(())
}

fn startup(app: &mut App) {
    get_playlists();
    process_user_playlists(app);
    let _ = currently_playing();
    process_currently_playing(app);
}
