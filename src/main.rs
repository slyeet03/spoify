use crate::app::App;
use crate::spotify::player::player::{currently_playing, process_currently_playing};
use crate::ui::tui;
use ratatui::style::Color;
use spotify::user_playlist::user_playlist::{get_playlists, process_user_playlists};
use std::fs::OpenOptions;
use std::io;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

mod app;
mod enums;
mod handlers;
mod spotify;
mod ui;

fn main() -> io::Result<()> {
    let mut app = App::default();
    startup(&mut app);

    let mut terminal = tui::init()?;

    let (tx, rx) = mpsc::channel();
    let mut player_info_app = app.clone();
    thread::spawn(move || update_player_info(tx, &mut player_info_app));

    app.run(&mut terminal, rx)?;

    tui::restore()?;
    Ok(())
}

fn update_player_info(tx: mpsc::Sender<()>, app: &mut App) {
    loop {
        currently_playing().unwrap();

        // Send a message to the main thread to update the UI
        if tx.send(()).is_err() {
            break;
        }

        thread::sleep(Duration::from_millis(900));
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
}
