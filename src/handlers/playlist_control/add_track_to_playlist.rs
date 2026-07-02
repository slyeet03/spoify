use crate::structs::NewRelease;
use crate::structs::UserSavedArtist;
use crate::structs::UserRecentlyPlayed;
use crate::MadeFY;
use crate::UserSavedAlbums;
use crate::LikedSongs;
use crate::UserPlaylist;
use crate::app::App;
use crate::enums::{Library, Menu, SearchMenu};
use crate::handlers::util::{down_key_for_list, up_key_for_list};
use crate::spotify::playlist_control::add_track_to_playlist::add_track_to_playlist;
use crate::structs::Search;

pub fn add_track_to_playlist_event(app: &mut App, search: &mut Search, userplaylist: &mut UserPlaylist, likedsongs: &mut LikedSongs, useralbum: &mut UserSavedAlbums, madefy: &mut MadeFY, recentlyplayed: &mut UserRecentlyPlayed, userartist: &mut UserSavedArtist, newrelease: &mut NewRelease) {
    if app.selected_menu == Menu::Library {
        if app.selected_library == Library::RecentlyPlayed {
            app.track_added_to_playlist_link =
                recentlyplayed.recently_played_links[recentlyplayed.recently_played_index].clone();
            app.selected_menu = Menu::AddTrackToPlaylist;
        } else if app.selected_library == Library::LikedSongs {
            app.track_added_to_playlist_link = likedsongs.links[likedsongs.index].clone();
            app.selected_menu = Menu::AddTrackToPlaylist;
        } else if app.selected_library == Library::MadeFY {
            if madefy.made_fy_track_selected {
                app.track_added_to_playlist_link =
                    madefy.made_fy_track_links[madefy.made_fy_track_index].clone();
                app.selected_menu = Menu::AddTrackToPlaylist;
            }
        } else if app.selected_library == Library::Albums {
            if useralbum.user_album_track_selected {
                app.track_added_to_playlist_link =
                    useralbum.user_album_track_links[useralbum.user_album_track_index].clone();
                app.selected_menu = Menu::AddTrackToPlaylist;
            }
        } else if app.selected_library == Library::Artists && userartist.user_artist_track_selected {
            app.track_added_to_playlist_link =
                userartist.user_artist_track_links[userartist.user_artist_track_index].clone();
            app.selected_menu = Menu::AddTrackToPlaylist;
        }
    } else if app.selected_menu == Menu::NewRelease {
        if newrelease.enter_for_playback_in_new_release {
            app.track_added_to_playlist_link =
                newrelease.new_release_spotify_urls[newrelease.new_release_index].clone();
            app.selected_menu = Menu::AddTrackToPlaylist;
        }
    } else if app.selected_menu == Menu::Playlists {
        if userplaylist.enter_for_playback_in_user_playlist {
            app.track_added_to_playlist_link =
                userplaylist.track_links[userplaylist.index].clone();
            app.selected_menu = Menu::AddTrackToPlaylist;
        }
    } else if app.selected_menu == Menu::Search {
        if app.is_in_track {
            app.track_added_to_playlist_link =
                search.track_links[search.track_index].clone();
            app.selected_menu = Menu::AddTrackToPlaylist;
        } else if search.menu == SearchMenu::SearchedAlbum {
            app.track_added_to_playlist_link =
                search.selected_album_tracks_links[search.searched_album_index].clone();
            app.selected_menu = Menu::AddTrackToPlaylist;
        } else if search.menu == SearchMenu::SearchedArtist {
            app.track_added_to_playlist_link =
                search.selected_artist_tracks_links[search.searched_artist_index].clone();
            app.selected_menu = Menu::AddTrackToPlaylist;
        } else if search.menu == SearchMenu::SearchedPlaylist {
            app.track_added_to_playlist_link =
                search.selected_playlist_tracks_links[search.searched_playlist_index].clone();
            app.selected_menu = Menu::AddTrackToPlaylist;
        }
    }
}

pub fn add_track_to_playlist_enter_event(app: &mut App, userplaylist: &mut UserPlaylist) {
    if app.selected_menu == Menu::AddTrackToPlaylist {
        app.playlist_link_for_track_addition =
            userplaylist.links[app.playlist_index_for_track_addition].clone();
        if let Err(e) = add_track_to_playlist(app) {
            println!("{}", e);
        }
        app.selected_menu = Menu::Default;
    }
}

pub fn add_track_to_playlist_up_event(app: &mut App, userplaylist: &mut UserPlaylist) {
    if app.selected_menu == Menu::AddTrackToPlaylist {
        (
            app.add_track_to_playlist_state,
            app.playlist_index_for_track_addition,
        ) = up_key_for_list(
            userplaylist.names.clone(),
            app.add_track_to_playlist_state.clone(),
        );
    }
}

pub fn add_track_to_playlist_down_event(app: &mut App, userplaylist: &mut UserPlaylist) {
    if app.selected_menu == Menu::AddTrackToPlaylist {
        (
            app.add_track_to_playlist_state,
            app.playlist_index_for_track_addition,
        ) = down_key_for_list(
            userplaylist.names.clone(),
            app.add_track_to_playlist_state.clone(),
        );
    }
}
