use crate::app::App;
use crate::spotify::auth::get_spotify_client;
use chrono::DateTime;
use rspotify::model::{
    Actions, AdditionalType, CurrentPlaybackContext, CurrentlyPlayingType, Device, DeviceType,
    Market, RepeatState,
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
        .current_playback(
            Some(Market::FromToken),
            Some(
                vec![AdditionalType::Episode]
                    .iter()
                    .map(|x| x as &AdditionalType),
            ),
        )
        .await;

    let currently_playing_tracks: CurrentPlaybackContext = match currently_playing_result {
        Ok(page) => page.unwrap_or_else(|| CurrentPlaybackContext {
            device: Device {
                id: None,
                is_active: false,
                is_private_session: false,
                is_restricted: false,
                name: "".to_string(),
                _type: DeviceType::Computer,
                volume_percent: Some(0),
            },
            repeat_state: RepeatState::Off,
            shuffle_state: false,
            context: None,
            timestamp: DateTime::default(),
            progress: None,
            is_playing: false,
            item: None,
            currently_playing_type: CurrentlyPlayingType::Unknown,
            actions: Actions::default(),
        }),
        Err(err) => {
            eprintln!("Error fetching currently played tracks: {}", err);
            CurrentPlaybackContext {
                device: Device {
                    id: None,
                    is_active: false,
                    is_private_session: false,
                    is_restricted: false,
                    name: "".to_string(),
                    _type: DeviceType::Computer,
                    volume_percent: Some(0),
                },
                repeat_state: RepeatState::Off,
                shuffle_state: false,
                context: None,
                timestamp: DateTime::default(),
                progress: None,
                is_playing: false,
                item: None,
                currently_playing_type: CurrentlyPlayingType::Unknown,
                actions: Actions::default(),
            }
        }
    };

    save_data_to_json(currently_playing_tracks);

    Ok(())
}

fn save_data_to_json(items: CurrentPlaybackContext) {
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
    app.currrent_timestamp = 0.0;
    app.ending_timestamp = 0.0;
    app.currently_playing_artist.clear();
    app.current_playing_name.clear();
    app.current_playing_album.clear();
    app.current_device_name.clear();
    app.current_device_volume.clear();

    let mut repeat_state = String::new();

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
        if let Some(currently_playing_type) = currently_playing
            .get("currently_playing_type")
            .and_then(Value::as_str)
        {
            app.currently_playing_media_type = currently_playing_type.to_string();
        }
        if let Some(progress_ms) = currently_playing.get("progress_ms").and_then(Value::as_i64) {
            app.currrent_timestamp = progress_ms as f64;
        }
        if let Some(is_playing) = currently_playing.get("is_playing").and_then(Value::as_bool) {
            app.is_playing = is_playing;
        }
        if let Some(repeat) = currently_playing
            .get("repeat_state")
            .and_then(Value::as_str)
        {
            repeat_state = repeat.to_string();
        }
        if let Some(device) = currently_playing.get("device").and_then(Value::as_object) {
            if let Some(device_name) = device.get("name").and_then(Value::as_str) {
                app.current_device_name = device_name.to_string();
            }
            if let Some(device_volume) = device.get("volume_percent").and_then(Value::as_u64) {
                app.current_device_volume = device_volume.to_string();
            }
        }
        if let Some(shuffle) = currently_playing
            .get("shuffle_state")
            .and_then(Value::as_bool)
        {
            app.is_shuffle = shuffle;
        }

        if let Some(item) = currently_playing.get("item").and_then(Value::as_object) {
            if let Some(duration_ms) = item.get("duration_ms").and_then(Value::as_i64) {
                app.ending_timestamp = duration_ms as f64;
            }
            if app.currently_playing_media_type == "episode" {
                if let Some(show) = item.get("show").and_then(Value::as_object) {
                    if let Some(show_name) = show.get("name").and_then(Value::as_str) {
                        app.current_playing_album = show_name.to_string();
                    }
                }
            } else {
                if let Some(album) = item.get("album").and_then(Value::as_object) {
                    if let Some(album_name) = album.get("name").and_then(Value::as_str) {
                        app.current_playing_album = album_name.to_string();
                    }
                }

                if let Some(artist_section) = item.get("artists").and_then(Value::as_array) {
                    if let Some(first_artist) = artist_section.get(0).and_then(Value::as_object) {
                        if let Some(artist_name) = first_artist.get("name").and_then(Value::as_str)
                        {
                            app.currently_playing_artist = artist_name.to_string();
                        }
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
    if app.is_shuffle {
        app.shuffle_status = "On".to_string();
    } else if !app.is_shuffle {
        app.shuffle_status = "Off".to_string();
    }
    if repeat_state == "track" {
        app.repeat_status = "Track".to_string();
    } else if repeat_state == "context" {
        app.repeat_status = "Album/Playlist".to_string();
    } else {
        app.repeat_status = "Off".to_string();
    }
}
