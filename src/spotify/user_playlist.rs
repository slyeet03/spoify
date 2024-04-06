use crate::app::App;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf; // Import the App struct

pub fn user_playlist(app: &mut App) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("spotify_cache");
    path.push("playlists.json");

    let file = File::open(&path).expect("Failed to open playlists.json");
    let reader = BufReader::new(file);

    let playlists: Value = serde_json::from_reader(reader).expect("Failed to parse playlists.json");

    if let Value::Array(playlists) = playlists {
        for playlist in playlists {
            if let Value::Object(playlist_obj) = playlist {
                if let Some(name) = playlist_obj.get("name").and_then(Value::as_str) {
                    app.user_playlist_names.push(name.to_string());
                }

                if let Some(link) = playlist_obj
                    .get("external_urls")
                    .and_then(Value::as_object)
                    .and_then(|urls| urls.get("spotify"))
                    .and_then(Value::as_str)
                {
                    app.user_playlist_links.push(link.to_string());
                }
            }
        }
    }
}
