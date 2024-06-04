// Fetches tracks from a user's selected Spotify playlist and stores information for display

use crate::app::App;
use crate::spotify::auth::get_spotify_client;
use futures::FutureExt;
use futures_util::TryStreamExt;
use regex::Regex;
use rspotify::model::{PlaylistId, PlaylistItem};
use rspotify::prelude::BaseClient;
use rspotify::ClientError;
use serde_json::{json, Value};
use std::env;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;

/// Fetches playlist tracks from Spotify
#[tokio::main]
pub async fn fetch_made_fy_tracks(app: &mut App) -> Result<(), ClientError> {
    // Get a Spotify client using an existing access token (if available).
    let spotify = get_spotify_client(app).await?;

    // Extract the playlist URI from the app's selected playlist URL
    let id = app.made_fy_playlist_links[app.made_fy_index].as_str();
    let re = Regex::new(r"/playlist/(.+)").unwrap();
    let captures = re.captures(id).unwrap();
    let playlist_uri = captures.get(1).unwrap().as_str();
    let playlist_id = PlaylistId::from_id(playlist_uri).unwrap();

    // Collect information about the playlist items (tracks)
    let mut playlist_items = Vec::new();

    let stream = spotify
        .playlist_items(playlist_id, None, None)
        .try_for_each(|item| {
            playlist_items.push(item);
            futures::future::ok(())
        })
        .boxed();

    stream.await?;

    save_file_to_json(playlist_items);

    Ok(())
}

/// Saves playlist items data (including tracks) to a JSON file
fn save_file_to_json(playlist_items: Vec<PlaylistItem>) {
    let json_data = json!(playlist_items);

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify");
    path.push("spotify_cache");
    std::fs::create_dir_all(&path).unwrap();
    path.push("made_fy_tracks.json");

    let mut file = File::create(&path).unwrap();
    let _ = file.write_all(json_data.to_string().as_bytes());
}

/// Processes the playlist tracks data stored in the cache file and populates the app's data structures
pub fn process_made_fy_tracks(app: &mut App) {
    // Clear any existing playlist tracks data in the app before processing new data
    app.made_fy_track_links.clear();
    app.made_fy_track_names.clear();
    app.made_fy_track_duration.clear();
    app.made_fy_artist_names.clear();
    app.made_fy_album_names.clear();

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify");
    path.push("spotify_cache");
    path.push("made_fy_tracks.json");

    let file = File::open(&path).expect("Failed to open made_fy_tracks.json");
    let reader = BufReader::new(file);
    let json_data: Value =
        serde_json::from_reader(reader).expect("Failed to parse made_fy_tracks.json");

    // Extract information about each track from the JSON data and populate the app's data structures for displaying the playlist
    if let Value::Array(tracks) = json_data {
        for track in tracks {
            if let Value::Object(track_obj) = track {
                if let Some(track_info) = track_obj.get("track").and_then(Value::as_object) {
                    if let Some(track_name) = track_info.get("name").and_then(Value::as_str) {
                        app.made_fy_track_names.push(track_name.to_string());
                    }

                    if let Some(track_duration) =
                        track_info.get("duration_ms").and_then(Value::as_u64)
                    {
                        app.made_fy_track_duration.push(track_duration as i64);
                    }

                    if let Some(artists) = track_info.get("artists").and_then(Value::as_array) {
                        if let Some(first_artist) = artists.get(0).and_then(Value::as_object) {
                            if let Some(artist_name) =
                                first_artist.get("name").and_then(Value::as_str)
                            {
                                app.made_fy_artist_names.push(artist_name.to_string());
                            }
                        }
                    }
                    if let Some(albums) = track_info.get("album").and_then(Value::as_object) {
                        if let Some(album_name) = albums.get("name").and_then(Value::as_str) {
                            app.made_fy_album_names.push(album_name.to_string());
                        }
                    }

                    if let Some(external_urls) =
                        track_info.get("external_urls").and_then(Value::as_object)
                    {
                        if let Some(track_link) =
                            external_urls.get("spotify").and_then(Value::as_str)
                        {
                            app.made_fy_track_links.push(track_link.to_string());
                        }
                    }
                }
            }
        }
    }
}
