use crate::app::App;
use crate::spotify::auth::get_spotify_client;
use rspotify::clients::OAuthClient;
use rspotify::{AuthCodeSpotify, ClientError};
use std::ops::Deref;

#[tokio::main]
pub async fn toogle_shuffle(app: &mut App) -> Result<(), ClientError> {
    let spotify_client = get_spotify_client().await.unwrap();

    let spotify = match &spotify_client.token {
        Some(token) => AuthCodeSpotify::from_token(token.clone()),
        None => return Err(ClientError::InvalidToken),
    };

    let device_id: Option<&str> = app.current_device_id.as_ref().map(Deref::deref);

    let result = spotify.shuffle(app.is_shuffle, device_id);

    if app.is_shuffle {
        app.shuffle_status = "On".to_string();
    } else if !app.is_shuffle {
        app.shuffle_status = "Off".to_string();
    }

    result.await?;

    Ok(())
}
