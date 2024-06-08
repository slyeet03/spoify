use crate::{
    app::App,
    enums::Menu,
    spotify::{
        playlist_control::playlist_follow::follow_playlist,
        user_playlist::user_playlist::{get_playlists, process_user_playlists},
    },
};

pub fn follow_playlist_event(app: &mut App) {
    app.playlist_link_to_follow.clear();
    if app.selected_menu == Menu::Search && app.selected_playlist_in_search_result {
        app.playlist_link_to_follow = app.playlist_links_search_results[app.playlist_index].clone();
        if let Err(e) = follow_playlist(app) {
            println!("{}", e);
        }
        // Fetch user playlists from spotify
        get_playlists(app);
        process_user_playlists(app);
    }
}
