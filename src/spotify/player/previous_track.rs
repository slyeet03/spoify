use crate::app::App;
use crate::spotify::auth::get_spotify_client;
use rspotify::clients::OAuthClient;
use rspotify::ClientError;
use crate::structs::UserCurrentlyPlaying;

// Main function to play the previous track
#[tokio::main]
pub async fn previous_track(app: &mut App, currentlyplaying: &mut UserCurrentlyPlaying) -> Result<(), ClientError> {
    // Get a Spotify client using an existing access token (if available).
    let spotify = get_spotify_client(app).await?;

    // Get the device ID from the application state (if available)
    let device_id: Option<&str> = currentlyplaying.current_device_id.as_deref();

    let result = spotify.previous_track(device_id);

    result.await?;

    Ok(())
}
