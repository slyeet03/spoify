use crate::app::App;
use crate::enums::Menu;
use crate::spotify::auth::get_spotify_client;
use rspotify::clients::OAuthClient;
use rspotify::ClientError;
use std::ops::Deref;

// Main function to toggle the shuffle mode
#[tokio::main]
pub async fn volume_decreament(app: &mut App) -> Result<(), ClientError> {
    // Get a Spotify client using an existing access token (if available).
    let spotify = get_spotify_client(app).await?;

    // Get the device ID from the application state (if available)
    let device_id: Option<&str> = app.current_device_id.as_ref().map(Deref::deref);

    // Decreament the current device volume by the configured volume decreament value
    if app.volume_percent != 0 {
        app.volume_percent -= app.volume_decreament_value;

        // Set the new volume on the current device
        let result = spotify.volume(app.volume_percent, device_id);

        result.await?;
    } else {
        app.error_text = format!("Volume is already at 0%");
        app.selected_menu = Menu::Error;
    }

    Ok(())
}
