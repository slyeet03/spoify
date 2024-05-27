extern crate rspotify;
extern crate serde_json;

use crate::app::App;
use futures::{FutureExt, TryStreamExt};
use rspotify::model::{AlbumId, ArtistId, FullTrack, SimplifiedAlbum, SimplifiedTrack};
use rspotify::{prelude::*, ClientCredsSpotify, ClientError, Credentials};
use serde_json::{json, Value};
use std::env;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;

/// Fetches the tracks from a new release album and stores them for later use
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

    // Collect tracks from the selected album
    let mut albums = Vec::new();
    let id = app.artist_links_search_results[app.artist_index].as_str();
    let artist_id: ArtistId = ArtistId::from_id(id).unwrap();

    // Stream the album tracks and collect them into a vector.
    let stream = spotify
        .artist_albums(artist_id, None, None)
        .try_for_each(|item| {
            albums.push(item);
            futures::future::ok(())
        })
        .boxed();

    stream.await?;

    save_tracks_to_json(albums);

    Ok(())
}

/// Saves a vector of simplified track data to a JSON file in the Spotify cache directory
fn save_tracks_to_json(items: Vec<SimplifiedAlbum>) {
    let json_data = json!(items);

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("spotify_cache");
    std::fs::create_dir_all(&path).unwrap();
    path.push("selected_searched_artist_albums.json");

    let mut file = File::create(&path).unwrap();
    let _ = file.write_all(json_data.to_string().as_bytes());
}

/// Processes the saved albums data stored in the cache file and populates the app's data structures
pub fn process_selected_artist_albums(app: &mut App) {
    // Clear any existing user album data in the app before processing
    app.selected_artist_albums_names.clear();
    app.selected_artist_albums_links.clear();
    app.selected_artist_albums_total_tracks.clear();

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("spotify_cache");
    path.push("selected_searched_artist_albums.json");

    let file = File::open(&path).expect("Failed to open selected_searched_artist_albums.json");
    let reader = BufReader::new(file);
    let json_data: Value = serde_json::from_reader(reader)
        .expect("Failed to parse selected_searched_artist_albums.json");

    // Extract information about each saved album from the JSON data
    if let Value::Array(albums) = json_data {
        for album in albums {
            if let Value::Object(album_obj) = album {
                if let Some(album_info) = album_obj.get("album").and_then(Value::as_object) {
                    if let Some(album_name) = album_info.get("name").and_then(Value::as_str) {
                        app.selected_artist_albums_names
                            .push(album_name.to_string());
                    }

                    if let Some(external_urls) =
                        album_info.get("external_urls").and_then(Value::as_object)
                    {
                        if let Some(album_link) =
                            external_urls.get("spotify").and_then(Value::as_str)
                        {
                            app.selected_artist_albums_links
                                .push(album_link.to_string());
                        }
                    }

                    if let Some(tracks) = album_info.get("tracks").and_then(Value::as_object) {
                        if let Some(total_tracks) = tracks.get("total").and_then(Value::as_u64) {
                            app.selected_artist_albums_total_tracks
                                .push(total_tracks as usize);
                        }
                    }
                }
            }
        }
    }
}
