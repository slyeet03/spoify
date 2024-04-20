extern crate rspotify;
extern crate serde_json;

use crate::app::App;
use dotenvy::dotenv;
use futures::{FutureExt, TryStreamExt};
use rspotify::{model::SimplifiedAlbum, prelude::*, ClientCredsSpotify, ClientError, Credentials};
use serde_json::{json, Value};
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;

/// Fetches a list of new releases from Spotify and stores them for later use
#[tokio::main]
pub async fn new_releases() -> Result<(), ClientError> {
    dotenv().expect(".env file not found");

    let client_id = env::var("CLIENT_ID").expect("You've not set the CLIENT_ID");
    let client_secret_id =
        env::var("CLIENT_SECRET_ID").expect("You've not set the CLIENT_SECRET_ID");

    // Create authentication credentials
    let creds = Credentials {
        id: client_id.to_string(),
        secret: Some(client_secret_id.to_string()),
    };

    // Create a Spotify client using client credentials flow.
    let spotify = ClientCredsSpotify::new(creds);

    // Request an access token from Spotify.
    spotify.request_token().await.unwrap();

    // Collect information about a limited number of new releases.
    let mut new_releases = Vec::new();
    let stream = spotify
        .new_releases(None)
        .try_for_each(|item| {
            new_releases.push(item);
            futures::future::ok(())
        })
        .boxed();

    stream.await?;

    save_new_releases_to_json(new_releases);

    Ok(())
}

/// Saves a vector of simplified album data to a JSON file in the Spotify cache directory
fn save_new_releases_to_json(items: Vec<SimplifiedAlbum>) {
    let json_data = json!(items);

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("spotify_cache");
    std::fs::create_dir_all(&path).unwrap();
    path.push("new_releases.json");

    let mut file = File::create(&path).unwrap();
    let _ = file.write_all(json_data.to_string().as_bytes());
}

/// Processes the new releases data stored in the cache file and populates the app's data structures
pub fn process_new_releases(app: &mut App) {
    app.new_release_artist.clear();
    app.new_release_name.clear();

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("spotify_cache");
    path.push("new_releases.json");

    let file = File::open(&path).expect("Failed to open new_releases.json");
    let reader = BufReader::new(file);
    let json_data: Value =
        serde_json::from_reader(reader).expect("Failed to parse new_releases.json");

    // Extract information about each new release from the JSON data
    if let Value::Array(albums) = json_data {
        for album in albums {
            if let Value::Object(album_obj) = album {
                if let Some(artists) = album_obj.get("artists") {
                    if let Value::Array(artist_array) = artists {
                        if let Some(first_artist) = artist_array.get(0) {
                            if let Value::Object(artist_obj) = first_artist {
                                if let Some(artist_name) = artist_obj.get("name") {
                                    if let Value::String(name) = artist_name {
                                        app.new_release_artist.push(name.clone());
                                    }
                                }
                            }
                        }
                    }
                }
                if let Some(id) = album_obj.get("id").and_then(Value::as_str) {
                    app.new_release_album_links.push(id.to_string());
                }
                if let Some(name) = album_obj.get("name") {
                    if let Value::String(album_name) = name {
                        app.new_release_name.push(album_name.clone());
                    }
                }
            }
        }
    }
}
