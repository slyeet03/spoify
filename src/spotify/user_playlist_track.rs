/*
fetches the tracks of selected playlists
stores their data in json file
store their names,artists,duration and links in their respective variables
*/

use crate::app::App;
use crate::spotify::auth::get_spotify_client;
use futures::FutureExt;
use futures_util::TryStreamExt;
use regex::Regex;
use rspotify::model::{PlaylistId, PlaylistItem};
use rspotify::prelude::BaseClient;
use rspotify::{AuthCodeSpotify, ClientError};
use serde_json::{json, Value};
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;

#[tokio::main]
pub async fn fetch_playlists_tracks(app: &mut App) -> Result<(), ClientError> {
    let spotify_client = get_spotify_client().await;
    let spotify = match &spotify_client.unwrap().token {
        Some(token) => AuthCodeSpotify::from_token(token.clone()),
        None => return Err(ClientError::InvalidToken),
    };

    let playlist_url = app.selected_playlist_uri.as_str();
    let re = Regex::new(r"/playlist/(.+)").unwrap();
    let captures = re.captures(playlist_url).unwrap();
    let playlist_uri = captures.get(1).unwrap().as_str();
    let playlist_id = PlaylistId::from_id(playlist_uri).unwrap();

    let mut playlist_items = Vec::new();

    let stream = spotify
        .playlist_items(playlist_id, None, None)
        .try_for_each(|item| {
            playlist_items.push(item);
            futures::future::ok(())
        })
        .boxed();

    stream.await?;

    save_playlists_to_json(playlist_items);

    Ok(())
}

fn save_playlists_to_json(playlist_items: Vec<PlaylistItem>) {
    let json_data = json!(playlist_items);

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("spotify_cache");
    std::fs::create_dir_all(&path).unwrap();
    path.push("playlist_item.json");

    let mut file = File::create(&path).unwrap();
    let _ = file.write_all(json_data.to_string().as_bytes());
}

use serde_json::Map;
use std::env;

pub fn process_playlist_tracks(app: &mut App) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("spotify_cache");
    path.push("playlist_item.json");

    let file = File::open(&path).expect("Failed to open playlist_item.json");
    let reader = BufReader::new(file);
    let json_data: Value =
        serde_json::from_reader(reader).expect("Failed to parse playlist_item.json");

    if let Value::Array(tracks) = json_data {
        for track in tracks {
            if let Value::Object(track_obj) = track {
                if let Some(track_info) = track_obj.get("track").and_then(Value::as_object) {
                    if let Some(track_name) = track_info.get("name").and_then(Value::as_str) {
                        app.user_playlist_track_names.push(track_name.to_string());
                    }

                    if let Some(track_duration) =
                        track_info.get("duration_ms").and_then(Value::as_u64)
                    {
                        app.user_playlist_track_duration.push(track_duration as i64);
                    }

                    if let Some(artists) = track_info.get("artists").and_then(Value::as_array) {
                        for artist in artists {
                            if let Some(artist_obj) = artist.as_object() {
                                if let Some(artist_name) =
                                    artist_obj.get("name").and_then(Value::as_str)
                                {
                                    app.user_playlist_artist_names.push(artist_name.to_string());
                                }
                            }
                        }
                    }

                    if let Some(external_urls) =
                        track_info.get("external_urls").and_then(Value::as_object)
                    {
                        if let Some(track_link) =
                            external_urls.get("spotify").and_then(Value::as_str)
                        {
                            app.user_playlist_track_links.push(track_link.to_string());
                        }
                    }
                }
            }
        }
    }
}
