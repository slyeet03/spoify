use crate::MadeFY;
use crate::UserSavedAlbums;
use crate::{
    app::App,
    enums::{Library, Menu},
};

pub fn exit_event(app: &mut App, useralbum: &mut UserSavedAlbums, madefy: &mut MadeFY) {
    if app.selected_menu == Menu::Search {
        app.selected_menu = Menu::Default;
    } else if app.selected_menu == Menu::Library {
        if app.selected_library == Library::MadeFY {
            if madefy.made_fy_track_selected {
                madefy.made_fy_track_selected = false;
                madefy.made_fy_track_display = false;
                madefy.made_fy_selected = true;
                madefy.made_fy_display = true;
            } else {
                app.selected_menu = Menu::Default;
            }
        } else if app.selected_library == Library::Albums {
            if useralbum.user_album_track_selected {
                useralbum.user_album_track_selected = false;
                useralbum.user_album_track_display = false;
                useralbum.user_album_current_album_selected = true;
                useralbum.user_album_display = true;
                useralbum.user_album_selected = true;
            } else {
                app.selected_menu = Menu::Default;
            }
        } else if app.selected_library == Library::Artists {
            if app.user_artist_track_selected {
                app.user_artist_track_selected = false;
                app.user_artist_track_display = false;
                app.user_artist_current_artist_selected = true;
                app.user_artist_display = true;
                app.user_artist_selected = true;
            } else {
                app.selected_menu = Menu::Default;
            }
        } else {
            app.selected_menu = Menu::Default;
        }
    } else if app.selected_menu == Menu::Playlists
        || app.selected_menu == Menu::NewRelease
        || app.selected_menu == Menu::AddTrackToPlaylist
    {
        app.selected_menu = Menu::Default;
    } else {
        app.exit();
    }
}
