use crate::UserSavedArtist;
use crate::MadeFY;
use crate::UserSavedAlbums;
use crate::{
    app::App,
    enums::{Library, Menu},
};

pub fn exit_event(app: &mut App, useralbum: &mut UserSavedAlbums, madefy: &mut MadeFY, userartist: &mut UserSavedArtist) {
    if app.selected_menu == Menu::Search {
        app.selected_menu = Menu::Default;
    } else if app.selected_menu == Menu::Library {
        if app.selected_library == Library::MadeFY {
            if madefy.track_selected {
                madefy.track_selected = false;
                madefy.track_display = false;
                madefy.selected = true;
                madefy.display = true;
            } else {
                app.selected_menu = Menu::Default;
            }
        } else if app.selected_library == Library::Albums {
            if useralbum.track_selected {
                useralbum.track_selected = false;
                useralbum.track_display = false;
                useralbum.current_album_selected = true;
                useralbum.display = true;
                useralbum.selected = true;
            } else {
                app.selected_menu = Menu::Default;
            }
        } else if app.selected_library == Library::Artists {
            if userartist.track_selected {
                userartist.track_selected = false;
                userartist.track_display = false;
                userartist.current_artist_selected = true;
                userartist.display = true;
                userartist.selected = true;
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
