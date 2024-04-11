use crate::app::App;
use crate::spotify::auth::get_spotify_client;
use rspotify::clients::OAuthClient;
use rspotify::model::Device;
use rspotify::{AuthCodeSpotify, ClientError};
use serde_json::Value;
use serde_json::{self, json};
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;

#[tokio::main]
pub async fn device() -> Result<(), ClientError> {
    let spotify_client = get_spotify_client().await.unwrap();

    let spotify = match &spotify_client.token {
        Some(token) => AuthCodeSpotify::from_token(token.clone()),
        None => return Err(ClientError::InvalidToken),
    };

    let devices = spotify.device().await.unwrap();

    save_devices_to_json(devices);

    Ok(())
}

fn save_devices_to_json(items: Vec<Device>) {
    let json_data = json!(items);

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("spotify_cache");
    std::fs::create_dir_all(&path).unwrap();
    path.push("devices.json");

    let mut file = File::create(&path).unwrap();
    let _ = file.write_all(json_data.to_string().as_bytes());
}

pub fn process_devices(app: &mut App) {
    app.devices_names.clear();
    app.devices_volume.clear();

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("spotify_cache");
    path.push("devices.json");

    let file = File::open(&path).expect("Failed to open devices.json");
    let reader = BufReader::new(file);
    let json_data: Value = serde_json::from_reader(reader).expect("Failed to parse devices.json");

    if let Value::Array(devices) = json_data {
        for device in devices {
            if let Value::Object(device_obj) = device {
                if let Some(device_name) = device_obj.get("name").and_then(Value::as_str) {
                    app.devices_names.push(device_name.to_string());
                }

                if let Some(volume) = device_obj.get("volume_percent").and_then(Value::as_u64) {
                    app.devices_volume.push(volume as usize);
                }
                if let Some(is_active) = device_obj.get("is_active").and_then(Value::as_bool) {
                    app.is_device_active.push(is_active as bool);
                }
                if let Some(device_id) = device_obj.get("id").and_then(Value::as_str) {
                    app.device_ids.push(device_id.to_string());
                }
            }
        }
    }
}

pub fn get_current_device(app: &mut App) {
    for value in &app.is_device_active {
        if *value {
            for name in &app.devices_names {
                app.current_device_name = name.to_string();
            }
            for volume in &app.devices_volume {
                app.current_device_volume = volume.to_string();
            }
            for id in &app.device_ids {
                app.current_device_id = Some(id.to_string());
            }
        }
    }
}
