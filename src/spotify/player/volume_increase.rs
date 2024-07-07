use crate::app::App;
use crate::enums::Menu;
use crate::spotify::auth::get_spotify_client;
use crate::structs::Settings;
use rspotify::clients::OAuthClient;
use rspotify::ClientError;

// Main function to toggle the shuffle mode
#[tokio::main]
pub async fn volume_increment(app: &mut App, settings: &mut Settings) -> Result<(), ClientError> {
    // Get a Spotify client using an existing access token (if available).
    let spotify = get_spotify_client(app).await?;

    // Get the device ID from the application state (if available)
    let device_id: Option<&str> = app.current_device_id.as_deref();

    // Increment the current device volume by the configured volume increment value
    if settings.volume_percent != 100 {
        settings.volume_percent += settings.volume_increment_value;

        // Set the new volume on the current device
        let result = spotify.volume(settings.volume_percent, device_id);

        result.await?;
    } else {
        app.error_text = "Volume is already at 100%".to_string();
        app.selected_menu = Menu::Error;
    }

    Ok(())
}
