use crate::app::App;
use crate::spotify::auth::get_spotify_client;
use chrono::Duration;
use regex::Regex;
use rspotify::clients::OAuthClient;
use rspotify::model::{PlayableId, TrackId};
use rspotify::ClientError;
use std::ops::Deref;

// Main function to toggle the shuffle mode
#[tokio::main]
pub async fn start_playback(app: &mut App) -> Result<(), ClientError> {
    // Get a Spotify client using an existing access token (if available).
    let spotify = get_spotify_client(app).await?;

    // Get the device ID from the application state (if available)
    let device_id: Option<&str> = app.current_device_id.as_ref().map(Deref::deref);

    let track_url = app.selected_link_for_playback.as_str();
    let re = Regex::new(r"/track/(.+)").unwrap();
    let captures = re.captures(track_url).unwrap();
    let track_id = captures.get(1).unwrap().as_str();
    let track_uri_in_string = format!("spotify:track:{}", track_id);
    let track_uri = track_uri_in_string.as_str();
    println!("track_uri: {}", track_uri);
    let playable_id = PlayableId::Track(TrackId::from_uri(track_uri).unwrap());

    let position = Duration::milliseconds(0);

    let result = spotify.start_uris_playback(vec![playable_id], device_id, None, Some(position));

    result.await?;

    Ok(())
}
