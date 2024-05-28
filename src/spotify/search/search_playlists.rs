extern crate rspotify;
extern crate serde_json;

use crate::app::App;
use futures::{FutureExt, TryStreamExt};
use rspotify::model::{PlaylistId, PlaylistItem};
use rspotify::{prelude::*, ClientCredsSpotify, ClientError, Credentials};
use serde_json::{json, Value};
use std::env;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;

#[tokio::main]
pub async fn search_selected_playlist_tracks(app: &mut App) -> Result<(), ClientError> {
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

    // Extract the playlist URI from the app's selected playlist URL
    let playlist_url = app.playlist_links_search_results[app.playlist_index].as_str();
    let playlist_id = PlaylistId::from_id(playlist_url).unwrap();

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

    save_tracks_to_json(playlist_items);

    Ok(())
}

/// Saves a vector of simplified track data to a JSON file in the Spotify cache directory
fn save_tracks_to_json(items: Vec<PlaylistItem>) {
    let json_data = json!(items);

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("spotify_cache");
    std::fs::create_dir_all(&path).unwrap();
    path.push("selected_searched_playlist_tracks.json");

    let mut file = File::create(&path).unwrap();
    let _ = file.write_all(json_data.to_string().as_bytes());
}

pub fn process_selected_playlist_tracks(app: &mut App) {
    app.selected_playlist_tracks_names.clear();
    app.selected_playlist_tracks_albums.clear();
    app.selected_playlist_tracks_artists.clear();
    app.selected_playlist_tracks_duration.clear();
    app.selected_playlist_tracks_links.clear();

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("spotify_cache");
    path.push("selected_searched_playlist_tracks.json");

    let file = File::open(&path).expect("Failed to open selected_searched_playlist_tracks.json");
    let reader = BufReader::new(file);
    let json_data: Value = serde_json::from_reader(reader)
        .expect("Failed to parse selected_searched_playlist_tracks.json");

    // Extract information about each track from the JSON data
    if let Value::Array(tracks) = &json_data {
        // Iterate over each track
        for track in tracks {
            if let Value::Object(track_obj) = track {
                // Extract track name
                if let Some(name) = track_obj
                    .get("track")
                    .and_then(|v| v.get("name"))
                    .and_then(|v| v.as_str())
                {
                    app.selected_playlist_tracks_names.push(name.to_owned());
                }

                // Extract artists
                if let Some(artists) = track_obj
                    .get("track")
                    .and_then(|v| v.get("artists"))
                    .and_then(|v| v.as_array())
                {
                    for artist in artists {
                        if let Some(artist_name) = artist.get("name").and_then(|v| v.as_str()) {
                            app.selected_playlist_tracks_artists
                                .push(artist_name.to_owned());
                        }
                    }
                }

                // Extract album name
                if let Some(album_name) = track_obj
                    .get("track")
                    .and_then(|v| v.get("album"))
                    .and_then(|v| v.get("name"))
                    .and_then(|v| v.as_str())
                {
                    app.selected_playlist_tracks_albums
                        .push(album_name.to_owned());
                }

                // Extract duration in milliseconds
                if let Some(duration) = track_obj
                    .get("track")
                    .and_then(|v| v.get("duration_ms"))
                    .and_then(|v| v.as_i64())
                {
                    app.selected_playlist_tracks_duration.push(duration);
                }

                // Extract external Spotify URL
                if let Some(url) = track_obj
                    .get("track")
                    .and_then(|v| v.get("external_urls"))
                    .and_then(|v| v.get("spotify"))
                    .and_then(|v| v.as_str())
                {
                    app.selected_playlist_tracks_links.push(url.to_owned());
                }
            }
        }
    }
}
