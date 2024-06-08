use super::util::{default, down_key_for_table, up_key_for_table};
use crate::{
    app::App,
    enums::{Library, Menu},
    spotify::{
        library_section::{
            liked_songs::{liked_tracks, process_liked_tracks},
            made_fy::{made_fy, process_made_fy},
            made_fy_tracks::{fetch_made_fy_tracks, process_made_fy_tracks},
            podcast::{process_podcasts, user_podcast},
            recently_played::{process_recently_played, recently_played},
            user_album_tracks::{process_user_album_tracks, user_album_tracks},
            user_albums::{process_user_albums, user_albums},
            user_artist_tracks::{process_user_artist_tracks, user_artist_tracks},
            user_artists::{process_user_artists, user_artists},
        },
        player::start_playback::start_playback,
    },
};

pub fn go_to_library_event(app: &mut App) {
    app.selected_menu = Menu::Library;
    app.library_state.select(Some(0)); //reseting the library state
    default(app);
}

pub fn library_down_event(app: &mut App) {
    if app.selected_menu == Menu::Library {
        if app.library_state.selected() == Some(0) {
            if app.made_fy_selected {
                (app.made_fy_state, app.made_fy_index) = down_key_for_table(
                    app.made_fy_playlist_names.clone(),
                    app.made_fy_state.clone(),
                );
            }
            if app.made_fy_track_selected {
                (app.made_fy_track_state, app.made_fy_track_index) = down_key_for_table(
                    app.made_fy_track_names.clone(),
                    app.made_fy_track_state.clone(),
                );
            }
        }
        if app.library_state.selected() == Some(2) {
            if app.liked_songs_selected {
                (app.liked_songs_state, app.liked_songs_index) =
                    down_key_for_table(app.liked_song_names.clone(), app.liked_songs_state.clone());
            }
        } else if app.library_state.selected() == Some(3) {
            if app.user_album_selected {
                (app.user_album_state, app.user_album_index) =
                    down_key_for_table(app.user_album_names.clone(), app.user_album_state.clone());
            }
            if app.user_album_track_selected {
                (app.user_album_track_state, app.user_album_track_index) = down_key_for_table(
                    app.user_album_track_names.clone(),
                    app.user_album_track_state.clone(),
                );
            }
        } else if app.library_state.selected() == Some(1) {
            if app.recently_played_selected {
                (app.recently_played_state, app.recently_played_index) = down_key_for_table(
                    app.recently_played_names.clone(),
                    app.recently_played_state.clone(),
                );
            }
        } else if app.library_state.selected() == Some(5) {
            if app.podcast_selected {
                (app.podcast_state, app.podcast_index) =
                    down_key_for_table(app.podcast_names.clone(), app.podcast_state.clone());
            }
        } else if app.library_state.selected() == Some(4) {
            if app.user_artist_selected {
                (app.user_artist_state, app.user_artist_index) = down_key_for_table(
                    app.user_artist_names.clone(),
                    app.user_artist_state.clone(),
                );
            }
            if app.user_artist_track_selected {
                (app.user_artist_track_state, app.user_artist_track_index) = down_key_for_table(
                    app.user_artist_track_names.clone(),
                    app.user_artist_track_state.clone(),
                );
            }
        }
    }
}

pub fn library_up_event(app: &mut App) {
    if app.selected_menu == Menu::Library {
        if app.library_state.selected() == Some(0) {
            if app.made_fy_selected {
                (app.made_fy_state, app.made_fy_index) = up_key_for_table(
                    app.made_fy_playlist_names.clone(),
                    app.made_fy_state.clone(),
                );
            }
            if app.made_fy_track_selected {
                (app.made_fy_track_state, app.made_fy_track_index) = up_key_for_table(
                    app.made_fy_track_names.clone(),
                    app.made_fy_track_state.clone(),
                );
            }
        }
        if app.library_state.selected() == Some(2) {
            if app.liked_songs_selected {
                (app.liked_songs_state, app.liked_songs_index) =
                    up_key_for_table(app.liked_song_names.clone(), app.liked_songs_state.clone());
            }
        } else if app.library_state.selected() == Some(3) {
            if app.user_album_selected {
                (app.user_album_state, app.user_album_index) =
                    up_key_for_table(app.user_album_names.clone(), app.user_album_state.clone());
            }
            if app.user_album_track_selected {
                (app.user_album_track_state, app.user_album_track_index) = up_key_for_table(
                    app.user_album_track_names.clone(),
                    app.user_album_track_state.clone(),
                );
            }
        } else if app.library_state.selected() == Some(1) {
            if app.recently_played_selected {
                (app.recently_played_state, app.recently_played_index) = up_key_for_table(
                    app.recently_played_names.clone(),
                    app.recently_played_state.clone(),
                )
            }
        } else if app.library_state.selected() == Some(5) {
            if app.podcast_selected {
                (app.podcast_state, app.podcast_index) =
                    up_key_for_table(app.podcast_names.clone(), app.podcast_state.clone());
            }
        } else if app.library_state.selected() == Some(4) {
            if app.user_artist_selected {
                (app.user_artist_state, app.user_artist_index) =
                    up_key_for_table(app.user_artist_names.clone(), app.user_artist_state.clone());
            }
            if app.user_artist_track_selected {
                (app.user_artist_track_state, app.user_artist_track_index) = up_key_for_table(
                    app.user_artist_track_names.clone(),
                    app.user_artist_track_state.clone(),
                );
            }
        }
    }
}

pub fn library_enter_event(app: &mut App) {
    if app.selected_menu == Menu::Library {
        app.searched_album_selected = false;
        app.searched_artist_selected = false;
        app.searched_playlist_selected = false;
        if app.library_state.selected() == Some(0) {
            app.selected_library = Library::MadeFY;
            if app.made_fy_current_playlist_selected {
                if let Err(e) = fetch_made_fy_tracks(app) {
                    println!("{}", e);
                }
                process_made_fy_tracks(app);
                app.made_fy_track_display = true;
                app.made_fy_display = false;
                app.made_fy_track_selected = true;
                app.made_fy_current_playlist_selected = false;
                app.made_fy_selected = false;
                app.enter_for_playback_in_made_fy = true;
                app.made_fy_track_state.select(Some(0));
            } else if app.enter_for_playback_in_made_fy {
                app.selected_link_for_playback =
                    app.made_fy_track_links[app.made_fy_track_index].clone();
                if let Err(e) = start_playback(app) {
                    println!("{}", e);
                }
            } else {
                if let Err(e) = made_fy(app) {
                    println!("{}", e);
                }
                process_made_fy(app);
                app.made_fy_display = true;
                app.made_fy_current_playlist_selected = true;
            }
        } else if app.library_state.selected() == Some(2) {
            app.selected_library = Library::LikedSongs;
            if app.enter_for_playback_in_liked_song {
                app.selected_link_for_playback =
                    app.liked_song_links[app.liked_songs_index].clone();
                if let Err(e) = start_playback(app) {
                    println!("{}", e);
                }
            } else {
                if let Err(e) = liked_tracks(app) {
                    println!("{}", e);
                }
                process_liked_tracks(app);
                app.liked_song_display = true;
                app.enter_for_playback_in_liked_song = true;
            }
        } else if app.library_state.selected() == Some(3) {
            app.selected_library = Library::Albums;
            if app.user_album_current_album_selected {
                if let Err(e) = user_album_tracks(app) {
                    println!("{}", e);
                }
                process_user_album_tracks(app);
                app.user_album_track_display = true;
                app.user_album_display = false;
                app.user_album_track_selected = true;
                app.user_album_current_album_selected = false;
                app.user_album_selected = false;
                app.user_album_track_state.select(Some(0));
            } else if app.enter_for_playback_in_user_album {
                app.selected_link_for_playback =
                    app.user_album_track_links[app.user_album_track_index].clone();
                if let Err(e) = start_playback(app) {
                    println!("{}", e);
                }
            } else {
                if let Err(e) = user_albums(app) {
                    println!("{}", e);
                }
                process_user_albums(app);
                app.user_album_display = true;
                app.user_album_current_album_selected = true;
                app.enter_for_playback_in_user_album = true;
            }
        } else if app.library_state.selected() == Some(1) {
            app.selected_library = Library::RecentlyPlayed;
            if app.enter_for_playback_in_recently_played {
                app.selected_link_for_playback =
                    app.recently_played_links[app.recently_played_index].clone();
                if let Err(e) = start_playback(app) {
                    println!("{}", e);
                }
            } else {
                if let Err(e) = recently_played(app) {
                    println!("{}", e);
                }
                process_recently_played(app);
                app.recently_played_display = true;
                app.enter_for_playback_in_recently_played = true;
            }
        } else if app.library_state.selected() == Some(5) {
            app.selected_library = Library::Podcasts;
            if let Err(e) = user_podcast(app) {
                println!("{}", e);
            }
            process_podcasts(app);
            app.podcast_display = true;
        } else if app.library_state.selected() == Some(4) {
            app.selected_library = Library::Artists;
            if app.user_artist_current_artist_selected {
                if let Err(e) = user_artist_tracks(app) {
                    println!("{}", e);
                }
                process_user_artist_tracks(app);
                app.user_artist_track_display = true;
                app.user_artist_display = false;
                app.user_artist_track_selected = true;
                app.user_artist_current_artist_selected = false;
                app.user_artist_selected = false;
                app.user_artist_track_state.select(Some(0));
            } else if app.enter_for_playback_in_saved_artist {
                app.selected_link_for_playback =
                    app.user_artist_track_links[app.user_artist_track_index].clone();
                if let Err(e) = start_playback(app) {
                    println!("{}", e);
                }
            } else {
                if let Err(e) = user_artists(app) {
                    println!("{}", e);
                }
                process_user_artists(app);
                app.user_artist_display = true;
                app.user_artist_current_artist_selected = true;
                app.enter_for_playback_in_saved_artist = true;
            }
        }
    }
}

pub fn library_tab_event(app: &mut App) {
    if app.selected_menu == Menu::Library {
        app.can_navigate_menu = !app.can_navigate_menu;
        if app.library_state.selected() == Some(0) && app.made_fy_display {
            app.made_fy_state.select(Some(0));
            app.made_fy_selected = !app.made_fy_selected;
        } else if app.library_state.selected() == Some(2) && app.liked_song_display {
            app.liked_songs_state.select(Some(0));
            app.liked_songs_selected = !app.liked_songs_selected;
        } else if app.library_state.selected() == Some(3) && app.user_album_display {
            app.user_album_state.select(Some(0));
            app.user_album_selected = !app.user_album_selected;
        } else if app.library_state.selected() == Some(1) && app.recently_played_display {
            app.recently_played_state.select(Some(0));
            app.recently_played_selected = !app.recently_played_selected;
        } else if app.library_state.selected() == Some(5) && app.podcast_display {
            app.podcast_state.select(Some(0));
            app.podcast_selected = !app.podcast_selected;
        } else if app.library_state.selected() == Some(4) && app.user_artist_display {
            app.user_artist_state.select(Some(0));
            app.user_artist_selected = !app.user_artist_selected;
        }
    }
}
