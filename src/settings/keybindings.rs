use serde::Deserialize;
use serde_yaml;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use yaml_rust::YamlLoader;

use crate::app::App;
use crate::structs::Key;

#[derive(Deserialize, Debug)]
struct Keybindings(HashMap<String, String>);

/// Reads the keybindings from the configuration file and returns them as a HashMap
pub fn read_keybindings(app: &mut App) -> HashMap<String, String> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push(app.file_name.clone());
    path.push("configure");
    path.push("keybindings.yml");

    let file = File::open(&path).expect("Unable to open keybindings file");
    let reader = BufReader::new(file);
    let Keybindings(keybindings) =
        serde_yaml::from_reader(reader).expect("Unable to parse keybindings from YAML");
    keybindings
}

/// Sets the keybindings in the App struct based on the loaded configuration
pub fn set_keybindings(app: &mut App, key: &mut Key) {
    let keybindings = read_keybindings(app);

    key.go_to_search_key = match keybindings.get("Go to Search") {
        Some(s) => s.chars().next().unwrap_or(' '),
        None => ' ',
    };
    key.go_to_library_key = match keybindings.get("Go to Library") {
        Some(s) => s.chars().next().unwrap_or(' '),
        None => ' ',
    };
    key.go_to_user_playlists_key = match keybindings.get("Go to User Playlists") {
        Some(s) => s.chars().next().unwrap_or(' '),
        None => ' ',
    };

    key.exit_application_key = match keybindings.get("Exit Application/Go Back") {
        Some(s) => s.chars().next().unwrap_or(' '),
        None => ' ',
    };

    key.pause_play_key = match keybindings.get("Pause/Play") {
        Some(s) => s.chars().next().unwrap_or(' '),
        None => ' ',
    };
    key.help_key = match keybindings.get("Help") {
        Some(s) => s.chars().next().unwrap_or(' '),
        None => ' ',
    };
    key.volume_up_key = match keybindings.get("Volume Up") {
        Some(s) => s.chars().next().unwrap_or(' '),
        None => ' ',
    };

    key.volume_down_key = match keybindings.get("Volume Down") {
        Some(s) => s.chars().next().unwrap_or(' '),
        None => ' ',
    };
    key.new_release_key = match keybindings.get("Go to New Release") {
        Some(s) => s.chars().next().unwrap_or(' '),
        None => ' ',
    };
    key.next_track_key = match keybindings.get("Play next track") {
        Some(s) => s.chars().next().unwrap_or(' '),
        None => ' ',
    };
    key.previous_track_key = match keybindings.get("Play previous track") {
        Some(s) => s.chars().next().unwrap_or(' '),
        None => ' ',
    };
    key.error_key = match keybindings.get("Error Screen") {
        Some(s) => s.chars().next().unwrap_or(' '),
        None => ' ',
    };
    key.player_fullscreen_key = match keybindings.get("Player in fullscreen") {
        Some(s) => s.chars().next().unwrap_or(' '),
        None => ' ',
    };
    key.change_keybind = match keybindings.get("Change Key Bindings") {
        Some(s) => s.chars().next().unwrap_or(' '),
        None => ' ',
    };
    key.refresh_key = match keybindings.get("Refresh") {
        Some(s) => s.chars().next().unwrap_or(' '),
        None => ' ',
    };
}

/// Parses the keybindings from the configuration file and populates the tasks and first_keys fields in the App struct
pub fn parse_keybindings(app: &mut App, keys: &mut Key) {
    // Construct the paths relative to the root directory
    let mut yaml_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    yaml_path.push("..");
    yaml_path.push(app.file_name.clone());
    yaml_path.push("configure");
    yaml_path.push("keybindings.yml");

    // Load the YAML file
    let yaml_data = std::fs::read_to_string(&yaml_path).expect("Failed to read YAML file");

    let docs = YamlLoader::load_from_str(yaml_data.as_str()).expect("Failed to load YAML data");
    let doc = &docs[0];

    // Iterate over the YAML data
    for (key, value) in doc.as_hash().unwrap() {
        let task = key.as_str().unwrap().to_string();
        let key_binding = value.as_str().unwrap().to_string();

        keys.tasks.push(task);
        keys.first_keys.push(key_binding);
    }
}
