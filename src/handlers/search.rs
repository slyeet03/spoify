use super::util::{
    default, default_search, down_key_for_list, down_key_for_table, up_key_for_list,
    up_key_for_table,
};
use crate::{
    app::App,
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

pub fn go_to_search_event(app: &mut App) {
    app.selected_menu = Menu::Search;
    default(app);
    app.input_mode = InputMode::Editing;
}

pub fn search_down_event(app: &mut App) {
    if app.selected_menu == Menu::Search {
        if app.selected_search {
            if app.selected_track_in_search_result {
                (app.track_state_in_search_result, app.track_index) = down_key_for_list(
                    app.track_names_search_results.clone(),
                    app.track_state_in_search_result.clone(),
                );
            }
            if app.selected_album_in_search_result {
                (app.album_state_in_search_result, app.album_index) = down_key_for_list(
                    app.album_names_search_results.clone(),
                    app.album_state_in_search_result.clone(),
                );
            }
            if app.selected_artist_in_search_result {
                (app.artist_state_in_search_result, app.artist_index) = down_key_for_list(
                    app.artist_names_search_results.clone(),
                    app.artist_state_in_search_result.clone(),
                );
            }
            if app.selected_playlist_in_search_result {
                (app.playlist_state_in_search_result, app.playlist_index) = down_key_for_list(
                    app.playlist_names_search_results.clone(),
                    app.playlist_state_in_search_result.clone(),
                );
            }
        }
        if app.search_menu == SearchMenu::SearchedAlbum && app.searched_album_selected {
            (app.searched_album_state, app.searched_album_index) = down_key_for_table(
                app.selected_album_tracks_names.clone(),
                app.searched_album_state.clone(),
            );
        }
        if app.search_menu == SearchMenu::SearchedArtist && app.searched_artist_selected {
            (app.searched_artist_state, app.searched_artist_index) = down_key_for_table(
                app.selected_artist_tracks_names.clone(),
                app.searched_artist_state.clone(),
            );
        }
        if app.search_menu == SearchMenu::SearchedPlaylist && app.searched_playlist_selected {
            (app.searched_playlist_state, app.searched_playlist_index) = down_key_for_table(
                app.selected_playlist_tracks_names.clone(),
                app.searched_playlist_state.clone(),
            );
        }
    }
}

pub fn search_up_event(app: &mut App) {
    if app.selected_menu == Menu::Search {
        if app.selected_search {
            if app.selected_track_in_search_result {
                (app.track_state_in_search_result, app.track_index) = up_key_for_list(
                    app.track_names_search_results.clone(),
                    app.track_state_in_search_result.clone(),
                );
            }
            if app.selected_album_in_search_result {
                (app.album_state_in_search_result, app.album_index) = up_key_for_list(
                    app.album_names_search_results.clone(),
                    app.album_state_in_search_result.clone(),
                );
            }
            if app.selected_artist_in_search_result {
                (app.artist_state_in_search_result, app.artist_index) = up_key_for_list(
                    app.artist_names_search_results.clone(),
                    app.artist_state_in_search_result.clone(),
                );
            }
            if app.selected_playlist_in_search_result {
                (app.playlist_state_in_search_result, app.playlist_index) = up_key_for_list(
                    app.playlist_names_search_results.clone(),
                    app.playlist_state_in_search_result.clone(),
                );
            }
        }
        if app.search_menu == SearchMenu::SearchedAlbum && app.searched_album_selected {
            (app.searched_album_state, app.searched_album_index) = up_key_for_table(
                app.selected_album_tracks_names.clone(),
                app.searched_album_state.clone(),
            );
        }
        if app.search_menu == SearchMenu::SearchedArtist && app.searched_artist_selected {
            (app.searched_artist_state, app.searched_artist_index) = up_key_for_table(
                app.selected_artist_tracks_names.clone(),
                app.searched_artist_state.clone(),
            );
        }
        if app.search_menu == SearchMenu::SearchedPlaylist && app.searched_playlist_selected {
            (app.searched_playlist_state, app.searched_playlist_index) = up_key_for_table(
                app.selected_playlist_tracks_names.clone(),
                app.searched_playlist_state.clone(),
            );
        }
    }
}

pub fn search_enter_event(app: &mut App) {
    if app.selected_menu == Menu::Search {
        if app.is_in_track {
            app.is_only_id = true;
            app.selected_link_for_playback =
                app.track_links_search_results[app.track_index].clone();
            if let Err(e) = start_playback(app) {
                println!("{}", e);
            }
        }

        if app.search_menu == SearchMenu::SearchedAlbum {
            app.is_only_id = false;
            app.selected_link_for_playback =
                app.selected_album_tracks_links[app.searched_album_index].clone();
            if let Err(e) = start_playback(app) {
                println!("{}", e);
            }
        } else if app.selected_album_in_search_result {
            if let Err(e) = search_selected_album_tracks(app) {
                println!("{}", e);
            }
            process_selected_album_tracks(app);
            default_search(app);
            app.search_menu = SearchMenu::SearchedAlbum;
            app.searched_album_selected = true;
        }

        if app.search_menu == SearchMenu::SearchedArtist {
            app.is_only_id = false;
            app.selected_link_for_playback =
                app.selected_artist_tracks_links[app.searched_artist_index].clone();
            if let Err(e) = start_playback(app) {
                println!("{}", e);
            }
        } else if app.selected_artist_in_search_result {
            if let Err(e) = search_selected_artist_tracks(app) {
                println!("{}", e);
            }
            process_selected_artist_tracks(app);
            default_search(app);
            app.search_menu = SearchMenu::SearchedArtist;
            app.searched_artist_selected = true;
        }

        if app.search_menu == SearchMenu::SearchedPlaylist {
            app.is_only_id = false;
            app.selected_link_for_playback =
                app.selected_playlist_tracks_links[app.searched_playlist_index].clone();
            if let Err(e) = start_playback(app) {
                println!("{}", e);
            }
        } else if app.selected_playlist_in_search_result {
            if let Err(e) = search_selected_playlist_tracks(app) {
                println!("{}", e);
            }
            process_selected_playlist_tracks(app);
            default_search(app);
            app.search_menu = SearchMenu::SearchedPlaylist;
            app.searched_playlist_selected = true;
        }
    }
}

pub fn search_tab_event(app: &mut App) {
    if app.selected_search {
        app.can_navigate_menu = false;
        app.track_state_in_search_result.select(None);
        app.artist_state_in_search_result.select(None);
        app.album_state_in_search_result.select(None);
        app.playlist_state_in_search_result.select(None);

        if app.search_state.selected() == Some(0) {
            app.track_state_in_search_result.select(Some(0));
            app.selected_track_in_search_result = !app.selected_track_in_search_result;
            app.selected_artist_in_search_result = false;
            app.selected_album_in_search_result = false;
            app.selected_playlist_in_search_result = false;
            app.is_in_track = true;
        } else if app.search_state.selected() == Some(1) {
            app.artist_state_in_search_result.select(Some(0));
            app.selected_artist_in_search_result = !app.selected_artist_in_search_result;
            app.selected_track_in_search_result = false;
            app.selected_album_in_search_result = false;
            app.selected_playlist_in_search_result = false;
            app.is_in_track = false;
        } else if app.search_state.selected() == Some(2) {
            app.album_state_in_search_result.select(Some(0));
            app.selected_album_in_search_result = !app.selected_album_in_search_result;
            app.selected_track_in_search_result = false;
            app.selected_artist_in_search_result = false;
            app.selected_playlist_in_search_result = false;
            app.is_in_track = false;
        } else if app.search_state.selected() == Some(3) {
            app.playlist_state_in_search_result.select(Some(0));
            app.selected_playlist_in_search_result = !app.selected_playlist_in_search_result;
            app.selected_track_in_search_result = false;
            app.selected_artist_in_search_result = false;
            app.selected_album_in_search_result = false;
            app.is_in_track = false;
        }
        let length = 4;
        let next_index = app.search_state.selected().unwrap_or(0) + 1;
        app.search_state.select(Some(next_index % length));
    }
}
