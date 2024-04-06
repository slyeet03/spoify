use crate::app::App;
use crate::enums::{InputMode, Menu};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use std::io;
use std::thread;

use crate::spotify::search::perform_search;

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
        }
        KeyCode::Char('p') => {
            app.selected_menu = Menu::Playlists;
            app.search_results_rendered = false;
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Char('s') => {
            app.selected_menu = Menu::Search;
            app.input_mode = InputMode::Normal;
        }

        KeyCode::Char('m') => app.selected_menu = Menu::Main,
        KeyCode::Down if app.selected_menu == Menu::Library => {
            //move down in the library list
            let next_index = app.library_state.selected().unwrap_or(0) + 1;
            app.library_state.select(Some(next_index % 6)); //wrapping around the last option
            app.search_results_rendered = false;
        }
        KeyCode::Up if app.selected_menu == Menu::Library => {
            //move up in the library list
            let prev_index = if app.library_state.selected().unwrap_or(0) == 0 {
                5 //wrapping to the last option when user presses up at the first option
            } else {
                app.library_state.selected().unwrap_or(0) - 1
            };
            app.library_state.select(Some(prev_index));
            app.search_results_rendered = false;
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

    if app.input_mode == InputMode::SearchResults {
        let (tx, rx) = std::sync::mpsc::channel();
        let query = app.search_query.clone();

        let join_handle = thread::spawn(move || {
            let search_results = perform_search(&query);
            tx.send(search_results).unwrap();
        });

        if let Ok(search_results) = rx.recv() {
            app.album_names = search_results.album_names;
            app.album_links = search_results.album_links;
            app.track_names = search_results.track_names;
            app.track_links = search_results.track_links;
            app.playlist_names = search_results.playlist_names;
            app.playlist_links = search_results.playlist_links;
            app.artist_names = search_results.artist_names;
            app.artist_links = search_results.artist_links;
            app.search_results_rendered = true;
        }

        join_handle.join().unwrap();
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
    app.input.clear();
    reset_cursor(app);
    app.input_mode = InputMode::SearchResults;
}
