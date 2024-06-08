use crate::app::App;
use crate::spotify::auth::get_spotify_client;
use rspotify::clients::OAuthClient;
use rspotify::ClientError;

// Main function to toggle the shuffle mode
#[tokio::main]
pub async fn toogle_shuffle(app: &mut App) -> Result<(), ClientError> {
    // Get a Spotify client using an existing access token (if available).
    let spotify = get_spotify_client(app).await?;

    // Get the device ID from the application state (if available)
    let device_id: Option<&str> = app.current_device_id.as_deref();

    // Toggle the shuffle mode for the specified device
    let result = spotify.shuffle(app.is_shuffle, device_id);

    result.await?;

    Ok(())
}
