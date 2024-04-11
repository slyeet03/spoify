use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;

use crate::app::App;
use crate::spotify::auth::{get_spotify_client, SpotifyClient};
use futures::{pin_mut, FutureExt};
use futures_util::TryStreamExt;
use rspotify::model::{CursorBasedPage, FullArtist, PlayHistory};
use rspotify::prelude::OAuthClient;
use rspotify::{AuthCodeSpotify, ClientError};
use serde_json::{json, Value};

#[tokio::main]
pub async fn user_artists() -> Result<(), ClientError> {
    let spotify_client = get_spotify_client().await.unwrap();

    let spotify = match &spotify_client.token {
        Some(token) => AuthCodeSpotify::from_token(token.clone()),
        None => return Err(ClientError::InvalidToken),
    };

    let artist_result = spotify.current_user_followed_artists(None, Some(50)).await;

    let artist_tracks: Vec<FullArtist> = match artist_result {
        Ok(page) => page.items.into_iter().collect(),
        Err(err) => {
            eprintln!("Error fetching recently played tracks: {}", err);
            Vec::new()
        }
    };
    save_artist_to_json(artist_tracks);
    Ok(())
}

fn save_artist_to_json(items: Vec<FullArtist>) {
    let json_data = json!(items);

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("spotify_cache");
    std::fs::create_dir_all(&path).unwrap();
    path.push("user_artist.json");

    let mut file = File::create(&path).unwrap();
    let _ = file.write_all(json_data.to_string().as_bytes());
}

pub fn process_user_artists(app: &mut App) {
    app.user_artist_names.clear();
    app.user_artist_links.clear();

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("spotify_cache");
    path.push("user_artist.json");

    let file = File::open(&path).expect("Failed to open user_artist.json");
    let reader = BufReader::new(file);
    let json_data: Value =
        serde_json::from_reader(reader).expect("Failed to parse user_artist.json");

    if let Value::Array(shows) = json_data {
        for show in shows {
            if let Value::Object(show_obj) = show {
                if let Some(show_name) = show_obj.get("name").and_then(Value::as_str) {
                    app.user_artist_names.push(show_name.to_string());
                }

                if let Some(external_urls) =
                    show_obj.get("external_urls").and_then(Value::as_object)
                {
                    if let Some(show_link) = external_urls.get("spotify").and_then(Value::as_str) {
                        app.user_artist_links.push(show_link.to_string());
                    }
                }
            }
        }
    }
}
