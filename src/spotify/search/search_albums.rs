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

#[tokio::main]
pub async fn search_selected_album_tracks(app: &mut App) -> Result<(), ClientError> {
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

    // Collect tracks from the selected album
    let mut tracks = Vec::new();
    let id = app.album_links_search_results[app.album_index].as_str();
    let album_id: AlbumId = AlbumId::from_id(id).unwrap();

    // Stream the album tracks and collect them into a vector.
    let stream = spotify
        .album_track(album_id, None)
        .try_for_each(|item| {
            tracks.push(item);
            futures::future::ok(())
        })
        .boxed();

    stream.await?;

    save_tracks_to_json(tracks);

    Ok(())
}

/// Saves a vector of simplified track data to a JSON file in the Spotify cache directory
fn save_tracks_to_json(items: Vec<SimplifiedTrack>) {
    let json_data = json!(items);

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify");
    path.push("spotify_cache");
    std::fs::create_dir_all(&path).unwrap();
    path.push("selected_searched_album_tracks.json");

    let mut file = File::create(&path).unwrap();
    let _ = file.write_all(json_data.to_string().as_bytes());
}

pub fn process_selected_album_tracks(app: &mut App) {
    app.selected_album_tracks_names.clear();
    app.selected_album_tracks_artists.clear();
    app.selected_album_tracks_duration.clear();
    app.selected_album_tracks_links.clear();

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify");
    path.push("spotify_cache");
    path.push("selected_searched_album_tracks.json");

    let file = File::open(&path).expect("Failed to open selected_searched_album_tracks.json");
    let reader = BufReader::new(file);
    let json_data: Value = serde_json::from_reader(reader)
        .expect("Failed to parse selected_searched_album_tracks.json");

    // Extract information about each track from the JSON data
    if let Value::Array(tracks) = &json_data {
        // Iterate over each track
        for track in tracks {
            if let Value::Object(track_obj) = track {
                // Extract track name
                if let Some(name) = track_obj.get("name").and_then(|v| v.as_str()) {
                    app.selected_album_tracks_names.push(name.to_owned());
                }

                // Extract first artist name
                if let Some(artists) = track_obj.get("artists").and_then(|v| v.as_array()) {
                    if !artists.is_empty() {
                        if let Some(first_artist) = artists.get(0).and_then(|v| v.as_object()) {
                            if let Some(artist_name) =
                                first_artist.get("name").and_then(|v| v.as_str())
                            {
                                app.selected_album_tracks_artists
                                    .push(artist_name.to_owned());
                            }
                        }
                    }
                }

                // Extract duration in milliseconds
                if let Some(duration) = track_obj.get("duration_ms").and_then(|v| v.as_i64()) {
                    app.selected_album_tracks_duration.push(duration);
                }

                // Extract external Spotify URL
                if let Some(url) = track_obj
                    .get("external_urls")
                    .and_then(|v| v.get("spotify"))
                    .and_then(|v| v.as_str())
                {
                    app.selected_album_tracks_links.push(url.to_owned());
                }
            }
        }
    }
}
