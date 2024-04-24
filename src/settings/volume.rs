use serde::Deserialize;
use serde_yaml;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::app::App;

#[derive(Deserialize, Debug)]
struct VolumeValues(HashMap<String, String>);

/// Reads the volume_values from the configuration file and returns them as a HashMap
pub fn read_volume_values() -> HashMap<String, String> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("configure");
    path.push("settings.yml");

    let file = File::open(&path).expect("Unable to open volume_values file");
    let reader = BufReader::new(file);
    let VolumeValues(volume_values) =
        serde_yaml::from_reader(reader).expect("Unable to parse volume_values from YAML");
    volume_values
}

/// Sets the volume_values in the App struct based on the loaded configuration
pub fn set_volume_values(app: &mut App) {
    let volume_values = read_volume_values();

    if let Some(value_str) = volume_values.get("Volume Increament Value") {
        app.volume_increment_value = value_str.parse::<u8>().unwrap_or(0);
    }

    if let Some(value_str) = volume_values.get("Volume Decreament Value") {
        app.volume_decreament_value = value_str.parse::<u8>().unwrap_or(0);
    }
}
