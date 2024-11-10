use serde::Deserialize;
use serde_yaml;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::app::App;
use crate::structs::Settings;

#[derive(Deserialize, Debug)]
struct SettingsValues(HashMap<String, String>);

/// Reads the settings from the configuration file and returns them as a HashMap
pub fn read_settings(app: &mut App) -> HashMap<String, String> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push(app.file_name.clone());
    path.push("configure");
    path.push("settings.yml");

    let file = File::open(&path).expect("Unable to open settings_values file");
    let reader = BufReader::new(file);
    let SettingsValues(settings_values) =
        serde_yaml::from_reader(reader).expect("Unable to parse settings_values from YAML");

    settings_values
}

/// Sets the settings in the App struct based on the loaded configuration
pub fn set_settings_values(app: &mut App, settings: &mut Settings) {
    let settings_values = read_settings(app);

    if let Some(value_str) = settings_values.get("Volume Increament Value") {
        settings.volume_increment_value = value_str.parse::<u8>().unwrap_or(0);
    }

    if let Some(value_str) = settings_values.get("Volume Decreament Value") {
        settings.volume_decreament_value = value_str.parse::<u8>().unwrap_or(0);
    }

    if let Some(value_str) = settings_values.get("Theme") {
        settings.theme_name = value_str.to_string();
    }
}
