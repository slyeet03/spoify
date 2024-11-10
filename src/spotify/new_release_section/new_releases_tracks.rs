extern crate rspotify;
extern crate serde_json;

use crate::app::App;
use futures::{FutureExt, TryStreamExt};
use rspotify::model::{AlbumId, SimplifiedTrack};
use rspotify::{prelude::*, ClientCredsSpotify, ClientError, Credentials};
use serde_json::{json, Value};
use std::env;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;

/// Fetches the tracks from a new release album and stores them for later use
#[tokio::main]
pub async fn new_releases_tracks(app: &mut App) -> Result<(), ClientError> {
    let client_id = &app.client_id;
    let client_secret_id = &app.client_secret;

    // Create authentication credentials
    let creds = Credentials {
        id: client_id.to_string(),
        secret: Some(client_secret_id.to_string()),
    };

    // Create a Spotify client using client credentials flow
    let spotify = ClientCredsSpotify::new(creds);

    // Request an access token from Spotify
    spotify.request_token().await.unwrap();

    // Collect tracks from the new release album
    let mut new_releases_tracks = Vec::new();
    let album_id: AlbumId = AlbumId::from_id(app.current_new_release_album_link.clone()).unwrap();

    // Stream the album tracks and collect them into a vector.
    let stream = spotify
        .album_track(album_id, None)
        .try_for_each(|item| {
            new_releases_tracks.push(item);
            futures::future::ok(())
        })
        .boxed();

    stream.await?;

    save_new_releases_tracks_to_json(app, new_releases_tracks);

    Ok(())
}

/// Saves a vector of simplified track data to a JSON file in the Spotify cache directory
fn save_new_releases_tracks_to_json(app: &mut App, items: Vec<SimplifiedTrack>) {
    let json_data = json!(items);

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push(app.file_name.clone());
    path.push("spotify_cache");
    std::fs::create_dir_all(&path).unwrap();
    path.push("new_releases_tracks.json");

    let mut file = File::create(&path).unwrap();
    let _ = file.write_all(json_data.to_string().as_bytes());
}

/// Processes the new releases tracks data stored in the cache file and populates the app's data structures
pub fn process_new_releases_tracks(app: &mut App) {
    app.new_release_track_names.clear();
    app.new_release_artist_names.clear();
    app.new_release_durations_ms.clear();
    app.new_release_spotify_urls.clear();

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push(app.file_name.clone());
    path.push("spotify_cache");
    path.push("new_releases_tracks.json");

    let file = File::open(&path).expect("Failed to open new_releases_tracks.json");
    let reader = BufReader::new(file);
    let json_data: Value =
        serde_json::from_reader(reader).expect("Failed to parse new_releases_tracks.json");

    // Extract information about each track from the JSON data
    if let Value::Array(tracks) = &json_data {
        // Iterate over each track
        for track in tracks {
            if let Value::Object(track_obj) = track {
                // Extract track name
                if let Some(name) = track_obj.get("name").and_then(|v| v.as_str()) {
                    app.new_release_track_names.push(name.to_owned());
                }

                // Extract first artist name
                if let Some(artists) = track_obj.get("artists").and_then(|v| v.as_array()) {
                    if !artists.is_empty() {
                        if let Some(first_artist) = artists.first().and_then(|v| v.as_object()) {
                            if let Some(artist_name) =
                                first_artist.get("name").and_then(|v| v.as_str())
                            {
                                app.new_release_artist_names.push(artist_name.to_owned());
                            }
                        }
                    }
                }

                // Extract duration in milliseconds
                if let Some(duration) = track_obj.get("duration_ms").and_then(|v| v.as_i64()) {
                    app.new_release_durations_ms.push(duration);
                }

                // Extract external Spotify URL
                if let Some(url) = track_obj
                    .get("external_urls")
                    .and_then(|v| v.get("spotify"))
                    .and_then(|v| v.as_str())
                {
                    app.new_release_spotify_urls.push(url.to_owned());
                }
            }
        }
    }
}
