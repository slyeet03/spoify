use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;

use crate::app::App;
use crate::spotify::auth::{get_spotify_client, SpotifyClient};
use futures::{pin_mut, FutureExt};
use futures_util::TryStreamExt;
use rspotify::model::{CursorBasedPage, PlayHistory, SavedAlbum};
use rspotify::prelude::OAuthClient;
use rspotify::{AuthCodeSpotify, ClientError};
use serde_json::{json, Value};

#[tokio::main]
pub async fn user_albums() -> Result<(), ClientError> {
    let spotify_client = get_spotify_client().await.unwrap();

    let spotify = match &spotify_client.token {
        Some(token) => AuthCodeSpotify::from_token(token.clone()),
        None => return Err(ClientError::InvalidToken),
    };
    let mut albums = Vec::new();
    // Executing the futures sequentially
    let stream = spotify
        .current_user_saved_albums(None)
        .try_for_each(|item| {
            albums.push(item);
            futures::future::ok(())
        })
        .boxed();

    stream.await?;

    save_albums_to_json(albums);

    Ok(())
}

fn save_albums_to_json(albums: Vec<SavedAlbum>) {
    let json_data = json!(albums);

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("spotify_cache");
    std::fs::create_dir_all(&path).unwrap();
    path.push("user_albums.json");

    let mut file = File::create(&path).unwrap();
    let _ = file.write_all(json_data.to_string().as_bytes());
}

pub fn process_user_albums(app: &mut App) {
    app.user_album_names.clear();
    app.user_album_links.clear();
    app.user_album_tracks.clear();
    app.user_album_artist_names.clear();

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("spotify_cache");
    path.push("user_albums.json");

    let file = File::open(&path).expect("Failed to open user_albums.json");
    let reader = BufReader::new(file);
    let json_data: Value =
        serde_json::from_reader(reader).expect("Failed to parse user_albums.json");

    if let Value::Array(albums) = json_data {
        for album in albums {
            if let Value::Object(album_obj) = album {
                if let Some(album_info) = album_obj.get("album").and_then(Value::as_object) {
                    if let Some(album_name) = album_info.get("name").and_then(Value::as_str) {
                        app.user_album_names.push(album_name.to_string());
                    }

                    if let Some(external_urls) =
                        album_info.get("external_urls").and_then(Value::as_object)
                    {
                        if let Some(album_link) =
                            external_urls.get("spotify").and_then(Value::as_str)
                        {
                            app.user_album_links.push(album_link.to_string());
                        }
                    }
                    if let Some(artists) = album_info.get("artists").and_then(Value::as_array) {
                        if let Some(first_artist) = artists.get(0).and_then(Value::as_object) {
                            if let Some(artist_name) =
                                first_artist.get("name").and_then(Value::as_str)
                            {
                                app.user_album_artist_names.push(artist_name.to_string());
                            }
                        }
                    }
                    if let Some(tracks) = album_info.get("tracks").and_then(Value::as_object) {
                        if let Some(track_total) = tracks.get("total").and_then(Value::as_str) {
                            app.user_album_tracks.push(track_total.to_string());
                        }
                    }
                }
            }
        }
    }
}
