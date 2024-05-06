use crate::app::App;
use crate::spotify::auth::get_spotify_client;
use rspotify::clients::OAuthClient;
use rspotify::model::RepeatState;
use rspotify::{AuthCodeSpotify, ClientError};
use std::ops::Deref;

// Main function to toggle the shuffle mode
#[tokio::main]
pub async fn cycle_repeat(app: &mut App) -> Result<(), ClientError> {
    let spotify_client = get_spotify_client(app).await.unwrap();

    // Get the Spotify client instance based on the token
    let spotify = match &spotify_client.token {
        Some(token) => AuthCodeSpotify::from_token(token.clone()),
        None => return Err(ClientError::InvalidToken),
    };

    // Get the device ID from the application state (if available)
    let device_id: Option<&str> = app.current_device_id.as_ref().map(Deref::deref);

    // Cycle through repeat states for the specified device
    let mut state: RepeatState = RepeatState::Off;

    if app.repeat_status == "Off" {
        state = RepeatState::Context;
    } else if app.repeat_status == "Album/Playlist" {
        state = RepeatState::Track;
    } else if app.repeat_status == "Track" {
        state = RepeatState::Off;
    }

    let result = spotify.repeat(state, device_id);

    result.await?;

    Ok(())
}
