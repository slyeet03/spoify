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
    newrelease.state.select(Some(0));
    default(app,search,userplaylist,likedsongs,useralbum,madefy,podcast,recentlyplayed,userartist, newrelease);
}

pub fn new_release_down_event(app: &mut App, search: &mut Search, newrelease: &mut NewRelease) {
    if app.selected_menu == Menu::NewRelease {
        if newrelease.album_selected {
            (newrelease.album_state, newrelease.index) = down_key_for_table(
                newrelease.track_names.clone(),
                newrelease.album_state.clone(),
            );
        } else {
            let length: usize = newrelease.name.len();
            let next_index: usize = newrelease.state.selected().unwrap_or(0) + 1;
            newrelease.state.select(Some(next_index % length));
            search.results_rendered = false;
            if next_index >= length {
            } else {
                newrelease.current = newrelease.name[next_index].clone();
                newrelease.current_album_link =
                    newrelease.album_links[next_index].clone();
            }
            newrelease.display = false;
        }
    }
}

pub fn new_release_up_event(app: &mut App, search: &mut Search, newrelease: &mut NewRelease) {
    if app.selected_menu == Menu::NewRelease {
        if newrelease.album_selected {
            (newrelease.album_state, newrelease.index) = up_key_for_table(
                newrelease.track_names.clone(),
                newrelease.album_state.clone(),
            );
        } else {
            let length: usize = newrelease.name.len();
            let prev_index: usize = if newrelease.state.selected().unwrap_or(0) == 0 {
                length - 1
            } else {
                newrelease.state.selected().unwrap_or(0) - 1
            };
            newrelease.state.select(Some(prev_index));
            search.results_rendered = false;
            newrelease.current = newrelease.name[prev_index].clone();
            newrelease.current_album_link = newrelease.album_links[prev_index].clone();
            newrelease.display = false;
        }
    }
}

pub fn new_release_enter_event(app: &mut App, search: &mut Search, newrelease: &mut NewRelease, currentlyplaying: &mut UserCurrentlyPlaying) {
    if app.selected_menu == Menu::NewRelease {
        if newrelease.enter_for_playback_in_new_release {
            app.selected_link_for_playback =
                newrelease.spotify_urls[newrelease.index].clone();
            if let Err(e) = start_playback(app, currentlyplaying) {
                println!("{}", e);
            }
        } else {
            if let Err(e) = new_releases_tracks(app, newrelease) {
                println!("{}", e);
            }
            process_new_releases_tracks(app, newrelease);
            newrelease.display = true;
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
        if newrelease.display {
            newrelease.album_state.select(Some(0));
            newrelease.album_selected = !newrelease.album_selected;
        }
    }
}
