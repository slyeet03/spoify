use crate::app::App;
use crate::spotify::auth::{get_spotify_client, SpotifyClient};
use futures::pin_mut;
use futures_util::TryStreamExt;
use rspotify::model::{CursorBasedPage, PlayHistory};
use rspotify::prelude::OAuthClient;
use rspotify::{AuthCodeSpotify, ClientError};

pub async fn liked_tracks() -> Result<(), ClientError> {
    let spotify_client = get_spotify_client().await.unwrap();

    let spotify = match &spotify_client.token {
        Some(token) => AuthCodeSpotify::from_token(token.clone()),
        None => return Err(ClientError::InvalidToken),
    };

    // Executing the futures sequentially
    let stream = spotify.current_user_saved_tracks(None);
    pin_mut!(stream);
    println!("Items (blocking):");
    while let Some(item) = stream.try_next().await.unwrap() {
        println!("* {}", item.track.name);
    }

    Ok(())
}
