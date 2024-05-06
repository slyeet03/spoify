use super::util::{
    delete_char, down_key_for_list, down_key_for_table, move_cursor_left, move_cursor_right,
    reset_cursor, up_key_for_list, up_key_for_table,
};
use crate::app::App;
use crate::enums::{InputMode, Menu};
use crate::spotify::library_section::{
    liked_songs::{liked_tracks, process_liked_tracks},
    podcast::{process_podcasts, user_podcast},
    recently_played::{process_recently_played, recently_played},
    user_albums::{process_user_albums, user_albums},
    user_artists::{process_user_artists, user_artists},
};
use crate::spotify::lyrics::lyric::lyric;
use crate::spotify::new_release_section::new_releases_tracks::{
    new_releases_tracks, process_new_releases_tracks,
};
use crate::spotify::player::volume_decrease::volume_decreament;
use crate::spotify::player::volume_increase::volume_increment;
use crate::spotify::player::{
    pause_playback::pause, play_playback::play, repeat::cycle_repeat, shuffle::toogle_shuffle,
};
use crate::spotify::search::process_search;
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
            // Exit the application when 'q' is pressed in Normal mode
            code if code == KeyCode::Char(exit_application_key)
                && app.input_mode != InputMode::Editing =>
            {
                app.exit()
            }
            KeyCode::Char('l') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                if let Err(e) = lyric(app) {
                    println!("{}", e);
                }
                app.selected_menu = Menu::Lyrics;
                app.new_release_state.select(Some(0));
                app.search_results_rendered = false;
                app.input_mode = InputMode::Normal;
                app.user_playlist_display = false;
                app.liked_song_display = false;
                app.selected_search = false;
                app.user_album_display = false;
                app.can_navigate_menu = false;
                app.recently_played_display = false;
                app.podcast_display = false;
                app.user_artist_display = false;
            }
            // Navigate to different menus (Library, Playlists, Search) when 'l', 'p', or 's' is pressed

            // Go to Library Menu
            code if code == KeyCode::Char(go_to_library_key)
                && app.input_mode != InputMode::Editing =>
            {
                app.selected_menu = Menu::Library;
                app.library_state.select(Some(0)); //reseting the library state
                app.search_results_rendered = false;
                app.input_mode = InputMode::Normal;
                app.user_playlist_display = false;
                app.liked_song_display = false;
                app.selected_search = false;
                app.user_album_display = false;
                app.recently_played_display = false;
                app.can_navigate_menu = true;
                app.podcast_display = false;
                app.user_artist_display = false;
            }

            // Go to user playlist menu
            code if code == KeyCode::Char(go_to_user_playlists_key)
                && app.input_mode != InputMode::Editing =>
            {
                app.selected_menu = Menu::Playlists;
                app.user_playlist_state.select(Some(0));
                app.search_results_rendered = false;
                app.input_mode = InputMode::Normal;
                app.user_playlist_display = false;
                app.liked_song_display = false;
                app.selected_search = false;
                app.user_album_display = false;
                app.can_navigate_menu = true;
                app.recently_played_display = false;
                app.podcast_display = false;
                app.user_artist_display = false;
            }

            // Go to search menu
            code if code == KeyCode::Char(go_to_search_key)
                && app.input_mode != InputMode::Editing =>
            {
                app.selected_menu = Menu::Search;
                app.input_mode = InputMode::Editing;
                app.search_results_rendered = false;
                app.liked_song_display = false;
                app.user_album_display = false;
                app.recently_played_display = false;
                app.can_navigate_menu = true;
                app.podcast_display = false;
                app.user_artist_display = false;
            }

            // Go to help menu
            code if code == KeyCode::Char(help_key) && app.input_mode != InputMode::Editing => {
                app.selected_menu = Menu::Help;
                app.search_results_rendered = false;
                app.input_mode = InputMode::Normal;
                app.user_playlist_display = false;
                app.liked_song_display = false;
                app.selected_search = false;
                app.user_album_display = false;
                app.can_navigate_menu = false;
                app.recently_played_display = false;
                app.podcast_display = false;
                app.user_artist_display = false;
            }

            // Go to New Release Menu
            code if code == KeyCode::Char(new_release_key)
                && app.input_mode != InputMode::Editing =>
            {
                app.selected_menu = Menu::NewRelease;
                app.new_release_state.select(Some(0));
                app.search_results_rendered = false;
                app.input_mode = InputMode::Normal;
                app.user_playlist_display = false;
                app.liked_song_display = false;
                app.selected_search = false;
                app.user_album_display = false;
                app.can_navigate_menu = true;
                app.recently_played_display = false;
                app.podcast_display = false;
                app.user_artist_display = false;
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

            // Down keybinding for all the menus
            KeyCode::Down if app.input_mode != InputMode::Editing => {
                if app.selected_menu == Menu::Library {
                    if app.library_state.selected() == Some(2) {
                        if app.liked_songs_selected {
                            app.liked_songs_state = down_key_for_table(
                                app.liked_song_names.clone(),
                                app.liked_songs_state.clone(),
                            );
                        }
                    } else if app.library_state.selected() == Some(3) {
                        if app.user_album_selected {
                            app.user_album_state = down_key_for_table(
                                app.user_album_names.clone(),
                                app.user_album_state.clone(),
                            );
                        }
                    } else if app.library_state.selected() == Some(1) {
                        if app.recently_played_selected {
                            app.recently_played_state = down_key_for_table(
                                app.recently_played_names.clone(),
                                app.recently_played_state.clone(),
                            );
                        }
                    } else if app.library_state.selected() == Some(5) {
                        if app.podcast_selected {
                            app.podcast_state = down_key_for_table(
                                app.podcast_names.clone(),
                                app.podcast_state.clone(),
                            );
                        }
                    } else if app.library_state.selected() == Some(4) {
                        if app.user_artist_selected {
                            app.user_artist_state = down_key_for_table(
                                app.user_artist_names.clone(),
                                app.user_artist_state.clone(),
                            );
                        }
                    }
                }
                if app.selected_menu == Menu::NewRelease {
                    if app.new_release_album_selected {
                        app.new_release_album_state = down_key_for_table(
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
                        app.user_playlist_tracks_state = down_key_for_table(
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
                            app.track_state_in_search_result = down_key_for_list(
                                app.track_names_search_results.clone(),
                                app.track_state_in_search_result.clone(),
                            );
                        }
                        if app.selected_album_in_search_result {
                            app.album_state_in_search_result = down_key_for_list(
                                app.album_names_search_results.clone(),
                                app.album_state_in_search_result.clone(),
                            );
                        }
                        if app.selected_artist_in_search_result {
                            app.artist_state_in_search_result = down_key_for_list(
                                app.artist_names_search_results.clone(),
                                app.artist_state_in_search_result.clone(),
                            );
                        }
                        if app.selected_playlist_in_search_result {
                            app.playlist_state_in_search_result = down_key_for_list(
                                app.playlist_names_search_results.clone(),
                                app.playlist_state_in_search_result.clone(),
                            );
                        }
                    }
                }
                if app.can_navigate_menu {
                    let next_index: usize = app.library_state.selected().unwrap_or(0) + 1;
                    app.library_state.select(Some(next_index % 6)); //wrapping around the last option
                    app.search_results_rendered = false;
                    app.liked_song_display = false;
                    app.user_album_display = false;
                    app.recently_played_display = false;
                    app.podcast_display = false;
                    app.user_artist_display = false;
                }
            }

            // Up keybinding for all the menus
            KeyCode::Up if app.input_mode != InputMode::Editing => {
                if app.selected_menu == Menu::Library {
                    if app.library_state.selected() == Some(2) {
                        if app.liked_songs_selected {
                            app.liked_songs_state = up_key_for_table(
                                app.liked_song_names.clone(),
                                app.liked_songs_state.clone(),
                            );
                        }
                    } else if app.library_state.selected() == Some(3) {
                        if app.user_album_selected {
                            app.user_album_state = up_key_for_table(
                                app.user_album_names.clone(),
                                app.user_album_state.clone(),
                            );
                        }
                    } else if app.library_state.selected() == Some(1) {
                        if app.recently_played_selected {
                            app.recently_played_state = up_key_for_table(
                                app.recently_played_names.clone(),
                                app.recently_played_state.clone(),
                            )
                        }
                    } else if app.library_state.selected() == Some(5) {
                        if app.podcast_selected {
                            app.podcast_state = up_key_for_table(
                                app.podcast_names.clone(),
                                app.podcast_state.clone(),
                            );
                        }
                    } else if app.library_state.selected() == Some(4) {
                        if app.user_artist_selected {
                            app.user_artist_state = up_key_for_table(
                                app.user_artist_names.clone(),
                                app.user_artist_state.clone(),
                            );
                        }
                    }
                }
                if app.selected_menu == Menu::NewRelease {
                    if app.new_release_album_selected {
                        app.new_release_album_state = up_key_for_table(
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
                        app.user_playlist_tracks_state = up_key_for_table(
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
                            app.track_state_in_search_result = up_key_for_list(
                                app.track_names_search_results.clone(),
                                app.track_state_in_search_result.clone(),
                            );
                        }
                        if app.selected_album_in_search_result {
                            app.album_state_in_search_result = up_key_for_list(
                                app.album_names_search_results.clone(),
                                app.album_state_in_search_result.clone(),
                            );
                        }
                        if app.selected_artist_in_search_result {
                            app.artist_state_in_search_result = up_key_for_list(
                                app.artist_names_search_results.clone(),
                                app.artist_state_in_search_result.clone(),
                            );
                        }
                        if app.selected_playlist_in_search_result {
                            app.playlist_state_in_search_result = up_key_for_list(
                                app.playlist_names_search_results.clone(),
                                app.playlist_state_in_search_result.clone(),
                            );
                        }
                    }
                }
                if app.can_navigate_menu {
                    let prev_index = if app.library_state.selected().unwrap_or(0) == 0 {
                        5 //wrapping to the last option when user presses up at the first option
                    } else {
                        app.library_state.selected().unwrap_or(0) - 1
                    };
                    app.library_state.select(Some(prev_index));
                    app.search_results_rendered = false;
                    app.liked_song_display = false;
                    app.user_album_display = false;
                    app.recently_played_display = false;
                    app.podcast_display = false;
                    app.user_artist_display = false;
                }
            }

            // Enter keybinding for all the menus
            KeyCode::Enter if app.input_mode != InputMode::Editing => {
                if app.selected_menu == Menu::Playlists {
                    if let Err(e) = fetch_playlists_tracks(app) {
                        println!("{}", e);
                    }
                    process_playlist_tracks(app);
                    app.user_playlist_display = true;
                }
                if app.selected_menu == Menu::NewRelease {
                    if let Err(e) = new_releases_tracks(app) {
                        println!("{}", e);
                    }
                    process_new_releases_tracks(app);
                    app.new_release_display = true;
                }
                if app.selected_menu == Menu::Library {
                    if app.library_state.selected() == Some(2) {
                        if let Err(e) = liked_tracks(app) {
                            println!("{}", e);
                        }
                        process_liked_tracks(app);
                        app.liked_song_display = true;
                    } else if app.library_state.selected() == Some(3) {
                        if let Err(e) = user_albums(app) {
                            println!("{}", e);
                        }
                        process_user_albums(app);
                        app.user_album_display = true;
                    } else if app.library_state.selected() == Some(1) {
                        if let Err(e) = recently_played(app) {
                            println!("{}", e);
                        }
                        process_recently_played(app);
                        app.recently_played_display = true;
                    } else if app.library_state.selected() == Some(5) {
                        if let Err(e) = user_podcast(app) {
                            println!("{}", e);
                        }
                        process_podcasts(app);
                        app.podcast_display = true;
                    } else if app.library_state.selected() == Some(4) {
                        if let Err(e) = user_artists(app) {
                            println!("{}", e);
                        }
                        process_user_artists(app);
                        app.user_artist_display = true;
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
                    } else if app.search_state.selected() == Some(1) {
                        app.artist_state_in_search_result.select(Some(0));
                        app.selected_artist_in_search_result =
                            !app.selected_artist_in_search_result;
                        app.selected_track_in_search_result = false;
                        app.selected_album_in_search_result = false;
                        app.selected_playlist_in_search_result = false;
                    } else if app.search_state.selected() == Some(2) {
                        app.album_state_in_search_result.select(Some(0));
                        app.selected_album_in_search_result = !app.selected_album_in_search_result;
                        app.selected_track_in_search_result = false;
                        app.selected_artist_in_search_result = false;
                        app.selected_playlist_in_search_result = false;
                    } else if app.search_state.selected() == Some(3) {
                        app.playlist_state_in_search_result.select(Some(0));
                        app.selected_playlist_in_search_result =
                            !app.selected_playlist_in_search_result;
                        app.selected_track_in_search_result = false;
                        app.selected_artist_in_search_result = false;
                        app.selected_album_in_search_result = false;
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
}
