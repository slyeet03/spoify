use crate::{
    app::App,
    enums::{Library, Menu},
};

pub fn exit_event(app: &mut App) {
    if app.selected_menu == Menu::Search {
        app.selected_menu = Menu::Default;
    } else if app.selected_menu == Menu::Library {
        if app.selected_library == Library::MadeFY {
            if app.made_fy_track_selected {
                app.made_fy_track_selected = false;
                app.made_fy_track_display = false;
                app.made_fy_selected = true;
                app.made_fy_display = true;
            } else {
                app.selected_menu = Menu::Default;
            }
        } else if app.selected_library == Library::Albums {
            if app.user_album_track_selected {
                app.user_album_track_selected = false;
                app.user_album_track_display = false;
                app.user_album_current_album_selected = true;
                app.user_album_display = true;
                app.user_album_selected = true;
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
