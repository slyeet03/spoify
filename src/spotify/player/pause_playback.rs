use crate::app::App;
use crate::spotify::auth::get_spotify_client;
use rspotify::clients::OAuthClient;
use rspotify::ClientError;
use std::ops::Deref;

// Main function to toggle the shuffle mode
#[tokio::main]
pub async fn pause(app: &mut App) -> Result<(), ClientError> {
    // Get a Spotify client using an existing access token (if available).
    let spotify = get_spotify_client(app).await?;

    // Get the device ID from the application state (if available)
    let device_id: Option<&str> = app.current_device_id.as_ref().map(Deref::deref);

    let result = spotify.pause_playback(device_id);

    result.await?;

    Ok(())
}
