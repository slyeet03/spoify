use crate::init_logger;
use crate::spotify::auth::{get_spotify_client, SpotifyClient};
use dotenv::dotenv;
use futures_util::StreamExt;
use futures_util::TryStreamExt;
use log::info;
use rspotify::clients::OAuthClient;
use rspotify::model::SimplifiedPlaylist;
use rspotify::{AuthCodeSpotify, ClientError};
use serde_json;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub async fn fetch_user_playlists(
    spotify_client: &SpotifyClient,
) -> Result<Vec<SimplifiedPlaylist>, ClientError> {
    dotenv().ok();
    let spotify = match &spotify_client.token {
        Some(token) => AuthCodeSpotify::from_token(token.clone()),
        None => return Err(ClientError::InvalidToken),
    };

    let mut playlists = Vec::new();
    let mut stream = spotify.current_user_playlists();

    while let Ok(Some(playlist)) = stream.try_next().await {
        playlists.push(playlist);
    }

    if playlists.is_empty() {
        info!("No playlists found. Check if the authorization code is valid and has the required scopes.");
    } else {
        info!("Fetched playlists");
    }

    Ok(playlists)
}

fn save_playlists_to_json(playlists: &[SimplifiedPlaylist]) {
    let json_data = serde_json::to_vec_pretty(playlists).unwrap();

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("spotify_cache");
    std::fs::create_dir_all(&path).unwrap();
    path.push("playlists.json");

    let mut file = File::create(&path).unwrap();
    file.write_all(&json_data).unwrap();
    info!("Playlists saved to {}", path.display());
}

#[tokio::main]
pub async fn get_playlists() {
    init_logger().unwrap();
    let spotify_client = get_spotify_client().await.unwrap();
    match fetch_user_playlists(&spotify_client).await {
        Ok(playlists) => {
            info!("Playlist caching successful");
            save_playlists_to_json(&playlists);
        }
        Err(e) => info!("Error fetching playlists: {}", e),
    }
}
