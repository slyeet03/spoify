use crate::UserPlaylist;
use crate::{
    app::App,
    enums::Menu,
    spotify::{
        playlist_control::playlist_unfollow::unfollow_playlist,
        user_playlist::user_playlist::{get_playlists, process_user_playlists},
    },
};

pub fn unfollow_playlist_event(app: &mut App, userplaylist: &mut UserPlaylist) {
    app.playlist_link_to_follow.clear();
    if app.selected_menu == Menu::Playlists && !userplaylist.enter_for_playback_in_user_playlist {
        app.playlist_link_to_follow = userplaylist.links[userplaylist.index].clone();
        if let Err(e) = unfollow_playlist(app) {
            println!("{}", e);
        }
        // Fetch user playlists from spotify
        get_playlists(app);
        process_user_playlists(app,userplaylist);
    }
}
