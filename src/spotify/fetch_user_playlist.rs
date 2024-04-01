use crate::spotify::auth::{get_spotify_client, SpotifyClient};
use futures_util::StreamExt;
use futures_util::TryStreamExt;
use rspotify::{clients::OAuthClient, model::SimplifiedPlaylist, ClientError};

async fn fetch_user_playlists() -> Result<(), ClientError> {
    let spotify_client = get_spotify_client().await?;

    //let mut playlists = Vec::new();

    let mut stream = spotify_client.spotify.current_user_playlists();

    while let Ok(Some(playlist)) = stream.try_next().await {
        println!("Playlist: {}", playlist.name);
    }

    Ok(())
}

#[tokio::main]
pub async fn spo() {
    match fetch_user_playlists().await {
        Ok(()) => println!("Playlist fetching successful"),
        Err(e) => eprintln!("Error fetching playlists: {}", e),
    }
}
