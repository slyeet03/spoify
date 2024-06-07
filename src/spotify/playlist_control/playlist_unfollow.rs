use crate::app::App;
use crate::enums::Menu;
use crate::spotify::auth::get_spotify_client;
use regex::Regex;
use rspotify::clients::OAuthClient;
use rspotify::model::PlaylistId;
use rspotify::ClientError;

// Main function to toggle the shuffle mode
#[tokio::main]
pub async fn unfollow_playlist(app: &mut App) -> Result<(), ClientError> {
    // Get a Spotify client using an existing access token (if available).
    let spotify = get_spotify_client(app).await?;

    let playlist_url = app.playlist_link_to_follow.as_str();
    let re_playlist = Regex::new(r"/playlist/(.+)").unwrap();
    let captures = re_playlist.captures(playlist_url).unwrap();
    let playlist_uri = captures.get(1).unwrap().as_str();
    let playlist_id = PlaylistId::from_id(playlist_uri).unwrap();

    let result = spotify.playlist_unfollow(playlist_id).await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            app.error_text = format!("Error unfollowing/deleting playlist: {}", e);
            app.selected_menu = Menu::Error;
            Err(e)
        }
    }
}
