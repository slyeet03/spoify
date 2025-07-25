use crate::LikedSongs;
use crate::UserPlaylist;
use super::util::{
    default, default_search, down_key_for_list, down_key_for_table, up_key_for_list,
    up_key_for_table,
};
use crate::{
    app::App,
    structs::Search,
    enums::{InputMode, Menu, SearchMenu},
    spotify::{
        player::start_playback::start_playback,
        search::{
            search_albums::{process_selected_album_tracks, search_selected_album_tracks},
            search_artists::{process_selected_artist_tracks, search_selected_artist_tracks},
            search_playlists::{process_selected_playlist_tracks, search_selected_playlist_tracks},
        },
    },
};

pub fn go_to_search_event(app: &mut App, search: &mut Search, userplaylist: &mut UserPlaylist, likedsongs: &mut LikedSongs) {
    app.selected_menu = Menu::Search;
    default(app,search,userplaylist,likedsongs);
    search.input_mode = InputMode::Editing;
}

pub fn search_down_event(app: &mut App, search: &mut Search) {
    if app.selected_menu == Menu::Search {
        if search.selected_search {
            if search.selected_track_in_search_result {
                (search.track_state_in_search_result, search.track_index) = down_key_for_list(
                    search.track_names_search_results.clone(),
                    search.track_state_in_search_result.clone(),
                );
            }
            if search.selected_album_in_search_result {
                (search.album_state_in_search_result, search.album_index) = down_key_for_list(
                    search.album_names_search_results.clone(),
                    search.album_state_in_search_result.clone(),
                );
            }
            if search.selected_artist_in_search_result {
                (search.artist_state_in_search_result, search.artist_index) = down_key_for_list(
                    search.artist_names_search_results.clone(),
                    search.artist_state_in_search_result.clone(),
                );
            }
            if search.selected_playlist_in_search_result {
                (search.playlist_state_in_search_result, search.playlist_index) = down_key_for_list(
                    search.playlist_names_search_results.clone(),
                    search.playlist_state_in_search_result.clone(),
                );
            }
        }
        if search.search_menu == SearchMenu::SearchedAlbum && search.searched_album_selected {
            (search.searched_album_state, search.searched_album_index) = down_key_for_table(
                search.selected_album_tracks_names.clone(),
                search.searched_album_state.clone(),
            );
        }
        if search.search_menu == SearchMenu::SearchedArtist && search.searched_artist_selected {
            (search.searched_artist_state, search.searched_artist_index) = down_key_for_table(
                search.selected_artist_tracks_names.clone(),
                search.searched_artist_state.clone(),
            );
        }
        if search.search_menu == SearchMenu::SearchedPlaylist && search.searched_playlist_selected {
            (search.searched_playlist_state, search.searched_playlist_index) = down_key_for_table(
                search.selected_playlist_tracks_names.clone(),
                search.searched_playlist_state.clone(),
            );
        }
    }
}

pub fn search_up_event(app: &mut App, search: &mut Search) {
    if app.selected_menu == Menu::Search {
        if search.selected_search {
            if search.selected_track_in_search_result {
                (search.track_state_in_search_result, search.track_index) = up_key_for_list(
                    search.track_names_search_results.clone(),
                    search.track_state_in_search_result.clone(),
                );
            }
            if search.selected_album_in_search_result {
                (search.album_state_in_search_result, search.album_index) = up_key_for_list(
                    search.album_names_search_results.clone(),
                    search.album_state_in_search_result.clone(),
                );
            }
            if search.selected_artist_in_search_result {
                (search.artist_state_in_search_result, search.artist_index) = up_key_for_list(
                    search.artist_names_search_results.clone(),
                    search.artist_state_in_search_result.clone(),
                );
            }
            if search.selected_playlist_in_search_result {
                (search.playlist_state_in_search_result, search.playlist_index) = up_key_for_list(
                    search.playlist_names_search_results.clone(),
                    search.playlist_state_in_search_result.clone(),
                );
            }
        }
        if search.search_menu == SearchMenu::SearchedAlbum && search.searched_album_selected {
            (search.searched_album_state, search.searched_album_index) = up_key_for_table(
                search.selected_album_tracks_names.clone(),
                search.searched_album_state.clone(),
            );
        }
        if search.search_menu == SearchMenu::SearchedArtist && search.searched_artist_selected {
            (search.searched_artist_state, search.searched_artist_index) = up_key_for_table(
                search.selected_artist_tracks_names.clone(),
                search.searched_artist_state.clone(),
            );
        }
        if search.search_menu == SearchMenu::SearchedPlaylist && search.searched_playlist_selected {
            (search.searched_playlist_state, search.searched_playlist_index) = up_key_for_table(
                search.selected_playlist_tracks_names.clone(),
                search.searched_playlist_state.clone(),
            );
        }
    }
}

pub fn search_enter_event(app: &mut App, search: &mut Search, likedsongs: &mut LikedSongs) {
    if app.selected_menu == Menu::Search {
        if app.is_in_track {
            app.is_only_id = true;
            app.selected_link_for_playback =
                search.track_links_search_results[search.track_index].clone();
            if let Err(e) = start_playback(app) {
                println!("{}", e);
            }
        }

        if search.search_menu == SearchMenu::SearchedAlbum {
            app.is_only_id = false;
            app.selected_link_for_playback =
                search.selected_album_tracks_links[search.searched_album_index].clone();
            if let Err(e) = start_playback(app) {
                println!("{}", e);
            }
        } else if search.selected_album_in_search_result {
            if let Err(e) = search_selected_album_tracks(app,search) {
                println!("{}", e);
            }
            process_selected_album_tracks(app,search);
            default_search(app,search,likedsongs);
            search.search_menu = SearchMenu::SearchedAlbum;
            search.searched_album_selected = true;
        }

        if search.search_menu == SearchMenu::SearchedArtist {
            app.is_only_id = false;
            app.selected_link_for_playback =
                search.selected_artist_tracks_links[search.searched_artist_index].clone();
            if let Err(e) = start_playback(app) {
                println!("{}", e);
            }
        } else if search.selected_artist_in_search_result {
            if let Err(e) = search_selected_artist_tracks(app,search) {
                println!("{}", e);
            }
            process_selected_artist_tracks(app,search);
            default_search(app,search,likedsongs);
            search.search_menu = SearchMenu::SearchedArtist;
            search.searched_artist_selected = true;
        }

        if search.search_menu == SearchMenu::SearchedPlaylist {
            app.is_only_id = false;
            app.selected_link_for_playback =
                search.selected_playlist_tracks_links[search.searched_playlist_index].clone();
            if let Err(e) = start_playback(app) {
                println!("{}", e);
            }
        } else if search.selected_playlist_in_search_result {
            if let Err(e) = search_selected_playlist_tracks(app,search) {
                println!("{}", e);
            }
            process_selected_playlist_tracks(app,search);
            default_search(app,search,likedsongs);
            search.search_menu = SearchMenu::SearchedPlaylist;
            search.searched_playlist_selected = true;
        }
    }
}

pub fn search_tab_event(app: &mut App, search: &mut Search) {
    if search.selected_search {
        app.can_navigate_menu = false;
        search.track_state_in_search_result.select(None);
        search.artist_state_in_search_result.select(None);
        search.album_state_in_search_result.select(None);
        search.playlist_state_in_search_result.select(None);

        if search.search_state.selected() == Some(0) {
            search.track_state_in_search_result.select(Some(0));
            search.selected_track_in_search_result = !search.selected_track_in_search_result;
            search.selected_artist_in_search_result = false;
            search.selected_album_in_search_result = false;
            search.selected_playlist_in_search_result = false;
            app.is_in_track = true;
        } else if search.search_state.selected() == Some(1) {
            search.artist_state_in_search_result.select(Some(0));
            search.selected_artist_in_search_result = !search.selected_artist_in_search_result;
            search.selected_track_in_search_result = false;
            search.selected_album_in_search_result = false;
            search.selected_playlist_in_search_result = false;
            app.is_in_track = false;
        } else if search.search_state.selected() == Some(2) {
            search.album_state_in_search_result.select(Some(0));
            search.selected_album_in_search_result = !search.selected_album_in_search_result;
            search.selected_track_in_search_result = false;
            search.selected_artist_in_search_result = false;
            search.selected_playlist_in_search_result = false;
            app.is_in_track = false;
        } else if search.search_state.selected() == Some(3) {
            search.playlist_state_in_search_result.select(Some(0));
            search.selected_playlist_in_search_result = !search.selected_playlist_in_search_result;
            search.selected_track_in_search_result = false;
            search.selected_artist_in_search_result = false;
            search.selected_album_in_search_result = false;
            app.is_in_track = false;
        }
        let length = 4;
        let next_index = search.search_state.selected().unwrap_or(0) + 1;
        search.search_state.select(Some(next_index % length));
    }
}
