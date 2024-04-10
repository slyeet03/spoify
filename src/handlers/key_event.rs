use crate::app::App;
use crate::enums::{InputMode, Library, Menu};
use crate::spotify::liked_songs::{liked_tracks, process_liked_tracks};
use crate::spotify::user_playlist_track::{fetch_playlists_tracks, process_playlist_tracks};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use std::io;
use std::thread;

use crate::spotify::search::{process_search, search};

pub fn handle_events(app: &mut App) -> io::Result<()> {
    match event::read()? {
        //handling key press events
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            handle_key_event(app, key_event);
            search_input(app).unwrap();
        }
        _ => {}
    };

    Ok(())
}

fn handle_key_event(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        //hadling key events
        KeyCode::Char('q') => app.exit(),
        KeyCode::Char('l') => {
            app.selected_menu = Menu::Library;
            app.library_state.select(Some(0)); //reseting the library state
            app.search_results_rendered = false;
            app.input_mode = InputMode::Normal;
            app.user_playlist_display = false;
            app.liked_song_display = false;
            app.selected_search = false;
        }
        KeyCode::Char('p') => {
            app.selected_menu = Menu::Playlists;
            app.user_playlist_state.select(Some(0));
            app.search_results_rendered = false;
            app.input_mode = InputMode::Normal;
            app.user_playlist_display = false;
            app.liked_song_display = false;
            app.selected_search = false;
        }

        KeyCode::Char('s') => {
            app.user_playlist_display = false;
            app.selected_menu = Menu::Search;
            app.input_mode = InputMode::Normal;
            app.liked_song_display = false;
        }

        KeyCode::Char('m') => app.selected_menu = Menu::Main,
        KeyCode::Down => {
            if app.selected_menu == Menu::Library {
                if app.library_state.selected() == Some(2) {
                    if app.liked_songs_selected {
                        let length = app.liked_song_names.len();
                        let next_index = app.liked_songs_state.selected().unwrap_or(0) + 1;
                        app.liked_songs_state.select(Some(next_index % length));
                    }
                }
                if !app.liked_songs_selected {
                    let next_index = app.library_state.selected().unwrap_or(0) + 1;
                    app.library_state.select(Some(next_index % 6)); //wrapping around the last option
                    app.search_results_rendered = false;
                    app.liked_song_display = false;
                }
            }
            if app.selected_menu == Menu::Playlists {
                if app.user_playlist_tracks_selected {
                    let length = app.user_playlist_track_names.len();
                    let next_index = app.user_playlist_tracks_state.selected().unwrap_or(0) + 1;
                    app.user_playlist_tracks_state
                        .select(Some(next_index % length));
                } else {
                    let length = app.user_playlist_names.len();
                    let next_index = app.user_playlist_state.selected().unwrap_or(0) + 1;
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
                if app.selected_track {
                    let length = app.track_names.len();
                    let next_index = app.track_state.selected().unwrap_or(0) + 1;
                    app.track_state.select(Some(next_index % length));
                }
                if app.selected_album {
                    let length = app.album_names.len();
                    let next_index = app.album_state.selected().unwrap_or(0) + 1;
                    app.album_state.select(Some(next_index % length));
                }
                if app.selected_artist {
                    let length = app.artist_names.len();
                    let next_index = app.artist_state.selected().unwrap_or(0) + 1;
                    app.artist_state.select(Some(next_index % length));
                }
                if app.selected_playlist {
                    let length = app.playlist_names.len();
                    let next_index = app.playlist_state.selected().unwrap_or(0) + 1;
                    app.playlist_state.select(Some(next_index % length));
                }
            }
        }
        KeyCode::Up => {
            if app.selected_menu == Menu::Library {
                if app.library_state.selected() == Some(2) {
                    if app.liked_songs_selected {
                        let length = app.liked_song_names.len();
                        let prev_index = if app.liked_songs_state.selected().unwrap_or(0) == 0 {
                            length - 1
                        } else {
                            app.liked_songs_state.selected().unwrap_or(0) - 1
                        };
                        app.liked_songs_state.select(Some(prev_index));
                    }
                }
                if !app.liked_songs_selected {
                    let prev_index = if app.library_state.selected().unwrap_or(0) == 0 {
                        5 //wrapping to the last option when user presses up at the first option
                    } else {
                        app.library_state.selected().unwrap_or(0) - 1
                    };
                    app.library_state.select(Some(prev_index));
                    app.search_results_rendered = false;
                    app.liked_song_display = false;
                }
            }
            if app.selected_menu == Menu::Playlists {
                if app.user_playlist_tracks_selected {
                    let length = app.user_playlist_track_names.len();
                    let prev_index = if app.user_playlist_tracks_state.selected().unwrap_or(0) == 0
                    {
                        length - 1
                    } else {
                        app.user_playlist_tracks_state.selected().unwrap_or(0) - 1
                    };
                    app.user_playlist_tracks_state.select(Some(prev_index));
                } else {
                    let length = app.user_playlist_names.len();
                    let prev_index = if app.user_playlist_state.selected().unwrap_or(0) == 0 {
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
                if app.selected_track {
                    let length = app.track_names.len();
                    let prev_index = if app.track_state.selected().unwrap_or(0) == 0 {
                        length - 1
                    } else {
                        app.track_state.selected().unwrap_or(0) - 1
                    };
                    app.track_state.select(Some(prev_index));
                }
                if app.selected_album {
                    let length = app.album_names.len();
                    let prev_index = if app.album_state.selected().unwrap_or(0) == 0 {
                        length - 1
                    } else {
                        app.album_state.selected().unwrap_or(0) - 1
                    };
                    app.album_state.select(Some(prev_index));
                }
                if app.selected_artist {
                    let length = app.artist_names.len();
                    let prev_index = if app.artist_state.selected().unwrap_or(0) == 0 {
                        length - 1
                    } else {
                        app.artist_state.selected().unwrap_or(0) - 1
                    };
                    app.artist_state.select(Some(prev_index));
                }
                if app.selected_playlist {
                    let length = app.playlist_names.len();
                    let prev_index = if app.playlist_state.selected().unwrap_or(0) == 0 {
                        length - 1
                    } else {
                        app.playlist_state.selected().unwrap_or(0) - 1
                    };
                    app.playlist_state.select(Some(prev_index));
                }
            }
        }
        KeyCode::Enter => {
            if app.selected_menu == Menu::Playlists {
                if let Err(e) = fetch_playlists_tracks(app) {
                    println!("{}", e);
                }
                process_playlist_tracks(app);
                app.user_playlist_display = true;
            }
            if app.selected_menu == Menu::Library {
                if app.library_state.selected() == Some(2) {
                    if let Err(e) = liked_tracks() {
                        println!("{}", e);
                    }
                    process_liked_tracks(app);
                    app.liked_song_display = true;
                }
            }
        }
        KeyCode::Tab => {
            if app.selected_menu == Menu::Playlists {
                if app.user_playlist_display {
                    app.user_playlist_tracks_state.select(Some(0));
                    app.user_playlist_tracks_selected = !app.user_playlist_tracks_selected;
                }
            }
            if app.selected_menu == Menu::Library {
                if app.library_state.selected() == Some(2) {
                    if app.liked_song_display {
                        app.liked_songs_state.select(Some(0));
                        app.liked_songs_selected = !app.liked_songs_selected;
                    }
                }
            }
            if app.selected_search {
                let length = 4;
                let next_index = app.search_state.selected().unwrap_or(0) + 1;
                app.search_state.select(Some(next_index % length));

                if app.search_state.selected() == Some(0) {
                    app.track_state.select(Some(0));
                    app.selected_track = !app.selected_track;
                    app.selected_artist = false;
                    app.selected_album = false;
                    app.selected_playlist = false;
                } else if app.search_state.selected() == Some(1) {
                    app.artist_state.select(Some(0));
                    app.selected_artist = !app.selected_artist;
                    app.selected_track = false;
                    app.selected_album = false;
                    app.selected_playlist = false;
                } else if app.search_state.selected() == Some(2) {
                    app.album_state.select(Some(0));
                    app.selected_album = !app.selected_album;
                    app.selected_track = false;
                    app.selected_artist = false;
                    app.selected_playlist = false;
                } else if app.search_state.selected() == Some(3) {
                    app.playlist_state.select(Some(0));
                    app.selected_playlist = !app.selected_playlist;
                    app.selected_track = false;
                    app.selected_artist = false;
                    app.selected_album = false;
                }
            }
        }

        _ => {}
    }
}

pub fn search_input(app: &mut App) -> io::Result<()> {
    if let Event::Key(key) = event::read()? {
        match app.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('s') => {
                    app.input_mode = InputMode::Editing;
                    app.search_results_rendered = false;
                }
                KeyCode::Char('q') => {
                    return Ok(());
                }
                _ => {}
            },
            InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Enter => {
                    submit_message(app);
                }
                KeyCode::Char(to_insert) => {
                    enter_char(app, to_insert);
                }
                KeyCode::Backspace => {
                    delete_char(app);
                }
                KeyCode::Left => {
                    move_cursor_left(app);
                }
                KeyCode::Right => {
                    move_cursor_right(app);
                }
                KeyCode::Esc => {
                    app.input_mode = InputMode::Normal;
                    app.search_results_rendered = false;
                }
                _ => {}
            },
            InputMode::Editing => {}
            _ => {}
        }
    } else {
        return Ok(());
    }

    Ok(())
}

fn move_cursor_left(app: &mut App) {
    let cursor_moved_left = app.cursor_position.saturating_sub(1);
    app.cursor_position = clamp_cursor(app, cursor_moved_left);
}

fn move_cursor_right(app: &mut App) {
    let cursor_moved_right = app.cursor_position.saturating_add(1);
    app.cursor_position = clamp_cursor(app, cursor_moved_right);
}

fn enter_char(app: &mut App, new_char: char) {
    app.input.insert(app.cursor_position, new_char);

    move_cursor_right(app);
}

fn delete_char(app: &mut App) {
    let is_not_cursor_leftmost = app.cursor_position != 0;
    if is_not_cursor_leftmost {
        let current_index = app.cursor_position;
        let from_left_to_current_index = current_index - 1;

        // Getting all characters before the selected character.
        let before_char_to_delete = app.input.chars().take(from_left_to_current_index);
        // Getting all characters after selected character.
        let after_char_to_delete = app.input.chars().skip(current_index);

        // Put all characters together except the selected one.
        // By leaving the selected one out, it is forgotten and therefore deleted.
        app.input = before_char_to_delete.chain(after_char_to_delete).collect();
        move_cursor_left(app);
    }
}

fn clamp_cursor(app: &mut App, new_cursor_pos: usize) -> usize {
    new_cursor_pos.clamp(0, app.input.len())
}
fn reset_cursor(app: &mut App) {
    app.cursor_position = 0;
}

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
