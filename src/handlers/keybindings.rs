use serde::Deserialize;
use serde_json::Value;
use serde_yaml;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::app::App;

#[derive(Deserialize, Debug)]
struct Keybindings(HashMap<String, String>);

pub fn read_keybindings() -> HashMap<String, String> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("configure");
    path.push("keybindings.yml");

    let file = File::open(&path).expect("Unable to open keybindings file");
    let reader = BufReader::new(file);
    let Keybindings(keybindings) =
        serde_yaml::from_reader(reader).expect("Unable to parse keybindings from YAML");
    keybindings
}

pub fn set_keybindings(app: &mut App) {
    let keybindings = read_keybindings();

    app.go_to_search_key = match keybindings.get("Go to Search") {
        Some(s) => s.chars().next().unwrap_or(' '),
        None => ' ',
    };
    app.go_to_library_key = match keybindings.get("Go to Library") {
        Some(s) => s.chars().next().unwrap_or(' '),
        None => ' ',
    };
    app.go_to_user_playlists_key = match keybindings.get("Go to User Playlists") {
        Some(s) => s.chars().next().unwrap_or(' '),
        None => ' ',
    };

    app.exit_application_key = match keybindings.get("Exit Application") {
        Some(s) => s.chars().next().unwrap_or(' '),
        None => ' ',
    };

    app.pause_play_key = match keybindings.get("Pause/Play") {
        Some(s) => s.chars().next().unwrap_or(' '),
        None => ' ',
    };
    app.help_key = match keybindings.get("Help") {
        Some(s) => s.chars().next().unwrap_or(' '),
        None => ' ',
    };
    app.volume_up_key = match keybindings.get("Volume Up") {
        Some(s) => s.chars().next().unwrap_or(' '),
        None => ' ',
    };

    app.volume_down_key = match keybindings.get("Volume Down") {
        Some(s) => s.chars().next().unwrap_or(' '),
        None => ' ',
    };
}

pub fn process_keybindings(app: &mut App) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("spotify_cache");
    path.push("keybindings.json");

    let file = File::open(&path).expect("Failed to open keybindings.json");
    let reader = BufReader::new(file);
    let json_data: Vec<Value> =
        serde_json::from_reader(reader).expect("Failed to parse keybindings.json");

    for obj in json_data {
        if let Value::Object(data) = obj {
            if let Some(first_key) = data.get("first key").and_then(Value::as_str) {
                app.first_keys.push(first_key.to_string());
            }

            if let Some(task) = data.get("task").and_then(Value::as_str) {
                app.tasks.push(task.to_string());
            }
        }
    }
}
