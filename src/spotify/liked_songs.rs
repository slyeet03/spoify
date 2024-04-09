use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;

use crate::app::App;
use crate::spotify::auth::{get_spotify_client, SpotifyClient};
use futures::{pin_mut, FutureExt};
use futures_util::TryStreamExt;
use rspotify::model::{CursorBasedPage, PlayHistory, SavedTrack};
use rspotify::prelude::OAuthClient;
use rspotify::{AuthCodeSpotify, ClientError};
use serde_json::{json, Value};

#[tokio::main]
pub async fn liked_tracks() -> Result<(), ClientError> {
    let spotify_client = get_spotify_client().await.unwrap();

    let spotify = match &spotify_client.token {
        Some(token) => AuthCodeSpotify::from_token(token.clone()),
        None => return Err(ClientError::InvalidToken),
    };
    let mut liked_songs = Vec::new();
    // Executing the futures sequentially
    let stream = spotify
        .current_user_saved_tracks(None)
        .try_for_each(|item| {
            liked_songs.push(item);
            futures::future::ok(())
        })
        .boxed();

    stream.await?;

    save_liked_songs_to_json(liked_songs);

    Ok(())
}

fn save_liked_songs_to_json(liked_songs: Vec<SavedTrack>) {
    let json_data = json!(liked_songs);

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("spotify_cache");
    std::fs::create_dir_all(&path).unwrap();
    path.push("liked_songs.json");

    let mut file = File::create(&path).unwrap();
    let _ = file.write_all(json_data.to_string().as_bytes());
}

pub fn process_liked_tracks(app: &mut App) {
    app.liked_song_links.clear();
    app.liked_song_names.clear();
    app.liked_song_duration.clear();
    app.liked_song_artist_names.clear();
    app.liked_song_album_names.clear();

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("spotify_cache");
    path.push("liked_songs.json");

    let file = File::open(&path).expect("Failed to open liked_songs.json");
    let reader = BufReader::new(file);
    let json_data: Value =
        serde_json::from_reader(reader).expect("Failed to parse liked_songs.json");

    if let Value::Array(tracks) = json_data {
        for track in tracks {
            if let Value::Object(track_obj) = track {
                if let Some(track_info) = track_obj.get("track").and_then(Value::as_object) {
                    if let Some(track_name) = track_info.get("name").and_then(Value::as_str) {
                        app.liked_song_names.push(track_name.to_string());
                    }

                    if let Some(track_duration) =
                        track_info.get("duration_ms").and_then(Value::as_u64)
                    {
                        app.liked_song_duration.push(track_duration as i64);
                    }

                    if let Some(artists) = track_info.get("artists").and_then(Value::as_array) {
                        if let Some(first_artist) = artists.get(0).and_then(Value::as_object) {
                            if let Some(artist_name) =
                                first_artist.get("name").and_then(Value::as_str)
                            {
                                app.liked_song_artist_names.push(artist_name.to_string());
                            }
                        }
                    }
                    if let Some(albums) = track_info.get("album").and_then(Value::as_object) {
                        if let Some(album_name) = albums.get("name").and_then(Value::as_str) {
                            app.liked_song_album_names.push(album_name.to_string());
                        }
                    }
                    if let Some(external_urls) =
                        track_info.get("external_urls").and_then(Value::as_object)
                    {
                        if let Some(track_link) =
                            external_urls.get("spotify").and_then(Value::as_str)
                        {
                            app.liked_song_links.push(track_link.to_string());
                        }
                    }
                }
            }
        }
    }
}
