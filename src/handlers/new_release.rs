use super::util::{default, down_key_for_table, up_key_for_table};
use crate::{
    app::App,
    enums::Menu,
    spotify::{
        new_release_section::new_releases_tracks::{
            new_releases_tracks, process_new_releases_tracks,
        },
        player::start_playback::start_playback,
    },
};

pub fn go_to_new_release_event(app: &mut App) {
    app.selected_menu = Menu::NewRelease;
    app.new_release_state.select(Some(0));
    default(app);
}

pub fn new_release_down_event(app: &mut App) {
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
}

pub fn new_release_up_event(app: &mut App) {
    if app.selected_menu == Menu::NewRelease {
        if app.new_release_album_selected {
            (app.new_release_album_state, app.new_release_index) = up_key_for_table(
                app.new_release_track_names.clone(),
                app.new_release_album_state.clone(),
            );
        } else {
            let length: usize = app.new_release_name.len();
            let prev_index: usize = if app.new_release_state.selected().unwrap_or(0) == 0 {
                length - 1
            } else {
                app.new_release_state.selected().unwrap_or(0) - 1
            };
            app.new_release_state.select(Some(prev_index));
            app.search_results_rendered = false;
            app.current_new_release = app.new_release_name[prev_index].clone();
            app.current_new_release_album_link = app.new_release_album_links[prev_index].clone();
            app.new_release_display = false;
        }
    }
}

pub fn new_release_enter_event(app: &mut App) {
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
}

pub fn new_release_tab_event(app: &mut App) {
    if app.selected_menu == Menu::NewRelease {
        app.can_navigate_menu = !app.can_navigate_menu;
        if app.new_release_display {
            app.new_release_album_state.select(Some(0));
            app.new_release_album_selected = !app.new_release_album_selected;
        }
    }
}
