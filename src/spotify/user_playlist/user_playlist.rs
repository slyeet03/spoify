// This section handles fetching and processing the user's Spotify playlists
use crate::app::App;
use crate::spotify::auth::get_spotify_client;
use futures_util::TryStreamExt;
use rspotify::clients::OAuthClient;
use rspotify::model::SimplifiedPlaylist;
use rspotify::{AuthCodeSpotify, ClientError};
use serde_json;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;

/// Fetches user playlists from Spotify
pub async fn fetch_user_playlists(
    spotify: &AuthCodeSpotify,
) -> Result<Vec<SimplifiedPlaylist>, ClientError> {
    // Collect information about the user playlists
    let mut playlists = Vec::new();
    let mut stream = spotify.current_user_playlists();
    while let Ok(Some(playlist)) = stream.try_next().await {
        playlists.push(playlist);
    }
    if playlists.is_empty() {
        println!("No playlists found. Check if the authorization code is valid and has the required scopes.");
    }
    Ok(playlists)
}

/// Saves a list of playlists in JSON format to a file for later use
fn save_playlists_to_json(playlists: &[SimplifiedPlaylist]) {
    let json_data = serde_json::to_vec_pretty(playlists).unwrap();

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify");
    path.push("spotify_cache");
    std::fs::create_dir_all(&path).unwrap();
    path.push("playlists.json");

    let mut file = File::create(&path).unwrap();
    file.write_all(&json_data).unwrap();
}

#[tokio::main]
pub async fn get_playlists(app: &mut App) {
    // Obtain a Spotify client using the access token (if available)
    let spotify = get_spotify_client(app).await.unwrap();
    match fetch_user_playlists(&spotify).await {
        Ok(playlists) => {
            save_playlists_to_json(&playlists);
        }
        Err(e) => println!("Error fetching playlists: {}", e),
    }
}

/// Processes the playlist data stored in the cache file and populates the app's data structures
pub fn process_user_playlists(app: &mut App) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify");
    path.push("spotify_cache");
    path.push("playlists.json");

    let file = File::open(&path).expect("Failed to open playlists.json");
    let reader = BufReader::new(file);

    let playlists: Value = serde_json::from_reader(reader).expect("Failed to parse playlists.json");

    // Extract information about each playlist from the JSON data and populate the app's data structures for displaying the playlists
    if let Value::Array(playlists) = playlists {
        for playlist in playlists {
            if let Value::Object(playlist_obj) = playlist {
                if let Some(name) = playlist_obj.get("name").and_then(Value::as_str) {
                    app.user_playlist_names.push(name.to_string());
                }

                if let Some(link) = playlist_obj
                    .get("external_urls")
                    .and_then(Value::as_object)
                    .and_then(|urls| urls.get("spotify"))
                    .and_then(Value::as_str)
                {
                    app.user_playlist_links.push(link.to_string());
                }
            }
        }
    }
}
