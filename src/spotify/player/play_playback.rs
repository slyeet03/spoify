use crate::app::App;
use crate::spotify::auth::get_spotify_client;
use chrono::TimeDelta;
use rspotify::clients::OAuthClient;
use rspotify::{AuthCodeSpotify, ClientError};
use std::ops::Deref;

use super::util::f64_to_duration;

// Main function to toggle the shuffle mode
#[tokio::main]
pub async fn play(app: &mut App) -> Result<(), ClientError> {
    let spotify_client = get_spotify_client().await.unwrap();

    // Get the Spotify client instance based on the token
    let spotify = match &spotify_client.token {
        Some(token) => AuthCodeSpotify::from_token(token.clone()),
        None => return Err(ClientError::InvalidToken),
    };

    // Get the device ID from the application state (if available)
    let device_id: Option<&str> = app.current_device_id.as_ref().map(Deref::deref);

    let duration_result: Result<TimeDelta, chrono::OutOfRangeError> =
        f64_to_duration(app.currrent_timestamp);

    match duration_result {
        Ok(duration) => {
            let _result = spotify.resume_playback(device_id, Some(duration));
            Ok(())
        }
        Err(err) => {
            eprintln!("{}", err);
            Err(ClientError::InvalidToken)
        }
    }
}
