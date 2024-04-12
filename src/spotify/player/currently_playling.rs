use crate::app::App;
use crate::spotify::auth::get_spotify_client;
use chrono::{DateTime, Utc};
use futures::future::err;
use futures::FutureExt;
use futures_util::TryStreamExt;
use rspotify::model::{
    track, Actions, AdditionalType, CurrentlyPlayingContext, CurrentlyPlayingType, Market,
    SavedAlbum,
};
use rspotify::prelude::OAuthClient;
use rspotify::{AuthCodeSpotify, ClientError};
use serde_json::{json, Value};
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;

#[tokio::main]
pub async fn currently_playing() -> Result<(), ClientError> {
    let spotify_client = get_spotify_client().await.unwrap();
    let spotify = match &spotify_client.token {
        Some(token) => AuthCodeSpotify::from_token(token.clone()),
        None => return Err(ClientError::InvalidToken),
    };

    let currently_playing_result = spotify
        .current_playing(
            Some(Market::FromToken),
            Some(
                vec![AdditionalType::Episode]
                    .iter()
                    .map(|x| x as &AdditionalType),
            ),
        )
        .await;

    let currently_playing_tracks: CurrentlyPlayingContext = match currently_playing_result {
        Ok(page) => page.unwrap_or_else(|| CurrentlyPlayingContext {
            item: None,
            currently_playing_type: CurrentlyPlayingType::Unknown,
            actions: Actions::default(),
            is_playing: false,
            context: None,
            timestamp: DateTime::default(),
            progress: None,
        }),
        Err(err) => {
            eprintln!("Error fetching recently played tracks: {}", err);
            CurrentlyPlayingContext {
                item: None,
                currently_playing_type: CurrentlyPlayingType::Unknown,
                actions: Actions::default(),
                is_playing: false,
                context: None,
                timestamp: DateTime::default(),
                progress: None,
            }
        }
    };

    save_data_to_json(currently_playing_tracks);

    Ok(())
}

fn save_data_to_json(items: CurrentlyPlayingContext) {
    let json_data = json!(items);
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("spotify_cache");
    std::fs::create_dir_all(&path).unwrap();
    path.push("currently_playing.json");
    let mut file = File::create(&path).unwrap();
    let _ = file.write_all(json_data.to_string().as_bytes());
}

pub fn process_currently_playing(app: &mut App) {
    app.currrent_timestamp = 0;
    app.ending_timestamp = 0;
    app.currently_playing_artist.clear();
    app.current_playing_name.clear();
    app.current_playing_album.clear();

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("spotify_cache");
    path.push("currently_playing.json");

    let file = File::open(&path).expect("Failed to open currently_playing.json");
    let reader = BufReader::new(file);
    let json_data: Value =
        serde_json::from_reader(reader).expect("Failed to parse currently_playing.json");

    if let Value::Object(currently_playing) = json_data {
        if let Some(progress_ms) = currently_playing.get("progress_ms").and_then(Value::as_i64) {
            app.currrent_timestamp = progress_ms as i64;
        }
        if let Some(is_playing) = currently_playing.get("is_playing").and_then(Value::as_bool) {
            app.is_playing = is_playing;
        }
        if let Some(item) = currently_playing.get("item").and_then(Value::as_object) {
            if let Some(duration_ms) = item.get("duration_ms").and_then(Value::as_i64) {
                app.ending_timestamp = duration_ms as i64;
            }

            if let Some(album) = item.get("album").and_then(Value::as_object) {
                if let Some(album_name) = album.get("name").and_then(Value::as_str) {
                    app.current_playing_album = album_name.to_string();
                }
            }

            if let Some(artist_section) = item.get("artists").and_then(Value::as_array) {
                if let Some(first_artist) = artist_section.get(0).and_then(Value::as_object) {
                    if let Some(artist_name) = first_artist.get("name").and_then(Value::as_str) {
                        app.currently_playing_artist = artist_name.to_string();
                    }
                }
            }

            if let Some(name) = item.get("name").and_then(Value::as_str) {
                app.current_playing_name = name.to_string();
            }
        }
    }

    if app.is_playing {
        app.playback_status = "Playing".to_owned();
    } else {
        app.playback_status = "Paused".to_owned();
    }
}
