use crate::structs::LikedSongs;
use crate::structs::MadeFY;
use crate::structs::NewRelease;
use crate::structs::Search;
use crate::structs::UserRecentlyPlayed;
use crate::structs::UserSavedAlbums;
use crate::structs::UserSavedArtist;
use crate::structs::UserSavedPodcast;
use std::io;
use std::sync::mpsc;
use std::thread;

use settings::creds::{read_creds, set_creds};
use structs::{Key, Settings, Themes, UserCurrentlyPlaying, UserPlaylist};
use ui::tui;
use util::{instruction, save_creds_to_yml, startup, update_player_info};

use crate::app::App;

mod app;
mod enums;
mod handlers;
mod settings;
mod spotify;
mod structs;
mod ui;
mod util;

fn main() -> io::Result<()> {
    let mut app: App = App::default();
    let mut key: Key = Key::default();
    let mut theme: Themes = Themes::default();
    let mut settings: Settings = Settings::default();
    let mut search: Search = Search::default();
    let mut userplaylist: UserPlaylist = UserPlaylist::default();
    let mut likedsongs: LikedSongs = LikedSongs::default();
    let mut useralbum: UserSavedAlbums = UserSavedAlbums::default();
    let mut madefy: MadeFY = MadeFY::default();
    let mut podcast: UserSavedPodcast = UserSavedPodcast::default();
    let mut recentlyplayed: UserRecentlyPlayed = UserRecentlyPlayed::default();
    let mut userartist: UserSavedArtist = UserSavedArtist::default();
    let mut newrelease: NewRelease = NewRelease::default();
    let mut currentlyplaying: UserCurrentlyPlaying = UserCurrentlyPlaying::default();

    app.file_name = "spoify-0.2.14".to_string(); //-0.2.14

    // Set the creds from the configure files
    read_creds(&mut app);
    set_creds(&mut app);

    if app.client_id == "" {
        instruction();
        save_creds_to_yml(&mut app);
    } else {
        // Fetch user's playlists, new releases, set keybinds and themes before the main app starts
        startup(
            &mut app,
            &mut key,
            &mut theme,
            &mut settings,
            &mut userplaylist,
            &mut newrelease,
        );

        let mut terminal = tui::init()?;

        let (tx1, rx1) = mpsc::channel();

        let mut player_info_app: App = app.clone();
        let mut player_info_settings: Settings = settings.clone();
        let mut currentlyplaying_clone = currentlyplaying.clone();

        // Spawn a new thread to update player's current playback
        let player_info_thread = thread::spawn(move || {
            update_player_info(
                tx1,
                &mut player_info_app,
                &mut player_info_settings,
                &mut currentlyplaying_clone,
            )
        });

        // Run the main app loop
        app.run(
            &mut terminal,
            rx1,
            &mut key,
            &mut theme,
            &mut settings,
            &mut search,
            &mut userplaylist,
            &mut likedsongs,
            &mut useralbum,
            &mut madefy,
            &mut podcast,
            &mut recentlyplayed,
            &mut userartist,
            &mut newrelease,
            &mut currentlyplaying,
        )?;

        // Wait for the spawned threads to complete
        if let Err(e) = player_info_thread.join() {
            eprintln!("Error in player_info_thread: {:?}", e);
        }

        tui::restore()?;
    }

    Ok(())
}
