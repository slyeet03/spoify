use super::change_keybindings::change_keybindings;
use super::error_screen::go_to_error_event;
use super::exit::exit_event;
use super::help::go_to_help_event;
use super::library::{
    go_to_library_event, library_down_event, library_enter_event, library_tab_event,
    library_up_event,
};
use super::new_release::{
    go_to_new_release_event, new_release_down_event, new_release_enter_event,
    new_release_tab_event, new_release_up_event,
};
use super::open_configure_folder::open_config_folder;
use super::player::{
    fullscreen_player_event, next_track_event, play_pause_event, previous_track_event,
    repeat_event, shuffle_event, volume_decreament_event, volume_increment_event,
};
use super::playlist_control::add_track_to_playlist::{
    add_track_to_playlist_down_event, add_track_to_playlist_enter_event,
    add_track_to_playlist_event, add_track_to_playlist_up_event,
};
use super::playlist_control::{
    follow_playlist::follow_playlist_event, unfollow_playlist::unfollow_playlist_event,
};
use super::search::{
    go_to_search_event, search_down_event, search_enter_event, search_tab_event, search_up_event,
};
use super::user_playlist::{
    go_to_user_playlists_event, user_playlist_down_event, user_playlist_enter_event,
    user_playlist_tab_event, user_playlist_up_event,
};
use super::util::{default_nav, delete_char, move_cursor_left, move_cursor_right, reset_cursor};
use crate::app::App;
use crate::enums::{InputMode, Menu};
use crate::spotify::search::search::process_search;
use crate::structs::Key;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::io::{self, Write};

/// Function to handle key events for the application
pub fn handle_key_event(app: &mut App, key_event: KeyEvent, key: &mut Key) {
    let go_to_search_key: char = key.go_to_search_key;
    let go_to_library_key: char = key.go_to_library_key;
    let go_to_user_playlists_key: char = key.go_to_user_playlists_key;
    let exit_application_key: char = key.exit_application_key;
    let help_key: char = key.help_key;
    let volume_up_key: char = key.volume_up_key;
    let volume_down_key: char = key.volume_down_key;
    let new_release_key: char = key.new_release_key;
    let next_track_key: char = key.next_track_key;
    let previous_track_key: char = key.previous_track_key;
    let error_key: char = key.error_key;
    let player_fullscreen_key: char = key.player_fullscreen_key;
    let change_keybind: char = key.change_keybind;

    if key_event.kind == KeyEventKind::Press {
        match key_event.code {
            // Toggle shuffle mode when Ctrl+S is pressed
            KeyCode::Char('s') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                shuffle_event(app);
            }
            // Open the configuration folder
            KeyCode::Char('`') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                open_config_folder(app, key);
            }

            // Cycle through repeat options when Ctrl+R is pressed
            KeyCode::Char('r') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                repeat_event(app);
            }

            KeyCode::Char('p') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                add_track_to_playlist_event(app);
            }

            // Follow Playlist
            KeyCode::Char('f') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                follow_playlist_event(app);
            }

            //Unfollow/Delete Playlist
            KeyCode::Char('d') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                unfollow_playlist_event(app);
            }

            // Exit the application when 'q' is pressed in Normal mode
            code if code == KeyCode::Char(exit_application_key)
                && app.input_mode != InputMode::Editing =>
            {
                exit_event(app);
            }

            // Navigate to different menus (Library, Playlists, Search, New Releases) when 'l', 'p', 's' or 'n' is pressed

            // Go to Library Menu
            code if code == KeyCode::Char(go_to_library_key)
                && app.input_mode != InputMode::Editing =>
            {
                go_to_library_event(app);
            }

            // Go to user playlist menu
            code if code == KeyCode::Char(go_to_user_playlists_key)
                && app.input_mode != InputMode::Editing =>
            {
                go_to_user_playlists_event(app);
            }

            // Go to search menu
            code if code == KeyCode::Char(go_to_search_key)
                && app.input_mode != InputMode::Editing =>
            {
                go_to_search_event(app);
            }

            // Go to help menu
            code if code == KeyCode::Char(help_key) && app.input_mode != InputMode::Editing => {
                go_to_help_event(app);
            }

            // Enter fullscreen mode for the player
            code if code == KeyCode::Char(player_fullscreen_key)
                && app.input_mode != InputMode::Editing =>
            {
                fullscreen_player_event(app);
            }

            // Go to New Release Menu
            code if code == KeyCode::Char(new_release_key)
                && app.input_mode != InputMode::Editing =>
            {
                go_to_new_release_event(app);
            }

            // Keys for Volume Control
            code if code == KeyCode::Char(volume_down_key)
                && app.input_mode != InputMode::Editing =>
            {
                volume_decreament_event(app);
            }

            code if code == KeyCode::Char(volume_up_key)
                && app.input_mode != InputMode::Editing =>
            {
                volume_increment_event(app);
            }

            // Keys for next and previous track
            code if code == KeyCode::Char(next_track_key)
                && app.input_mode != InputMode::Editing =>
            {
                next_track_event(app);
            }
            code if code == KeyCode::Char(previous_track_key)
                && app.input_mode != InputMode::Editing =>
            {
                previous_track_event(app);
            }

            // Key for Error Screen
            code if code == KeyCode::Char(error_key) && app.input_mode != InputMode::Editing => {
                go_to_error_event(app);
            }

            code if code == KeyCode::Char(change_keybind)
                && app.input_mode != InputMode::Editing =>
            {
                change_keybindings(app, key);
            }

            // Down keybinding for all the menus
            KeyCode::Down if app.input_mode != InputMode::Editing => {
                library_down_event(app);
                new_release_down_event(app);
                user_playlist_down_event(app);
                search_down_event(app);
                add_track_to_playlist_down_event(app);

                if app.can_navigate_menu {
                    let next_index: usize = app.library_state.selected().unwrap_or(0) + 1;
                    app.library_state.select(Some(next_index % 6)); //wrapping around the last option
                    default_nav(app);
                }
            }

            // Up keybinding for all the menus
            KeyCode::Up if app.input_mode != InputMode::Editing => {
                library_up_event(app);
                new_release_up_event(app);
                user_playlist_up_event(app);
                search_up_event(app);
                add_track_to_playlist_up_event(app);

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
                user_playlist_enter_event(app);
                new_release_enter_event(app);
                library_enter_event(app);
                search_enter_event(app);
                add_track_to_playlist_enter_event(app);
            }

            // Tab keybinding for all the menus
            KeyCode::Tab if app.input_mode != InputMode::Editing => {
                user_playlist_tab_event(app);
                new_release_tab_event(app);
                library_tab_event(app);
                search_tab_event(app);
            }

            // Pause/Play using Spacebar
            KeyCode::Char(' ') if app.input_mode != InputMode::Editing => {
                play_pause_event(app);
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
            InputMode::Normal => {}
            InputMode::SearchResults => {}
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
