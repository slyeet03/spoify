use crate::LikedSongs;
use crate::UserPlaylist;
use super::util::{default, down_key_for_table, up_key_for_table};
use crate::{
    app::App,
    structs::Search,
    enums::Menu,
    spotify::{
        player::start_playback::start_playback,
        user_playlist::user_playlist_track::{fetch_playlists_tracks, process_playlist_tracks},
    },
};

pub fn go_to_user_playlists_event(app: &mut App,search:&mut Search, userplaylist: &mut UserPlaylist, likedsongs: &mut LikedSongs) {
    if app.have_playlist {
        app.selected_menu = Menu::Playlists;
        userplaylist.user_playlist_state.select(Some(0));
        default(app,search,userplaylist,likedsongs);
        userplaylist.selected_playlist_uri = userplaylist.user_playlist_links[0].clone();
        userplaylist.current_user_playlist = userplaylist.user_playlist_names[0].clone();
    } else {
        app.error_text = "You don't have any playlist saved".to_string();
        app.selected_menu = Menu::Error;
    }
}

pub fn user_playlist_down_event(app: &mut App, search: &mut Search, userplaylist: &mut UserPlaylist) {
    if app.selected_menu == Menu::Playlists {
        if userplaylist.user_playlist_tracks_selected {
            (userplaylist.user_playlist_tracks_state, userplaylist.user_playlist_index) = down_key_for_table(
                userplaylist.user_playlist_track_names.clone(),
                userplaylist.user_playlist_tracks_state.clone(),
            );
        } else {
            let length: usize = userplaylist.user_playlist_names.len();
            let next_index: usize = userplaylist.user_playlist_state.selected().unwrap_or(0) + 1;
            userplaylist.user_playlist_state.select(Some(next_index % length));
            search.search_results_rendered = false;
            if next_index >= length {
            } else {
                userplaylist.selected_playlist_uri = userplaylist.user_playlist_links[next_index].clone();
                userplaylist.current_user_playlist = userplaylist.user_playlist_names[next_index].clone();
            }
            userplaylist.user_playlist_display = false;
        }
    }
}

pub fn user_playlist_up_event(app: &mut App,search: &mut Search, userplaylist: &mut UserPlaylist) {
    if app.selected_menu == Menu::Playlists {
        if userplaylist.user_playlist_tracks_selected {
            (userplaylist.user_playlist_tracks_state, userplaylist.user_playlist_index) = up_key_for_table(
                userplaylist.user_playlist_track_names.clone(),
                userplaylist.user_playlist_tracks_state.clone(),
            );
        } else {
            let length: usize = userplaylist.user_playlist_names.len();
            let prev_index: usize = if userplaylist.user_playlist_state.selected().unwrap_or(0) == 0 {
                length - 1
            } else {
                userplaylist.user_playlist_state.selected().unwrap_or(0) - 1
            };
            userplaylist.user_playlist_state.select(Some(prev_index));
            search.search_results_rendered = false;
            userplaylist.selected_playlist_uri = userplaylist.user_playlist_links[prev_index].clone();
            userplaylist.current_user_playlist = userplaylist.user_playlist_names[prev_index].clone();
            userplaylist.user_playlist_display = false;
        }
    }
}

pub fn user_playlist_enter_event(app: &mut App, search: &mut Search, userplaylist: &mut UserPlaylist) {
    if app.selected_menu == Menu::Playlists {
        if userplaylist.enter_for_playback_in_user_playlist {
            app.selected_link_for_playback =
                userplaylist.user_playlist_track_links[userplaylist.user_playlist_index].clone();
            if let Err(e) = start_playback(app) {
                println!("{}", e);
            }
        } else {
            if let Err(e) = fetch_playlists_tracks(app,userplaylist) {
                println!("{}", e);
            }
            process_playlist_tracks(app,userplaylist);
            userplaylist.user_playlist_display = true;
            search.searched_album_selected = false;
            search.searched_artist_selected = false;
            search.searched_playlist_selected = false;
            userplaylist.enter_for_playback_in_user_playlist = true;
        }
    }
}

pub fn user_playlist_tab_event(app: &mut App, userplaylist: &mut UserPlaylist) {
    if app.selected_menu == Menu::Playlists {
        app.can_navigate_menu = !app.can_navigate_menu;
        if userplaylist.user_playlist_display {
            userplaylist.user_playlist_tracks_state.select(Some(0));
            userplaylist.user_playlist_tracks_selected = !userplaylist.user_playlist_tracks_selected;
        }
    }
}
