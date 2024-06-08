use crate::app::App;
use crate::spotify::auth::get_spotify_client;
use rspotify::clients::OAuthClient;
use rspotify::ClientError;

// Main function to play the next track
#[tokio::main]
pub async fn next_track(app: &mut App) -> Result<(), ClientError> {
    // Get a Spotify client using an existing access token (if available).
    let spotify = get_spotify_client(app).await?;

    // Get the device ID from the application state (if available)
    let device_id: Option<&str> = app.current_device_id.as_deref();

    let result = spotify.next_track(device_id);

    result.await?;

    Ok(())
}
