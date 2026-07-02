use crate::app::App;
use crate::spotify::auth::get_spotify_client;
use rspotify::clients::OAuthClient;
use rspotify::model::RepeatState;
use rspotify::ClientError;
use crate::structs::UserCurrentlyPlaying;

// Main function to toggle the shuffle mode
#[tokio::main]
pub async fn cycle_repeat(app: &mut App,currentlyplaying: &mut UserCurrentlyPlaying) -> Result<(), ClientError> {
    // Get a Spotify client using an existing access token (if available).
    let spotify = get_spotify_client(app).await?;

    // Get the device ID from the application state (if available)
    let device_id: Option<&str> = currentlyplaying.current_device_id.as_deref();

    // Cycle through repeat states for the specified device
    let mut state: RepeatState = RepeatState::Off;

    if currentlyplaying.repeat_status == "Off" {
        state = RepeatState::Context;
    } else if currentlyplaying.repeat_status == "Album/Playlist" {
        state = RepeatState::Track;
    } else if currentlyplaying.repeat_status == "Track" {
        state = RepeatState::Off;
    }

    let result = spotify.repeat(state, device_id);

    result.await?;

    Ok(())
}
