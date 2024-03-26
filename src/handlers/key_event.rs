use crate::app::App;
use crate::enums::{InputMode, Menu};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use std::io;

use crate::spotify::query_storage::query_storage;

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
        }
        KeyCode::Char('p') => app.selected_menu = Menu::Playlists,
        KeyCode::Char('s') => {
            app.selected_menu = Menu::Search;
        }

        KeyCode::Char('m') => app.selected_menu = Menu::Main,
        KeyCode::Down if app.selected_menu == Menu::Library => {
            //move down in the library list
            let next_index = app.library_state.selected().unwrap_or(0) + 1;
            app.library_state.select(Some(next_index % 6)); //wrapping around the last option
        }
        KeyCode::Up if app.selected_menu == Menu::Library => {
            //move up in the library list
            let prev_index = if app.library_state.selected().unwrap_or(0) == 0 {
                5 //wrapping to the last option when user presses up at the first option
            } else {
                app.library_state.selected().unwrap_or(0) - 1
            };
            app.library_state.select(Some(prev_index));
        }
        _ => {}
    }
}

pub fn search_input(app: &mut App) -> io::Result<()> {
    let data_dir = std::path::Path::new("./data");
    if let Event::Key(key) = event::read()? {
        match app.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('s') => {
                    app.input_mode = InputMode::Editing;
                }
                KeyCode::Char('q') => {
                    return Ok(());
                }
                _ => {}
            },
            InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Enter => {
                    app.search_query = app.input.clone();
                    app.input.clear();
                    app.input_mode = InputMode::SearchResults;
                    //submit_message(app);
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
        let query = app.search_query.as_str();
        let (album_names, album_links, track_names, track_links, playlist_names, playlist_links) =
            query_storage(query, data_dir).unwrap_or_default();

        app.album_names = album_names;
        app.album_links = album_links;
        app.track_names = track_names;
        app.track_links = track_links;
        app.playlist_names = playlist_names;
        app.playlist_links = playlist_links;
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
        // Method "remove" is not used on the saved text for deleting the selected char.
        // Reason: Using remove on String works on bytes instead of the chars.
        // Using remove would require special care because of char boundaries.

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
