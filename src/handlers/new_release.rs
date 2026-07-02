use crate::structs::NewRelease;
use crate::UserSavedArtist;
use crate::structs::UserRecentlyPlayed;
use crate::UserSavedPodcast;
use crate::MadeFY;
use crate::UserSavedAlbums;
use crate::LikedSongs;
use crate::UserPlaylist;
use crate::UserCurrentlyPlaying;
use super::util::{default, down_key_for_table, up_key_for_table};
use crate::{
    app::App,
    enums::Menu,
    structs::Search,
    spotify::{
        new_release_section::new_releases_tracks::{
            new_releases_tracks, process_new_releases_tracks,
        },
        player::start_playback::start_playback,
    },
};

pub fn go_to_new_release_event(app: &mut App, search:&mut Search, userplaylist: &mut UserPlaylist, likedsongs: &mut LikedSongs, useralbum: &mut UserSavedAlbums, madefy: &mut MadeFY,podcast: &mut UserSavedPodcast, recentlyplayed: &mut UserRecentlyPlayed, userartist: &mut UserSavedArtist, newrelease: &mut NewRelease) {
    app.selected_menu = Menu::NewRelease;
    newrelease.new_release_state.select(Some(0));
    default(app,search,userplaylist,likedsongs,useralbum,madefy,podcast,recentlyplayed,userartist, newrelease);
}

pub fn new_release_down_event(app: &mut App, search: &mut Search, newrelease: &mut NewRelease) {
    if app.selected_menu == Menu::NewRelease {
        if newrelease.new_release_album_selected {
            (newrelease.new_release_album_state, newrelease.new_release_index) = down_key_for_table(
                newrelease.new_release_track_names.clone(),
                newrelease.new_release_album_state.clone(),
            );
        } else {
            let length: usize = newrelease.new_release_name.len();
            let next_index: usize = newrelease.new_release_state.selected().unwrap_or(0) + 1;
            newrelease.new_release_state.select(Some(next_index % length));
            search.search_results_rendered = false;
            if next_index >= length {
            } else {
                newrelease.current_new_release = newrelease.new_release_name[next_index].clone();
                newrelease.current_new_release_album_link =
                    newrelease.new_release_album_links[next_index].clone();
            }
            newrelease.new_release_display = false;
        }
    }
}

pub fn new_release_up_event(app: &mut App, search: &mut Search, newrelease: &mut NewRelease) {
    if app.selected_menu == Menu::NewRelease {
        if newrelease.new_release_album_selected {
            (newrelease.new_release_album_state, newrelease.new_release_index) = up_key_for_table(
                newrelease.new_release_track_names.clone(),
                newrelease.new_release_album_state.clone(),
            );
        } else {
            let length: usize = newrelease.new_release_name.len();
            let prev_index: usize = if newrelease.new_release_state.selected().unwrap_or(0) == 0 {
                length - 1
            } else {
                newrelease.new_release_state.selected().unwrap_or(0) - 1
            };
            newrelease.new_release_state.select(Some(prev_index));
            search.search_results_rendered = false;
            newrelease.current_new_release = newrelease.new_release_name[prev_index].clone();
            newrelease.current_new_release_album_link = newrelease.new_release_album_links[prev_index].clone();
            newrelease.new_release_display = false;
        }
    }
}

pub fn new_release_enter_event(app: &mut App, search: &mut Search, newrelease: &mut NewRelease, currentlyplaying: &mut UserCurrentlyPlaying) {
    if app.selected_menu == Menu::NewRelease {
        if newrelease.enter_for_playback_in_new_release {
            app.selected_link_for_playback =
                newrelease.new_release_spotify_urls[newrelease.new_release_index].clone();
            if let Err(e) = start_playback(app, currentlyplaying) {
                println!("{}", e);
            }
        } else {
            if let Err(e) = new_releases_tracks(app, newrelease) {
                println!("{}", e);
            }
            process_new_releases_tracks(app, newrelease);
            newrelease.new_release_display = true;
            search.searched_album_selected = false;
            search.searched_artist_selected = false;
            search.searched_playlist_selected = false;
            newrelease.enter_for_playback_in_new_release = true;
        }
    }
}

pub fn new_release_tab_event(app: &mut App, newrelease: &mut NewRelease) {
    if app.selected_menu == Menu::NewRelease {
        app.can_navigate_menu = !app.can_navigate_menu;
        if newrelease.new_release_display {
            newrelease.new_release_album_state.select(Some(0));
            newrelease.new_release_album_selected = !newrelease.new_release_album_selected;
        }
    }
}
