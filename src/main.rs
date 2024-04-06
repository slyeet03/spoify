use std::io;
use std::path::PathBuf;

use crate::app::App;
use crate::spotify::user_playlist::user_playlist;

use crate::ui::tui;

use log::{error, info, warn};
use std::fs::OpenOptions;
use std::fs::{self};

use std::io::Write;

mod app;
mod enums;
mod handlers;
mod spotify;
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

pub fn init_logger() -> std::io::Result<()> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("..");
    path.push("spoify-tui");
    path.push("spotify_cache");
    path.push("spotify_log");
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
