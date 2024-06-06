use crate::app::App;
use crate::enums::Menu;
use crate::spotify::auth::get_spotify_client;
use regex::Regex;
use rspotify::clients::OAuthClient;
use rspotify::model::{PlayableId, PlaylistId, TrackId};
use rspotify::ClientError;

// Main function to add a track to a playlist
#[tokio::main]
pub async fn add_track_to_playlist(app: &mut App) -> Result<(), ClientError> {
    // Get a Spotify client using an existing access token (if available).
    let spotify = get_spotify_client(app).await?;

    let playlist_url = app.playlist_link_for_track_addition.as_str();
    let re_playlist = Regex::new(r"/playlist/(.+)").unwrap();
    let captures = re_playlist.captures(playlist_url).unwrap();
    let playlist_uri = captures.get(1).unwrap().as_str();
    let playlist_id = PlaylistId::from_id(playlist_uri).unwrap();
    let mut track_id: TrackId = TrackId::from_id("").unwrap();
    let _ = track_id;

    if app.is_in_track {
        let track_url = app.track_added_to_playlist_link.as_str();
        track_id = TrackId::from_id(track_url).unwrap();
    } else {
        let track_url = app.track_added_to_playlist_link.as_str();
        let re_track = Regex::new(r"/track/(.+)").unwrap();
        let captures = re_track.captures(track_url).unwrap();
        let track_uri = captures.get(1).unwrap().as_str();
        track_id = TrackId::from_id(track_uri).unwrap();
    }

    let position = Some(0);

    let track_ids = [PlayableId::Track(track_id.clone())].into_iter();

    let result = spotify
        .playlist_add_items(playlist_id, track_ids, position)
        .await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            app.error_text = format!("Error adding track to the playlist: {}", e);
            app.selected_menu = Menu::Error;
            Err(e)
        }
    }
}
