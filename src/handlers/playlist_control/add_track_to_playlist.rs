use crate::app::App;
use crate::enums::{Library, Menu, SearchMenu};
use crate::handlers::util::{down_key_for_list, up_key_for_list};
use crate::spotify::playlist_control::add_track_to_playlist::add_track_to_playlist;

pub fn add_track_to_playlist_event(app: &mut App) {
    if app.selected_menu == Menu::Library {
        if app.selected_library == Library::RecentlyPlayed {
            app.track_added_to_playlist_link =
                app.recently_played_links[app.recently_played_index].clone();
            app.selected_menu = Menu::AddTrackToPlaylist;
        } else if app.selected_library == Library::LikedSongs {
            app.track_added_to_playlist_link = app.liked_song_links[app.liked_songs_index].clone();
            app.selected_menu = Menu::AddTrackToPlaylist;
        } else if app.selected_library == Library::MadeFY {
            if app.made_fy_track_selected {
                app.track_added_to_playlist_link =
                    app.made_fy_track_links[app.made_fy_track_index].clone();
                app.selected_menu = Menu::AddTrackToPlaylist;
            }
        } else if app.selected_library == Library::Albums {
            if app.user_album_track_selected {
                app.track_added_to_playlist_link =
                    app.user_album_track_links[app.user_album_track_index].clone();
                app.selected_menu = Menu::AddTrackToPlaylist;
            }
        } else if app.selected_library == Library::Artists && app.user_artist_track_selected {
            app.track_added_to_playlist_link =
                app.user_artist_track_links[app.user_artist_track_index].clone();
            app.selected_menu = Menu::AddTrackToPlaylist;
        }
    } else if app.selected_menu == Menu::NewRelease {
        if app.enter_for_playback_in_new_release {
            app.track_added_to_playlist_link =
                app.new_release_spotify_urls[app.new_release_index].clone();
            app.selected_menu = Menu::AddTrackToPlaylist;
        }
    } else if app.selected_menu == Menu::Playlists {
        if app.enter_for_playback_in_user_playlist {
            app.track_added_to_playlist_link =
                app.user_playlist_track_links[app.user_playlist_index].clone();
            app.selected_menu = Menu::AddTrackToPlaylist;
        }
    } else if app.selected_menu == Menu::Search {
        if app.is_in_track {
            app.track_added_to_playlist_link =
                app.track_links_search_results[app.track_index].clone();
            app.selected_menu = Menu::AddTrackToPlaylist;
        } else if app.search_menu == SearchMenu::SearchedAlbum {
            app.track_added_to_playlist_link =
                app.selected_album_tracks_links[app.searched_album_index].clone();
            app.selected_menu = Menu::AddTrackToPlaylist;
        } else if app.search_menu == SearchMenu::SearchedArtist {
            app.track_added_to_playlist_link =
                app.selected_artist_tracks_links[app.searched_artist_index].clone();
            app.selected_menu = Menu::AddTrackToPlaylist;
        } else if app.search_menu == SearchMenu::SearchedPlaylist {
            app.track_added_to_playlist_link =
                app.selected_playlist_tracks_links[app.searched_playlist_index].clone();
            app.selected_menu = Menu::AddTrackToPlaylist;
        }
    }
}

pub fn add_track_to_playlist_enter_event(app: &mut App) {
    if app.selected_menu == Menu::AddTrackToPlaylist {
        app.playlist_link_for_track_addition =
            app.user_playlist_links[app.playlist_index_for_track_addition].clone();
        if let Err(e) = add_track_to_playlist(app) {
            println!("{}", e);
        }
        app.selected_menu = Menu::Default;
    }
}

pub fn add_track_to_playlist_up_event(app: &mut App) {
    if app.selected_menu == Menu::AddTrackToPlaylist {
        (
            app.add_track_to_playlist_state,
            app.playlist_index_for_track_addition,
        ) = up_key_for_list(
            app.user_playlist_names.clone(),
            app.add_track_to_playlist_state.clone(),
        );
    }
}

pub fn add_track_to_playlist_down_event(app: &mut App) {
    if app.selected_menu == Menu::AddTrackToPlaylist {
        (
            app.add_track_to_playlist_state,
            app.playlist_index_for_track_addition,
        ) = down_key_for_list(
            app.user_playlist_names.clone(),
            app.add_track_to_playlist_state.clone(),
        );
    }
}
