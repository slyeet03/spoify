use super::util::{
    default, default_nav, default_search, delete_char, down_key_for_list, down_key_for_table,
    move_cursor_left, move_cursor_right, reset_cursor, up_key_for_list, up_key_for_table,
};
use crate::app::App;
use crate::enums::{InputMode, Library, Menu, SearchMenu};
use crate::spotify::library_section::made_fy::{made_fy, process_made_fy};
use crate::spotify::library_section::made_fy_tracks::{
    fetch_made_fy_tracks, process_made_fy_tracks,
};
use crate::spotify::library_section::user_album_tracks::{
    process_user_album_tracks, user_album_tracks,
};
use crate::spotify::library_section::user_artist_tracks::{
    process_user_artist_tracks, user_artist_tracks,
};
use crate::spotify::library_section::{
    liked_songs::{liked_tracks, process_liked_tracks},
    podcast::{process_podcasts, user_podcast},
    recently_played::{process_recently_played, recently_played},
    user_albums::{process_user_albums, user_albums},
    user_artists::{process_user_artists, user_artists},
};
use crate::spotify::new_release_section::new_releases_tracks::{
    new_releases_tracks, process_new_releases_tracks,
};
use crate::spotify::player::next_track::next_track;
use crate::spotify::player::previous_track::previous_track;
use crate::spotify::player::start_playback::start_playback;
use crate::spotify::player::volume_decrease::volume_decreament;
use crate::spotify::player::volume_increase::volume_increment;
use crate::spotify::player::{
    pause_playback::pause, play_playback::play, repeat::cycle_repeat, shuffle::toogle_shuffle,
};
use crate::spotify::playlist_control::add_track_to_playlist::add_track_to_playlist;
use crate::spotify::search::search::process_search;
use crate::spotify::search::search_albums::{
    process_selected_album_tracks, search_selected_album_tracks,
};
use crate::spotify::search::search_artists::{
    process_selected_artist_tracks, search_selected_artist_tracks,
};
use crate::spotify::search::search_playlists::{
    process_selected_playlist_tracks, search_selected_playlist_tracks,
};
use crate::spotify::user_playlist::user_playlist_track::{
    fetch_playlists_tracks, process_playlist_tracks,
};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::io::{self, Write};

/// Function to handle key events for the application
pub fn handle_key_event(app: &mut App, key_event: KeyEvent) {
    let go_to_search_key: char = app.go_to_search_key;
    let go_to_library_key: char = app.go_to_library_key;
    let go_to_user_playlists_key: char = app.go_to_user_playlists_key;
    let exit_application_key: char = app.exit_application_key;
    let help_key: char = app.help_key;
    let volume_up_key: char = app.volume_up_key;
    let volume_down_key: char = app.volume_down_key;
    let new_release_key: char = app.new_release_key;
    let next_track_key: char = app.next_track_key;
    let previous_track_key: char = app.previous_track_key;
    let error_key: char = app.error_key;
    let player_fullscreen_key: char = app.player_fullscreen_key;

    if key_event.kind == KeyEventKind::Press {
        match key_event.code {
            // Toggle shuffle mode when Ctrl+S is pressed
            KeyCode::Char('s') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                app.is_shuffle = !app.is_shuffle;
                if let Err(e) = toogle_shuffle(app) {
                    println!("{}", e);
                }
            }
            // Cycle through repeat options when Ctrl+R is pressed
            KeyCode::Char('r') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                if let Err(e) = cycle_repeat(app) {
                    println!("{}", e);
                }
            }

            KeyCode::Char('p') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                if app.selected_menu == Menu::Library {
                    if app.selected_library == Library::RecentlyPlayed {
                        app.track_added_to_playlist_link =
                            app.recently_played_links[app.recently_played_index].clone();
                    } else if app.selected_library == Library::LikedSongs {
                        app.track_added_to_playlist_link =
                            app.liked_song_links[app.liked_songs_index].clone();
                    } else if app.selected_library == Library::MadeFY {
                        if app.made_fy_track_selected {
                            app.track_added_to_playlist_link =
                                app.made_fy_track_links[app.made_fy_track_index].clone();
                        }
                    } else if app.selected_library == Library::Albums {
                        if app.user_album_track_selected {
                            app.track_added_to_playlist_link =
                                app.user_album_track_links[app.user_album_track_index].clone();
                        }
                    } else if app.selected_library == Library::Artists {
                        if app.user_artist_track_selected {
                            app.track_added_to_playlist_link =
                                app.user_artist_track_links[app.user_artist_track_index].clone();
                        }
                        app.selected_menu = Menu::AddTrackToPlaylist;
                    }
                } else if app.selected_menu == Menu::NewRelease {
                    if app.enter_for_playback_in_new_release {
                        app.track_added_to_playlist_link =
                            app.new_release_spotify_urls[app.new_release_index].clone();
                        app.selected_menu = Menu::AddTrackToPlaylist;
                    }
                } else if app.selected_menu == Menu::Playlists {
                    if app.enter_for_playback_in_user_playlist {
                        app.track_added_to_playlist_link =
                            app.user_playlist_track_links[app.user_playlist_index].clone();
                        app.selected_menu = Menu::AddTrackToPlaylist;
                    }
                } else if app.selected_menu == Menu::Search {
                    if app.is_in_track {
                        app.track_added_to_playlist_link =
                            app.track_links_search_results[app.track_index].clone();
                        app.selected_menu = Menu::AddTrackToPlaylist;
                    } else if app.search_menu == SearchMenu::SearchedAlbum {
                        app.track_added_to_playlist_link =
                            app.selected_album_tracks_links[app.searched_album_index].clone();
                        app.selected_menu = Menu::AddTrackToPlaylist;
                    } else if app.search_menu == SearchMenu::SearchedArtist {
                        app.track_added_to_playlist_link =
                            app.selected_artist_tracks_links[app.searched_artist_index].clone();
                        app.selected_menu = Menu::AddTrackToPlaylist;
                    } else if app.search_menu == SearchMenu::SearchedPlaylist {
                        app.track_added_to_playlist_link =
                            app.selected_playlist_tracks_links[app.searched_playlist_index].clone();
                        app.selected_menu = Menu::AddTrackToPlaylist;
                    }
                }
            }

            // Exit the application when 'q' is pressed in Normal mode
            code if code == KeyCode::Char(exit_application_key)
                && app.input_mode != InputMode::Editing =>
            {
                if app.selected_menu == Menu::Search {
                    app.selected_menu = Menu::Default;
                } else if app.selected_menu == Menu::Library {
                    if app.selected_library == Library::MadeFY {
                        if app.made_fy_track_selected {
                            app.made_fy_track_selected = false;
                            app.made_fy_track_display = false;
                            app.made_fy_selected = true;
                            app.made_fy_display = true;
                        } else {
                            app.selected_menu = Menu::Default;
                        }
                    } else if app.selected_library == Library::Albums {
                        if app.user_album_track_selected {
                            app.user_album_track_selected = false;
                            app.user_album_track_display = false;
                            app.user_album_current_album_selected = true;
                            app.user_album_display = true;
                            app.user_album_selected = true;
                        } else {
                            app.selected_menu = Menu::Default;
                        }
                    } else if app.selected_library == Library::Artists {
                        if app.user_artist_track_selected {
                            app.user_artist_track_selected = false;
                            app.user_artist_track_display = false;
                            app.user_artist_current_artist_selected = true;
                            app.user_artist_display = true;
                            app.user_artist_selected = true;
                        } else {
                            app.selected_menu = Menu::Default;
                        }
                    } else {
                        app.selected_menu = Menu::Default;
                    }
                } else if app.selected_menu == Menu::Playlists {
                    app.selected_menu = Menu::Default;
                } else if app.selected_menu == Menu::NewRelease {
                    app.selected_menu = Menu::Default;
                } else if app.selected_menu == Menu::AddTrackToPlaylist {
                    app.selected_menu = Menu::Default;
                } else {
                    app.exit();
                }
            }

            // Navigate to different menus (Library, Playlists, Search, New Releases) when 'l', 'p', 's' or 'n' is pressed

            // Go to Library Menu
            code if code == KeyCode::Char(go_to_library_key)
                && app.input_mode != InputMode::Editing =>
            {
                app.selected_menu = Menu::Library;
                app.library_state.select(Some(0)); //reseting the library state
                default(app);
            }

            // Go to user playlist menu
            code if code == KeyCode::Char(go_to_user_playlists_key)
                && app.input_mode != InputMode::Editing =>
            {
                app.selected_menu = Menu::Playlists;
                app.user_playlist_state.select(Some(0));
                default(app);
                app.selected_playlist_uri = app.user_playlist_links[0].clone();
                app.current_user_playlist = app.user_playlist_names[0].clone();
            }

            // Go to search menu
            code if code == KeyCode::Char(go_to_search_key)
                && app.input_mode != InputMode::Editing =>
            {
                app.selected_menu = Menu::Search;
                default(app);
                app.input_mode = InputMode::Editing;
            }

            // Go to help menu
            code if code == KeyCode::Char(help_key) && app.input_mode != InputMode::Editing => {
                if app.selected_menu == Menu::Help {
                    app.selected_menu = Menu::Default;
                } else {
                    app.selected_menu = Menu::Help;
                }
            }

            code if code == KeyCode::Char(player_fullscreen_key)
                && app.input_mode != InputMode::Editing =>
            {
                if app.selected_menu == Menu::Player {
                    app.selected_menu = Menu::Default;
                } else {
                    app.selected_menu = Menu::Player;
                }
            }

            // Go to New Release Menu
            code if code == KeyCode::Char(new_release_key)
                && app.input_mode != InputMode::Editing =>
            {
                app.selected_menu = Menu::NewRelease;
                app.new_release_state.select(Some(0));
                default(app);
            }

            // Keys for Volume Control
            code if code == KeyCode::Char(volume_down_key)
                && app.input_mode != InputMode::Editing =>
            {
                if let Err(e) = volume_decreament(app) {
                    println!("{}", e);
                }
            }

            code if code == KeyCode::Char(volume_up_key)
                && app.input_mode != InputMode::Editing =>
            {
                if let Err(e) = volume_increment(app) {
                    println!("{}", e);
                }
            }

            // Keys for next and previous track
            code if code == KeyCode::Char(next_track_key)
                && app.input_mode != InputMode::Editing =>
            {
                if let Err(e) = next_track(app) {
                    println!("{}", e);
                }
            }
            code if code == KeyCode::Char(previous_track_key)
                && app.input_mode != InputMode::Editing =>
            {
                if let Err(e) = previous_track(app) {
                    println!("{}", e);
                }
            }

            // Key for Error Screen
            code if code == KeyCode::Char(error_key) && app.input_mode != InputMode::Editing => {
                if app.selected_menu == Menu::Error {
                    app.selected_menu = Menu::Default;
                } else {
                    app.selected_menu = Menu::Error;
                }
            }

            // Down keybinding for all the menus
            KeyCode::Down if app.input_mode != InputMode::Editing => {
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
                            (app.liked_songs_state, app.liked_songs_index) = down_key_for_table(
                                app.liked_song_names.clone(),
                                app.liked_songs_state.clone(),
                            );
                        }
                    } else if app.library_state.selected() == Some(3) {
                        if app.user_album_selected {
                            (app.user_album_state, app.user_album_index) = down_key_for_table(
                                app.user_album_names.clone(),
                                app.user_album_state.clone(),
                            );
                        }
                        if app.user_album_track_selected {
                            (app.user_album_track_state, app.user_album_track_index) =
                                down_key_for_table(
                                    app.user_album_track_names.clone(),
                                    app.user_album_track_state.clone(),
                                );
                        }
                    } else if app.library_state.selected() == Some(1) {
                        if app.recently_played_selected {
                            (app.recently_played_state, app.recently_played_index) =
                                down_key_for_table(
                                    app.recently_played_names.clone(),
                                    app.recently_played_state.clone(),
                                );
                        }
                    } else if app.library_state.selected() == Some(5) {
                        if app.podcast_selected {
                            (app.podcast_state, app.podcast_index) = down_key_for_table(
                                app.podcast_names.clone(),
                                app.podcast_state.clone(),
                            );
                        }
                    } else if app.library_state.selected() == Some(4) {
                        if app.user_artist_selected {
                            (app.user_artist_state, app.user_artist_index) = down_key_for_table(
                                app.user_artist_names.clone(),
                                app.user_artist_state.clone(),
                            );
                        }
                        if app.user_artist_track_selected {
                            (app.user_artist_track_state, app.user_artist_track_index) =
                                down_key_for_table(
                                    app.user_artist_track_names.clone(),
                                    app.user_artist_track_state.clone(),
                                );
                        }
                    }
                }
                if app.selected_menu == Menu::NewRelease {
                    if app.new_release_album_selected {
                        (app.new_release_album_state, app.new_release_index) = down_key_for_table(
                            app.new_release_track_names.clone(),
                            app.new_release_album_state.clone(),
                        );
                    } else {
                        let length: usize = app.new_release_name.len();
                        let next_index: usize = app.new_release_state.selected().unwrap_or(0) + 1;
                        app.new_release_state.select(Some(next_index % length));
                        app.search_results_rendered = false;
                        if next_index >= length {
                        } else {
                            app.current_new_release = app.new_release_name[next_index].clone();
                            app.current_new_release_album_link =
                                app.new_release_album_links[next_index].clone();
                        }
                        app.new_release_display = false;
                    }
                }
                if app.selected_menu == Menu::Playlists {
                    if app.user_playlist_tracks_selected {
                        (app.user_playlist_tracks_state, app.user_playlist_index) =
                            down_key_for_table(
                                app.user_playlist_track_names.clone(),
                                app.user_playlist_tracks_state.clone(),
                            );
                    } else {
                        let length: usize = app.user_playlist_names.len();
                        let next_index: usize = app.user_playlist_state.selected().unwrap_or(0) + 1;
                        app.user_playlist_state.select(Some(next_index % length));
                        app.search_results_rendered = false;
                        if next_index >= length {
                        } else {
                            app.selected_playlist_uri = app.user_playlist_links[next_index].clone();
                            app.current_user_playlist = app.user_playlist_names[next_index].clone();
                        }
                        app.user_playlist_display = false;
                    }
                }
                if app.selected_menu == Menu::Search {
                    if app.selected_search {
                        if app.selected_track_in_search_result {
                            (app.track_state_in_search_result, app.track_index) = down_key_for_list(
                                app.track_names_search_results.clone(),
                                app.track_state_in_search_result.clone(),
                            );
                        }
                        if app.selected_album_in_search_result {
                            (app.album_state_in_search_result, app.album_index) = down_key_for_list(
                                app.album_names_search_results.clone(),
                                app.album_state_in_search_result.clone(),
                            );
                        }
                        if app.selected_artist_in_search_result {
                            (app.artist_state_in_search_result, app.artist_index) =
                                down_key_for_list(
                                    app.artist_names_search_results.clone(),
                                    app.artist_state_in_search_result.clone(),
                                );
                        }
                        if app.selected_playlist_in_search_result {
                            (app.playlist_state_in_search_result, app.playlist_index) =
                                down_key_for_list(
                                    app.playlist_names_search_results.clone(),
                                    app.playlist_state_in_search_result.clone(),
                                );
                        }
                    }
                    if app.search_menu == SearchMenu::SearchedAlbum {
                        if app.searched_album_selected {
                            (app.searched_album_state, app.searched_album_index) =
                                down_key_for_table(
                                    app.selected_album_tracks_names.clone(),
                                    app.searched_album_state.clone(),
                                );
                        }
                    }
                    if app.search_menu == SearchMenu::SearchedArtist {
                        if app.searched_artist_selected {
                            (app.searched_artist_state, app.searched_artist_index) =
                                down_key_for_table(
                                    app.selected_artist_tracks_names.clone(),
                                    app.searched_artist_state.clone(),
                                );
                        }
                    }
                    if app.search_menu == SearchMenu::SearchedPlaylist {
                        if app.searched_playlist_selected {
                            (app.searched_playlist_state, app.searched_playlist_index) =
                                down_key_for_table(
                                    app.selected_playlist_tracks_names.clone(),
                                    app.searched_playlist_state.clone(),
                                );
                        }
                    }
                }
                if app.selected_menu == Menu::AddTrackToPlaylist {
                    (
                        app.add_track_to_playlist_state,
                        app.playlist_index_for_track_addition,
                    ) = down_key_for_list(
                        app.user_playlist_names.clone(),
                        app.add_track_to_playlist_state.clone(),
                    );
                }
                if app.can_navigate_menu {
                    let next_index: usize = app.library_state.selected().unwrap_or(0) + 1;
                    app.library_state.select(Some(next_index % 6)); //wrapping around the last option
                    default_nav(app);
                }
            }

            // Up keybinding for all the menus
            KeyCode::Up if app.input_mode != InputMode::Editing => {
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
                            (app.liked_songs_state, app.liked_songs_index) = up_key_for_table(
                                app.liked_song_names.clone(),
                                app.liked_songs_state.clone(),
                            );
                        }
                    } else if app.library_state.selected() == Some(3) {
                        if app.user_album_selected {
                            (app.user_album_state, app.user_album_index) = up_key_for_table(
                                app.user_album_names.clone(),
                                app.user_album_state.clone(),
                            );
                        }
                        if app.user_album_track_selected {
                            (app.user_album_track_state, app.user_album_track_index) =
                                up_key_for_table(
                                    app.user_album_track_names.clone(),
                                    app.user_album_track_state.clone(),
                                );
                        }
                    } else if app.library_state.selected() == Some(1) {
                        if app.recently_played_selected {
                            (app.recently_played_state, app.recently_played_index) =
                                up_key_for_table(
                                    app.recently_played_names.clone(),
                                    app.recently_played_state.clone(),
                                )
                        }
                    } else if app.library_state.selected() == Some(5) {
                        if app.podcast_selected {
                            (app.podcast_state, app.podcast_index) = up_key_for_table(
                                app.podcast_names.clone(),
                                app.podcast_state.clone(),
                            );
                        }
                    } else if app.library_state.selected() == Some(4) {
                        if app.user_artist_selected {
                            (app.user_artist_state, app.user_artist_index) = up_key_for_table(
                                app.user_artist_names.clone(),
                                app.user_artist_state.clone(),
                            );
                        }
                        if app.user_artist_track_selected {
                            (app.user_artist_track_state, app.user_artist_track_index) =
                                up_key_for_table(
                                    app.user_artist_track_names.clone(),
                                    app.user_artist_track_state.clone(),
                                );
                        }
                    }
                }
                if app.selected_menu == Menu::NewRelease {
                    if app.new_release_album_selected {
                        (app.new_release_album_state, app.new_release_index) = up_key_for_table(
                            app.new_release_track_names.clone(),
                            app.new_release_album_state.clone(),
                        );
                    } else {
                        let length: usize = app.new_release_name.len();
                        let prev_index: usize =
                            if app.new_release_state.selected().unwrap_or(0) == 0 {
                                length - 1
                            } else {
                                app.new_release_state.selected().unwrap_or(0) - 1
                            };
                        app.new_release_state.select(Some(prev_index));
                        app.search_results_rendered = false;
                        app.current_new_release = app.new_release_name[prev_index].clone();
                        app.current_new_release_album_link =
                            app.new_release_album_links[prev_index].clone();
                        app.new_release_display = false;
                    }
                }
                if app.selected_menu == Menu::Playlists {
                    if app.user_playlist_tracks_selected {
                        (app.user_playlist_tracks_state, app.user_playlist_index) =
                            up_key_for_table(
                                app.user_playlist_track_names.clone(),
                                app.user_playlist_tracks_state.clone(),
                            );
                    } else {
                        let length: usize = app.user_playlist_names.len();
                        let prev_index: usize =
                            if app.user_playlist_state.selected().unwrap_or(0) == 0 {
                                length - 1
                            } else {
                                app.user_playlist_state.selected().unwrap_or(0) - 1
                            };
                        app.user_playlist_state.select(Some(prev_index));
                        app.search_results_rendered = false;
                        app.selected_playlist_uri = app.user_playlist_links[prev_index].clone();
                        app.current_user_playlist = app.user_playlist_names[prev_index].clone();
                        app.user_playlist_display = false;
                    }
                }
                if app.selected_menu == Menu::Search {
                    if app.selected_search {
                        if app.selected_track_in_search_result {
                            (app.track_state_in_search_result, app.track_index) = up_key_for_list(
                                app.track_names_search_results.clone(),
                                app.track_state_in_search_result.clone(),
                            );
                        }
                        if app.selected_album_in_search_result {
                            (app.album_state_in_search_result, app.album_index) = up_key_for_list(
                                app.album_names_search_results.clone(),
                                app.album_state_in_search_result.clone(),
                            );
                        }
                        if app.selected_artist_in_search_result {
                            (app.artist_state_in_search_result, app.artist_index) = up_key_for_list(
                                app.artist_names_search_results.clone(),
                                app.artist_state_in_search_result.clone(),
                            );
                        }
                        if app.selected_playlist_in_search_result {
                            (app.playlist_state_in_search_result, app.playlist_index) =
                                up_key_for_list(
                                    app.playlist_names_search_results.clone(),
                                    app.playlist_state_in_search_result.clone(),
                                );
                        }
                    }
                    if app.search_menu == SearchMenu::SearchedAlbum {
                        if app.searched_album_selected {
                            (app.searched_album_state, app.searched_album_index) = up_key_for_table(
                                app.selected_album_tracks_names.clone(),
                                app.searched_album_state.clone(),
                            );
                        }
                    }
                    if app.search_menu == SearchMenu::SearchedArtist {
                        if app.searched_artist_selected {
                            (app.searched_artist_state, app.searched_artist_index) =
                                up_key_for_table(
                                    app.selected_artist_tracks_names.clone(),
                                    app.searched_artist_state.clone(),
                                );
                        }
                    }
                    if app.search_menu == SearchMenu::SearchedPlaylist {
                        if app.searched_playlist_selected {
                            (app.searched_playlist_state, app.searched_playlist_index) =
                                up_key_for_table(
                                    app.selected_playlist_tracks_names.clone(),
                                    app.searched_playlist_state.clone(),
                                );
                        }
                    }
                }
                if app.selected_menu == Menu::AddTrackToPlaylist {
                    (
                        app.add_track_to_playlist_state,
                        app.playlist_index_for_track_addition,
                    ) = up_key_for_list(
                        app.user_playlist_names.clone(),
                        app.add_track_to_playlist_state.clone(),
                    );
                }
                if app.can_navigate_menu {
                    let prev_index = if app.library_state.selected().unwrap_or(0) == 0 {
                        5 //wrapping to the last option when user presses up at the first option
                    } else {
                        app.library_state.selected().unwrap_or(0) - 1
                    };
                    app.library_state.select(Some(prev_index));
                    default_nav(app);
                }
            }

            // Enter keybinding for all the menus
            KeyCode::Enter if app.input_mode != InputMode::Editing => {
                if app.selected_menu == Menu::Playlists {
                    if app.enter_for_playback_in_user_playlist {
                        app.selected_link_for_playback =
                            app.user_playlist_track_links[app.user_playlist_index].clone();
                        if let Err(e) = start_playback(app) {
                            println!("{}", e);
                        }
                    } else {
                        if let Err(e) = fetch_playlists_tracks(app) {
                            println!("{}", e);
                        }
                        process_playlist_tracks(app);
                        app.user_playlist_display = true;
                        app.searched_album_selected = false;
                        app.searched_artist_selected = false;
                        app.searched_playlist_selected = false;
                        app.enter_for_playback_in_user_playlist = true;
                    }
                }
                if app.selected_menu == Menu::NewRelease {
                    if app.enter_for_playback_in_new_release {
                        app.selected_link_for_playback =
                            app.new_release_spotify_urls[app.new_release_index].clone();
                        if let Err(e) = start_playback(app) {
                            println!("{}", e);
                        }
                    } else {
                        if let Err(e) = new_releases_tracks(app) {
                            println!("{}", e);
                        }
                        process_new_releases_tracks(app);
                        app.new_release_display = true;
                        app.searched_album_selected = false;
                        app.searched_artist_selected = false;
                        app.searched_playlist_selected = false;
                        app.enter_for_playback_in_new_release = true;
                    }
                }
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
                if app.selected_menu == Menu::Search {
                    if app.is_in_track {
                        app.is_only_id = true;
                        app.selected_link_for_playback =
                            app.track_links_search_results[app.track_index].clone();
                        if let Err(e) = start_playback(app) {
                            println!("{}", e);
                        }
                    }

                    if app.search_menu == SearchMenu::SearchedAlbum {
                        app.is_only_id = false;
                        app.selected_link_for_playback =
                            app.selected_album_tracks_links[app.searched_album_index].clone();
                        if let Err(e) = start_playback(app) {
                            println!("{}", e);
                        }
                    } else if app.selected_album_in_search_result {
                        if let Err(e) = search_selected_album_tracks(app) {
                            println!("{}", e);
                        }
                        process_selected_album_tracks(app);
                        default_search(app);
                        app.search_menu = SearchMenu::SearchedAlbum;
                        app.searched_album_selected = true;
                    }

                    if app.search_menu == SearchMenu::SearchedArtist {
                        app.is_only_id = false;
                        app.selected_link_for_playback =
                            app.selected_artist_tracks_links[app.searched_artist_index].clone();
                        if let Err(e) = start_playback(app) {
                            println!("{}", e);
                        }
                    } else if app.selected_artist_in_search_result {
                        if let Err(e) = search_selected_artist_tracks(app) {
                            println!("{}", e);
                        }
                        process_selected_artist_tracks(app);
                        default_search(app);
                        app.search_menu = SearchMenu::SearchedArtist;
                        app.searched_artist_selected = true;
                    }

                    if app.search_menu == SearchMenu::SearchedPlaylist {
                        app.is_only_id = false;
                        app.selected_link_for_playback =
                            app.selected_playlist_tracks_links[app.searched_playlist_index].clone();
                        if let Err(e) = start_playback(app) {
                            println!("{}", e);
                        }
                    } else if app.selected_playlist_in_search_result {
                        if let Err(e) = search_selected_playlist_tracks(app) {
                            println!("{}", e);
                        }
                        process_selected_playlist_tracks(app);
                        default_search(app);
                        app.search_menu = SearchMenu::SearchedPlaylist;
                        app.searched_playlist_selected = true;
                    }
                }
                if app.selected_menu == Menu::AddTrackToPlaylist {
                    app.playlist_link_for_track_addition =
                        app.user_playlist_links[app.playlist_index_for_track_addition].clone();
                    if let Err(e) = add_track_to_playlist(app) {
                        println!("{}", e);
                    }
                }
            }

            // Tab keybinding for all the menus
            KeyCode::Tab if app.input_mode != InputMode::Editing => {
                if app.selected_menu == Menu::Playlists {
                    app.can_navigate_menu = !app.can_navigate_menu;
                    if app.user_playlist_display {
                        app.user_playlist_tracks_state.select(Some(0));
                        app.user_playlist_tracks_selected = !app.user_playlist_tracks_selected;
                    }
                }
                if app.selected_menu == Menu::NewRelease {
                    app.can_navigate_menu = !app.can_navigate_menu;
                    if app.new_release_display {
                        app.new_release_album_state.select(Some(0));
                        app.new_release_album_selected = !app.new_release_album_selected;
                    }
                }
                if app.selected_menu == Menu::Library {
                    app.can_navigate_menu = !app.can_navigate_menu;
                    if app.library_state.selected() == Some(0) {
                        if app.made_fy_display {
                            app.made_fy_state.select(Some(0));
                            app.made_fy_selected = !app.made_fy_selected;
                        }
                    }
                    if app.library_state.selected() == Some(2) {
                        if app.liked_song_display {
                            app.liked_songs_state.select(Some(0));
                            app.liked_songs_selected = !app.liked_songs_selected;
                        }
                    } else if app.library_state.selected() == Some(3) {
                        if app.user_album_display {
                            app.user_album_state.select(Some(0));
                            app.user_album_selected = !app.user_album_selected;
                        }
                    } else if app.library_state.selected() == Some(1) {
                        if app.recently_played_display {
                            app.recently_played_state.select(Some(0));
                            app.recently_played_selected = !app.recently_played_selected;
                        }
                    } else if app.library_state.selected() == Some(5) {
                        if app.podcast_display {
                            app.podcast_state.select(Some(0));
                            app.podcast_selected = !app.podcast_selected;
                        }
                    } else if app.library_state.selected() == Some(4) {
                        if app.user_artist_display {
                            app.user_artist_state.select(Some(0));
                            app.user_artist_selected = !app.user_artist_selected;
                        }
                    }
                }
                if app.selected_search {
                    app.can_navigate_menu = false;
                    app.track_state_in_search_result.select(None);
                    app.artist_state_in_search_result.select(None);
                    app.album_state_in_search_result.select(None);
                    app.playlist_state_in_search_result.select(None);

                    if app.search_state.selected() == Some(0) {
                        app.track_state_in_search_result.select(Some(0));
                        app.selected_track_in_search_result = !app.selected_track_in_search_result;
                        app.selected_artist_in_search_result = false;
                        app.selected_album_in_search_result = false;
                        app.selected_playlist_in_search_result = false;
                        app.is_in_track = true;
                    } else if app.search_state.selected() == Some(1) {
                        app.artist_state_in_search_result.select(Some(0));
                        app.selected_artist_in_search_result =
                            !app.selected_artist_in_search_result;
                        app.selected_track_in_search_result = false;
                        app.selected_album_in_search_result = false;
                        app.selected_playlist_in_search_result = false;
                        app.is_in_track = false;
                    } else if app.search_state.selected() == Some(2) {
                        app.album_state_in_search_result.select(Some(0));
                        app.selected_album_in_search_result = !app.selected_album_in_search_result;
                        app.selected_track_in_search_result = false;
                        app.selected_artist_in_search_result = false;
                        app.selected_playlist_in_search_result = false;
                        app.is_in_track = false;
                    } else if app.search_state.selected() == Some(3) {
                        app.playlist_state_in_search_result.select(Some(0));
                        app.selected_playlist_in_search_result =
                            !app.selected_playlist_in_search_result;
                        app.selected_track_in_search_result = false;
                        app.selected_artist_in_search_result = false;
                        app.selected_album_in_search_result = false;
                        app.is_in_track = false;
                    }
                    let length = 4;
                    let next_index = app.search_state.selected().unwrap_or(0) + 1;
                    app.search_state.select(Some(next_index % length));
                }
            }

            // Pause/Play using Spacebar
            KeyCode::Char(' ') if app.input_mode != InputMode::Editing => {
                if app.playback_status == "Paused" {
                    if let Err(e) = play(app) {
                        println!("{}", e);
                    }
                } else if app.playback_status == "Playing" {
                    if let Err(e) = pause(app) {
                        println!("{}", e);
                    }
                }
            }

            // Just exit from Search Menu
            KeyCode::Esc if app.input_mode != InputMode::Editing => {
                app.selected_menu = Menu::Default;
            }

            // Handle character input in search mode
            KeyCode::Char(c) if app.input_mode == InputMode::Editing => {
                // Handle character input in search mode
                if !c.is_control() {
                    app.input.push(c);
                    move_cursor_right(app);
                }
            }

            _ => {}
        }
    }
}

/// Function to handle search input and related key events
pub fn search_input(app: &mut App, key_event: KeyEvent) -> io::Result<()> {
    if key_event.kind == KeyEventKind::Press {
        match app.input_mode {
            InputMode::Editing => match key_event.code {
                // Submit the search query when Enter is pressed
                KeyCode::Enter => {
                    submit_message(app);
                    std::io::sink().write_all(&[0])?;
                }
                // Delete a character when Backspace is pressed
                KeyCode::Backspace => {
                    delete_char(app);
                    std::io::sink().write_all(&[0])?;
                }
                // Move the cursor left when Left arrow is pressed
                KeyCode::Left => {
                    move_cursor_left(app);
                    std::io::sink().write_all(&[0])?;
                }
                // Move the cursor right when Right arrow is pressed
                KeyCode::Right => {
                    move_cursor_right(app);
                    std::io::sink().write_all(&[0])?;
                }
                // Exit search mode when Esc is pressed
                KeyCode::Esc => {
                    app.input_mode = InputMode::Normal;
                    app.search_results_rendered = false;
                    std::io::sink().write_all(&[0])?;
                }
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}

// Submit the search query and process the search results
fn submit_message(app: &mut App) {
    app.search_query = app.input.clone();

    let binding = app.search_query.clone();
    let query = binding.as_str();

    let _ = process_search(app, query);

    app.input.clear();
    reset_cursor(app);

    app.input_mode = InputMode::SearchResults;
    app.search_results_rendered = true;
    app.selected_search = true;
    app.search_state.select(Some(0));
}
