use crate::structs::NewRelease;
use crate::structs::UserSavedArtist;
use crate::structs::UserRecentlyPlayed;
use crate::structs::UserSavedPodcast;
use crate::structs::MadeFY;
use crate::UserSavedAlbums;
use crate::LikedSongs;
use crate::UserPlaylist;
use crate::UserCurrentlyPlaying;
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
use super::refresh::refresh_event;
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
use crate::structs::{Key, Settings, Themes, Search};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::io::{self, Write};

/// Function to handle key events for the application
pub fn handle_key_event(
    app: &mut App,
    key_event: KeyEvent,
    key: &mut Key,
    theme: &mut Themes,
    settings: &mut Settings,
    search: &mut Search, 
    userplaylist: &mut UserPlaylist, 
    likedsongs: &mut LikedSongs,
    useralbum: &mut UserSavedAlbums,
    madefy: &mut MadeFY,
    podcast: &mut UserSavedPodcast, 
    recentlyplayed: &mut UserRecentlyPlayed, 
    userartist: &mut UserSavedArtist, 
    newrelease: &mut NewRelease,
    currentlyplaying: &mut UserCurrentlyPlaying,
) {
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
    let refresh_key: char = key.refresh_key;
    let open_config_fold_key: char = key.open_config_fold_key;

    if key_event.kind == KeyEventKind::Press {
        match key_event.code {
            // Toggle shuffle mode when Ctrl+S is pressed
            KeyCode::Char('s') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                shuffle_event(app,currentlyplaying);
            }
            // Open the configuration folder
            code if code == KeyCode::Char(open_config_fold_key)
                && search.input_mode != InputMode::Editing =>
            {
                open_config_folder(app, key);
            }

            // Cycle through repeat options when Ctrl+R is pressed
            KeyCode::Char('r') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                repeat_event(app,currentlyplaying);
            }

            KeyCode::Char('p') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                add_track_to_playlist_event(app,search,userplaylist,likedsongs,useralbum,madefy,recentlyplayed,userartist,newrelease);
            }

            // Follow Playlist
            KeyCode::Char('f') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                follow_playlist_event(app,search,userplaylist);
            }

            //Unfollow/Delete Playlist
            KeyCode::Char('d') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                unfollow_playlist_event(app,userplaylist);
            }

            // Exit the application when 'q' is pressed in Normal mode
            code if code == KeyCode::Char(exit_application_key)
                && search.input_mode != InputMode::Editing =>
            {
                exit_event(app,useralbum,madefy,userartist);
            }

            // Run the startup function again
            code if code == KeyCode::Char(refresh_key) && search.input_mode != InputMode::Editing => {
                refresh_event(app, key, theme, settings,userplaylist,newrelease);
            }

            // Navigate to different menus (Library, Playlists, Search, New Releases) when 'l', 'p', 's' or 'n' is pressed

            // Go to Library Menu
            code if code == KeyCode::Char(go_to_library_key)
                && search.input_mode != InputMode::Editing =>
            {
                go_to_library_event(app,search,userplaylist,likedsongs,useralbum,madefy,podcast,recentlyplayed,userartist, newrelease);
            }

            // Go to user playlist menu
            code if code == KeyCode::Char(go_to_user_playlists_key)
                && search.input_mode != InputMode::Editing =>
            {
                go_to_user_playlists_event(app,search,userplaylist,likedsongs,useralbum,madefy,podcast,recentlyplayed,userartist,newrelease);
            }

            // Go to search menu
            code if code == KeyCode::Char(go_to_search_key)
                && search.input_mode != InputMode::Editing =>
            {
                go_to_search_event(app,search,userplaylist,likedsongs,useralbum,madefy,podcast,recentlyplayed,userartist, newrelease);
            }

            // Go to help menu
            code if code == KeyCode::Char(help_key) && search.input_mode != InputMode::Editing => {
                go_to_help_event(app);
            }

            // Enter fullscreen mode for the player
            code if code == KeyCode::Char(player_fullscreen_key)
                && search.input_mode != InputMode::Editing =>
            {
                fullscreen_player_event(app);
            }

            // Go to New Release Menu
            code if code == KeyCode::Char(new_release_key)
                && search.input_mode != InputMode::Editing =>
            {
                go_to_new_release_event(app,search,userplaylist,likedsongs,useralbum,madefy,podcast,recentlyplayed,userartist,newrelease);
            }

            // Keys for Volume Control
            code if code == KeyCode::Char(volume_down_key)
                && search.input_mode != InputMode::Editing =>
            {
                volume_decreament_event(app, settings);
            }

            code if code == KeyCode::Char(volume_up_key)
                && search.input_mode != InputMode::Editing =>
            {
                volume_increment_event(app, settings);
            }

            // Keys for next and previous track
            code if code == KeyCode::Char(next_track_key)
                && search.input_mode != InputMode::Editing =>
            {
                next_track_event(app,currentlyplaying);
            }
            code if code == KeyCode::Char(previous_track_key)
                && search.input_mode != InputMode::Editing =>
            {
                previous_track_event(app, currentlyplaying);
            }

            // Key for Error Screen
            code if code == KeyCode::Char(error_key) && search.input_mode != InputMode::Editing => {
                go_to_error_event(app);
            }

            code if code == KeyCode::Char(change_keybind)
                && search.input_mode != InputMode::Editing =>
            {
                change_keybindings(app, key);
            }

            // Down keybinding for all the menus
            KeyCode::Down if search.input_mode != InputMode::Editing => {
                library_down_event(app,likedsongs,useralbum,madefy,podcast,recentlyplayed,userartist);
                new_release_down_event(app,search,newrelease);
                user_playlist_down_event(app,search,userplaylist);
                search_down_event(app,search);
                add_track_to_playlist_down_event(app,userplaylist);

                if app.can_navigate_menu {
                    let next_index: usize = app.library_state.selected().unwrap_or(0) + 1;
                    app.library_state.select(Some(next_index % 6)); //wrapping around the last option
                    default_nav(app,search,userplaylist,likedsongs,useralbum,madefy,podcast,recentlyplayed,userartist,newrelease);
                }
            }

            // Up keybinding for all the menus
            KeyCode::Up if search.input_mode != InputMode::Editing => {
                library_up_event(app,likedsongs,useralbum,madefy,podcast,recentlyplayed,userartist);
                new_release_up_event(app,search,newrelease);
                user_playlist_up_event(app,search,userplaylist);
                search_up_event(app,search);
                add_track_to_playlist_up_event(app,userplaylist);

                if app.can_navigate_menu {
                    let prev_index = if app.library_state.selected().unwrap_or(0) == 0 {
                        5 //wrapping to the last option when user presses up at the first option
                    } else {
                        app.library_state.selected().unwrap_or(0) - 1
                    };
                    app.library_state.select(Some(prev_index));
                    default_nav(app,search,userplaylist,likedsongs,useralbum,madefy,podcast,recentlyplayed,userartist,newrelease);
                }
            }

            // Enter keybinding for all the menus
            KeyCode::Enter if search.input_mode != InputMode::Editing => {
                user_playlist_enter_event(app,search,userplaylist, currentlyplaying);
                new_release_enter_event(app,search,newrelease, currentlyplaying);
                library_enter_event(app,search,likedsongs,useralbum,madefy,podcast,recentlyplayed,userartist, currentlyplaying);
                search_enter_event(app,search,likedsongs,useralbum,podcast,recentlyplayed,userartist,currentlyplaying);
                add_track_to_playlist_enter_event(app,userplaylist);
            }

            // Tab keybinding for all the menus
            KeyCode::Tab if search.input_mode != InputMode::Editing => {
                user_playlist_tab_event(app,userplaylist);
                new_release_tab_event(app,newrelease);
                library_tab_event(app,likedsongs,useralbum,madefy,podcast,recentlyplayed,userartist);
                search_tab_event(app,search);
            }

            // Pause/Play using Spacebar
            KeyCode::Char(' ') if search.input_mode != InputMode::Editing => {
                play_pause_event(app, currentlyplaying);
            }

            // Just exit from Search Menu
            KeyCode::Esc if search.input_mode != InputMode::Editing => {
                app.selected_menu = Menu::Default;
            }

            // Handle character input in search mode
            KeyCode::Char(c) if search.input_mode == InputMode::Editing => {
                // Handle character input in search mode
                if !c.is_control() {
                    search.input.push(c);
                    move_cursor_right(search);
                }
            }

            _ => {}
        }
    }
}

/// Function to handle search input and related key events
pub fn search_input(app: &mut App,search: &mut Search, key_event: KeyEvent) -> io::Result<()> {
    if key_event.kind == KeyEventKind::Press {
        match search.input_mode {
            InputMode::Editing => match key_event.code {
                // Submit the search query when Enter is pressed
                KeyCode::Enter => {
                    submit_message(app,search);
                    std::io::sink().write_all(&[0])?;
                }
                // Delete a character when Backspace is pressed
                KeyCode::Backspace => {
                    delete_char(search);
                    std::io::sink().write_all(&[0])?;
                }
                // Move the cursor left when Left arrow is pressed
                KeyCode::Left => {
                    move_cursor_left(search);
                    std::io::sink().write_all(&[0])?;
                }
                // Move the cursor right when Right arrow is pressed
                KeyCode::Right => {
                    move_cursor_right(search);
                    std::io::sink().write_all(&[0])?;
                }
                // Exit search mode when Esc is pressed
                KeyCode::Esc => {
                    search.input_mode = InputMode::Normal;
                    search.search_results_rendered = false;
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
fn submit_message(app: &mut App,search: &mut Search) {
    search.search_query = search.input.clone();

    let binding = search.search_query.clone();
    let query = binding.as_str();

    let _ = process_search(app, query,search);

    search.input.clear();
    reset_cursor(search);

    search.input_mode = InputMode::SearchResults;
    search.search_results_rendered = true;
    search.selected_search = true;
    search.search_state.select(Some(0));
}
