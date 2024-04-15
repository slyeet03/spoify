/*
fetches the user playlist list
saves their data in a json file
saves their name and links in variables
*/

use crate::app::App;
use crate::spotify::auth::{get_spotify_client, SpotifyClient};
use dotenv::dotenv;
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
        println!("No playlists found. Check if the authorization code is valid and has the required scopes.");
    } else {
        println!("Fetched playlists");
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
    println!("Playlists saved to {}", path.display());
}

#[tokio::main]
pub async fn get_playlists() {
    let spotify_client = get_spotify_client().await.unwrap();
    match fetch_user_playlists(&spotify_client).await {
        Ok(playlists) => {
            println!("Playlist caching successful");
            save_playlists_to_json(&playlists);
        }
        Err(e) => println!("Error fetching playlists: {}", e),
    }
}

pub fn process_user_playlists(app: &mut App) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("spotify_cache");
    path.push("playlists.json");

    let file = File::open(&path).expect("Failed to open playlists.json");
    let reader = BufReader::new(file);

    let playlists: Value = serde_json::from_reader(reader).expect("Failed to parse playlists.json");

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
