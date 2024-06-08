use super::util::{default, down_key_for_table, up_key_for_table};
use crate::{
    app::App,
    enums::Menu,
    spotify::{
        player::start_playback::start_playback,
        user_playlist::user_playlist_track::{fetch_playlists_tracks, process_playlist_tracks},
    },
};

pub fn go_to_user_playlists_event(app: &mut App) {
    if app.have_playlist {
        app.selected_menu = Menu::Playlists;
        app.user_playlist_state.select(Some(0));
        default(app);
        app.selected_playlist_uri = app.user_playlist_links[0].clone();
        app.current_user_playlist = app.user_playlist_names[0].clone();
    } else {
        app.error_text = "You don't have any playlist saved".to_string();
        app.selected_menu = Menu::Error;
    }
}

pub fn user_playlist_down_event(app: &mut App) {
    if app.selected_menu == Menu::Playlists {
        if app.user_playlist_tracks_selected {
            (app.user_playlist_tracks_state, app.user_playlist_index) = down_key_for_table(
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
}

pub fn user_playlist_up_event(app: &mut App) {
    if app.selected_menu == Menu::Playlists {
        if app.user_playlist_tracks_selected {
            (app.user_playlist_tracks_state, app.user_playlist_index) = up_key_for_table(
                app.user_playlist_track_names.clone(),
                app.user_playlist_tracks_state.clone(),
            );
        } else {
            let length: usize = app.user_playlist_names.len();
            let prev_index: usize = if app.user_playlist_state.selected().unwrap_or(0) == 0 {
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
}

pub fn user_playlist_enter_event(app: &mut App) {
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
}

pub fn user_playlist_tab_event(app: &mut App) {
    if app.selected_menu == Menu::Playlists {
        app.can_navigate_menu = !app.can_navigate_menu;
        if app.user_playlist_display {
            app.user_playlist_tracks_state.select(Some(0));
            app.user_playlist_tracks_selected = !app.user_playlist_tracks_selected;
        }
    }
}
