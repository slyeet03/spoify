use crate::app::App;
use crate::spotify::player::currently_playling::{currently_playing, process_currently_playing};
use crate::spotify::player::devices::device;
use crate::spotify::player::devices::get_current_device;
use crate::spotify::player::devices::process_devices;
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
    let _ = device();
    process_devices(app);
    get_current_device(app);
    let _ = currently_playing();
    process_currently_playing(app);
}
