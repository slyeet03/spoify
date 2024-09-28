use std::fs::File;
use std::io::{BufReader, Write};

use crate::app::App;
use crate::spotify::auth::get_spotify_client;
use crate::util::get_project_dir;
use futures::FutureExt;
use futures_util::TryStreamExt;
use rspotify::clients::BaseClient;
use rspotify::model::SimplifiedPlaylist;
use rspotify::ClientError;
use serde_json::{json, Value};

/// Fetches a user's liked songs from Spotify
#[tokio::main]
pub async fn made_fy(app: &mut App) -> Result<(), ClientError> {
    // Get a Spotify client using an existing access token (if available).
    let spotify = get_spotify_client(app).await?;

    let category_id = "0JQ5DAt0tbjZptfcdMSKl3";
    // Collect all the user's liked songs from Spotify.
    let mut made_fy_playlists = Vec::new();
    // Executing the futures sequentially
    let stream = spotify
        .category_playlists(category_id, None)
        .try_for_each(|item| {
            made_fy_playlists.push(item);
            futures::future::ok(())
        })
        .boxed();

    stream.await?;

    save_file_to_json(app, made_fy_playlists);

    Ok(())
}

/// Saves a vector of liked songs to a JSON file in the Spotify cache directory.
fn save_file_to_json(app: &mut App, made_fy_playlists: Vec<SimplifiedPlaylist>) {
    let json_data = json!(made_fy_playlists);

    let project_dir = get_project_dir(&app.file_name);
    let mut path = project_dir.join("spotify_cache");
    std::fs::create_dir_all(&path).unwrap();
    path = path.join("made_fy_playlists.json");

    let mut file = File::create(&path).unwrap();
    let _ = file.write_all(json_data.to_string().as_bytes());
}

/// Processes the playlist data stored in the cache file and populates the app's data structures
pub fn process_made_fy(app: &mut App) {
    app.made_fy_playlist_names.clear();
    app.made_fy_playlist_links.clear();
    app.made_fy_playlist_track_total.clear();

    let project_dir = get_project_dir(&app.file_name);
    let mut path = project_dir.join("spotify_cache");
    path = path.join("made_fy_playlists.json");

    let file = File::open(&path).expect("Failed to open made_fy_playlists.json");
    let reader = BufReader::new(file);

    let playlists: Value =
        serde_json::from_reader(reader).expect("Failed to parse made_fy_playlists.json");

    // Extract information about each playlist from the JSON data and populate the app's data structures for displaying the playlists
    if let Value::Array(playlists) = playlists {
        for playlist in playlists {
            if let Value::Object(playlist_obj) = playlist {
                if let Some(name) = playlist_obj.get("name").and_then(Value::as_str) {
                    app.made_fy_playlist_names.push(name.to_string());
                }

                if let Some(link) = playlist_obj
                    .get("external_urls")
                    .and_then(Value::as_object)
                    .and_then(|urls| urls.get("spotify"))
                    .and_then(Value::as_str)
                {
                    app.made_fy_playlist_links.push(link.to_string());
                }

                if let Some(total_tracks) = playlist_obj
                    .get("tracks")
                    .and_then(Value::as_object)
                    .and_then(|tracks| tracks.get("total"))
                    .and_then(Value::as_i64)
                {
                    app.made_fy_playlist_track_total.push(total_tracks);
                }
            }
        }
    }
}
