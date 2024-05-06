use crate::app::App;
use crate::spotify::auth::get_spotify_client;
use rspotify::clients::OAuthClient;
use rspotify::{AuthCodeSpotify, ClientError};
use std::ops::Deref;

// Main function to toggle the shuffle mode
#[tokio::main]
pub async fn toogle_shuffle(app: &mut App) -> Result<(), ClientError> {
    let spotify_client = get_spotify_client(app).await.unwrap();

    // Get the Spotify client instance based on the token
    let spotify = match &spotify_client.token {
        Some(token) => AuthCodeSpotify::from_token(token.clone()),
        None => return Err(ClientError::InvalidToken),
    };

    // Get the device ID from the application state (if available)
    let device_id: Option<&str> = app.current_device_id.as_ref().map(Deref::deref);

    // Toggle the shuffle mode for the specified device
    let result = spotify.shuffle(app.is_shuffle, device_id);

    result.await?;

    Ok(())
}
