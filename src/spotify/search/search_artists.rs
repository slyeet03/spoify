extern crate rspotify;
extern crate serde_json;

use crate::app::App;
use rspotify::model::{ArtistId, FullTrack};
use rspotify::{prelude::*, ClientCredsSpotify, ClientError, Credentials};
use serde_json::{json, Value};
use std::env;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;

#[tokio::main]
pub async fn search_selected_artist_tracks(app: &mut App) -> Result<(), ClientError> {
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

    let id = app.artist_links_search_results[app.artist_index].as_str();
    let artist_id: ArtistId = ArtistId::from_id(id).unwrap();

    let tracks = match spotify.artist_top_tracks(artist_id, None).await {
        Ok(tracks) => tracks,
        Err(err) => {
            eprintln!("Error fetching recently played tracks: {}", err);
            Vec::new()
        }
    };

    save_tracks_to_json(tracks);

    Ok(())
}

/// Saves a vector of simplified track data to a JSON file in the Spotify cache directory
fn save_tracks_to_json(items: Vec<FullTrack>) {
    let json_data = json!(items);

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify");
    path.push("spotify_cache");
    std::fs::create_dir_all(&path).unwrap();
    path.push("selected_searched_artist_tracks.json");

    let mut file = File::create(&path).unwrap();
    let _ = file.write_all(json_data.to_string().as_bytes());
}

pub fn process_selected_artist_tracks(app: &mut App) {
    // Clear any existing user track data in the app before processing
    app.selected_artist_tracks_names.clear();
    app.selected_artist_track_album_names.clear();
    app.selected_artist_tracks_duration.clear();
    app.selected_artist_tracks_links.clear();

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify");
    path.push("spotify_cache");
    path.push("selected_searched_artist_tracks.json");

    let file = File::open(&path).expect("Failed to open selected_searched_artist_tracks.json");
    let reader = BufReader::new(file);
    let json_data: Vec<Value> = serde_json::from_reader(reader)
        .expect("Failed to parse selected_searched_artist_tracks.json");

    // Extract information about each track from the JSON data
    for track_data in json_data {
        if let Value::Object(track_obj) = track_data {
            if let Some(track_name) = track_obj.get("name").and_then(Value::as_str) {
                app.selected_artist_tracks_names
                    .push(track_name.to_string());
            }

            if let Some(album_info) = track_obj.get("album").and_then(Value::as_object) {
                if let Some(album_name) = album_info.get("name").and_then(Value::as_str) {
                    app.selected_artist_track_album_names
                        .push(album_name.to_string());
                }
            }

            if let Some(duration_ms) = track_obj.get("duration_ms").and_then(Value::as_i64) {
                app.selected_artist_tracks_duration.push(duration_ms);
            }

            if let Some(external_urls) = track_obj.get("external_urls").and_then(Value::as_object) {
                if let Some(track_link) = external_urls.get("spotify").and_then(Value::as_str) {
                    app.selected_artist_tracks_links
                        .push(track_link.to_string());
                }
            }
        }
    }
}
