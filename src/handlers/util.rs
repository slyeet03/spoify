use crate::structs::NewRelease;
use crate::structs::UserSavedArtist;
use crate::structs::UserRecentlyPlayed;
use crate::structs::UserSavedPodcast;
use crate::MadeFY;
use crate::UserSavedAlbums;
use crate::LikedSongs;
use crate::UserPlaylist;
use ratatui::widgets::{ListState, TableState};

use crate::{
    app::App,
    enums::{InputMode, SearchMenu},
    structs::Search,
};

// Helper functions for cursor movement and character deletion
pub fn move_cursor_left(search: &mut Search) {
    let cursor_moved_left = search.cursor_position.saturating_sub(1);
    search.cursor_position = clamp_cursor(cursor_moved_left,search);
}

pub fn move_cursor_right(search: &mut Search) {
    let cursor_moved_right = search.cursor_position.saturating_add(1);
    search.cursor_position = clamp_cursor(cursor_moved_right,search);
}

pub fn delete_char(search: &mut Search) {
    let is_not_cursor_leftmost = search.cursor_position != 0;
    if is_not_cursor_leftmost {
        let current_index = search.cursor_position;
        let from_left_to_current_index = current_index - 1;

        // Getting all characters before the selected character.
        let before_char_to_delete = search.input.chars().take(from_left_to_current_index);
        // Getting all characters after selected character.
        let after_char_to_delete = search.input.chars().skip(current_index);

        // Put all characters together except the selected one.
        // By leaving the selected one out, it is forgotten and therefore deleted.
        search.input = before_char_to_delete.chain(after_char_to_delete).collect();
        move_cursor_left(search);
    }
}

pub fn clamp_cursor(new_cursor_pos: usize, search: &mut Search) -> usize {
    new_cursor_pos.clamp(0, search.input.len())
}
pub fn reset_cursor(search: &mut Search) {
    search.cursor_position = 0;
}

pub fn down_key_for_table(names: Vec<String>, mut state: TableState) -> (TableState, usize) {
    let length: usize = names.len();
    let next_index: usize = state.selected().unwrap_or(0) + 1;
    state.select(Some(next_index % length));

    (state, next_index)
}

pub fn down_key_for_list(names: Vec<String>, mut state: ListState) -> (ListState, usize) {
    let length: usize = names.len();
    let next_index: usize = state.selected().unwrap_or(0) + 1;
    state.select(Some(next_index % length));

    (state, next_index)
}

pub fn up_key_for_table(names: Vec<String>, mut state: TableState) -> (TableState, usize) {
    let length: usize = names.len();
    let prev_index: usize = if state.selected().unwrap_or(0) == 0 {
        length - 1
    } else {
        state.selected().unwrap_or(0) - 1
    };
    state.select(Some(prev_index));

    (state, prev_index)
}

pub fn up_key_for_list(names: Vec<String>, mut state: ListState) -> (ListState, usize) {
    let length: usize = names.len();
    let prev_index: usize = if state.selected().unwrap_or(length) == 0 {
        length - 1
    } else {
        state.selected().unwrap_or(length) - 1
    };
    state.select(Some(prev_index));

    (state, prev_index)
}

pub fn default(
    app: &mut App, search: &mut Search,
    userplaylist: &mut UserPlaylist,
    likedsongs: &mut LikedSongs,
    useralbum: &mut UserSavedAlbums,
    madefy: &mut MadeFY,
    podcast: &mut UserSavedPodcast,
    recentlyplayed: &mut UserRecentlyPlayed, 
    userartist: &mut UserSavedArtist, 
    newrelease: &mut NewRelease
    ) {
    search.results_rendered = false;
    search.input_mode = InputMode::Normal;
    userplaylist.display = false;
    likedsongs.display = false;
    search.selected_search = false;
    useralbum.display = false;
    recentlyplayed.display = false;
    app.can_navigate_menu = true;
    podcast.display = false;
    userartist.user_artist_display = false;
    search.searched_album_selected = false;
    search.searched_artist_selected = false;
    search.searched_playlist_selected = false;
    madefy.display = false;
    madefy.track_display = false;
    madefy.track_selected = false;
    madefy.current_playlist_selected = false;
    useralbum.current_album_selected = false;
    useralbum.track_selected = false;
    useralbum.track_display = false;
    userartist.user_artist_current_artist_selected = false;
    userartist.user_artist_track_selected = false;
    userartist.user_artist_track_display = false;
    madefy.enter_for_playback_in_made_fy = false;
    likedsongs.enter_for_playback_in_liked_song = false;
    useralbum.enter_for_playback_in_user_album = false;
    recentlyplayed.enter_for_playback_in_recently_played = false;
    userartist.enter_for_playback_in_saved_artist = false;
    userplaylist.enter_for_playback_in_user_playlist = false;
    newrelease.enter_for_playback_in_new_release = false;
    app.is_only_id = false;
    app.selected_link_for_playback.clear();
    app.is_in_track = false;
    search.menu = SearchMenu::Default;
}

pub fn default_nav(app: &mut App, search: &mut Search, userplaylist: &mut UserPlaylist, likedsongs: &mut LikedSongs, useralbum: &mut UserSavedAlbums, madefy: &mut MadeFY,podcast: &mut UserSavedPodcast, recentlyplayed: &mut UserRecentlyPlayed, userartist: &mut UserSavedArtist, newrelease: &mut NewRelease) {
    search.results_rendered = false;
    likedsongs.display = false;
    useralbum.display = false;
    recentlyplayed.display = false;
    podcast.display = false;
    userartist.user_artist_display = false;
    madefy.display = false;
    madefy.track_display = false;
    madefy.track_selected = false;
    useralbum.current_album_selected = false;
    useralbum.track_selected = false;
    useralbum.track_display = false;
    userartist.user_artist_current_artist_selected = false;
    userartist.user_artist_track_selected = false;
    userartist.user_artist_track_display = false;
    madefy.enter_for_playback_in_made_fy = false;
    likedsongs.enter_for_playback_in_liked_song = false;
    useralbum.enter_for_playback_in_user_album = false;
    recentlyplayed.enter_for_playback_in_recently_played = false;
    userartist.enter_for_playback_in_saved_artist = false;
    userplaylist.enter_for_playback_in_user_playlist = false;
    newrelease.enter_for_playback_in_new_release = false;
    app.is_only_id = false;
    app.selected_link_for_playback.clear();
    app.is_in_track = false;
}

pub fn default_search(app: &mut App, search: &mut Search, likedsongs: &mut LikedSongs, useralbum: &mut UserSavedAlbums,podcast: &mut UserSavedPodcast, recentlyplayed: &mut UserRecentlyPlayed, userartist: &mut UserSavedArtist) {
    search.results_rendered = false;
    likedsongs.display = false;
    useralbum.display = false;
    recentlyplayed.display = false;
    app.can_navigate_menu = true;
    podcast.display = false;
    userartist.user_artist_display = false;
    search.searched_album_selected = false;
    search.searched_artist_selected = false;
    search.searched_album_selected = false;
    search.searched_playlist_selected = false;
    search.selected_search = false;
    app.is_only_id = false;
    app.selected_link_for_playback.clear();
    app.is_in_track = false;
    search.menu = SearchMenu::Default;
}
