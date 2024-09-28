// This section handles fetching and processing the user's Spotify playlists
use crate::app::App;
use crate::spotify::auth::get_spotify_client;
use crate::util::get_project_dir;
use futures_util::TryStreamExt;
use rspotify::clients::OAuthClient;
use rspotify::model::SimplifiedPlaylist;
use rspotify::{AuthCodeSpotify, ClientError};
use serde_json;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;

/// Fetches user playlists from Spotify
pub async fn fetch_user_playlists(
    app: &mut App,
    spotify: &AuthCodeSpotify,
) -> Result<Vec<SimplifiedPlaylist>, ClientError> {
    // Collect information about the user playlists
    let mut playlists = Vec::new();
    let mut stream = spotify.current_user_playlists();
    while let Ok(Some(playlist)) = stream.try_next().await {
        playlists.push(playlist);
        app.have_playlist = true;
    }
    if playlists.is_empty() {
        println!("No playlists found.");
        app.have_playlist = false;
    }
    Ok(playlists)
}

/// Saves a list of playlists in JSON format to a file for later use
fn save_playlists_to_json(app: &mut App, playlists: &[SimplifiedPlaylist]) {
    let json_data = serde_json::to_vec_pretty(playlists).unwrap();

    let project_dir = get_project_dir(&app.file_name);
    let mut path = project_dir.join("spotify_cache");
    std::fs::create_dir_all(&path).unwrap();
    path = path.join("playlists.json");

    let mut file = File::create(&path).unwrap();
    file.write_all(&json_data).unwrap();
}

#[tokio::main]
pub async fn get_playlists(app: &mut App) {
    // Obtain a Spotify client using the access token (if available)
    let spotify = get_spotify_client(app).await.unwrap();
    match fetch_user_playlists(app, &spotify).await {
        Ok(playlists) => {
            save_playlists_to_json(app, &playlists);
        }
        Err(e) => println!("Error fetching playlists: {}", e),
    }
}

/// Processes the playlist data stored in the cache file and populates the app's data structures
pub fn process_user_playlists(app: &mut App) {
    app.user_playlist_names.clear();
    app.user_playlist_links.clear();

    let project_dir = get_project_dir(&app.file_name);
    let mut path = project_dir.join("spotify_cache");
    path = path.join("playlists.json");

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
