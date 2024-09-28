use crate::app::App;
use crate::spotify::auth::get_spotify_client;
use crate::util::get_project_dir;
use rspotify::model::PlayHistory;
use rspotify::prelude::OAuthClient;
use rspotify::ClientError;
use serde_json::{json, Value};
use std::fs::File;
use std::io::{BufReader, Write};

/// Fetches a user's recently played tracks from Spotify
#[tokio::main]
pub async fn recently_played(app: &mut App) -> Result<(), ClientError> {
    // Get a Spotify client using an existing access token (if available).
    let spotify = get_spotify_client(app).await?;

    // Retrieve up to 50 of the user's recently played tracks
    let recently_played_result = spotify.current_user_recently_played(Some(50), None).await;

    let recently_played_tracks: Vec<PlayHistory> = match recently_played_result {
        Ok(page) => page.items.into_iter().collect(),
        Err(err) => {
            app.error_text = format!("Error fetching recently played tracks: {}", err);
            Vec::new()
        }
    };
    save_recently_played_to_json(app, recently_played_tracks);
    Ok(())
}

/// Saves a vector of recently played tracks to a JSON file in the Spotify cache directory
fn save_recently_played_to_json(app: &mut App, items: Vec<PlayHistory>) {
    let json_data = json!(items);

    let project_dir = get_project_dir(&app.file_name);
    let mut path = project_dir.join("spotify_cache");
    std::fs::create_dir_all(&path).unwrap();
    path = path.join("recently_played.json");

    let mut file = File::create(&path).unwrap();
    let _ = file.write_all(json_data.to_string().as_bytes());
}

/// Processes the recently played tracks data stored in the cache file and populates the app's data structures
pub fn process_recently_played(app: &mut App) {
    // Clear any existing recently played track data in the app before processing
    app.recently_played_links.clear();
    app.recently_played_names.clear();
    app.recently_played_duration.clear();
    app.recently_played_artist_names.clear();
    app.recently_played_album_names.clear();

    let project_dir = get_project_dir(&app.file_name);
    let mut path = project_dir.join("spotify_cache");
    path = path.join("recently_played.json");

    let file = File::open(&path).expect("Failed to open recently_played.json");
    let reader = BufReader::new(file);
    let json_data: Value =
        serde_json::from_reader(reader).expect("Failed to parse recently_played.json");

    // Extract information about each recently played track from the JSON data
    if let Value::Array(tracks) = json_data {
        for track in tracks {
            if let Value::Object(track_obj) = track {
                if let Some(track_info) = track_obj.get("track").and_then(Value::as_object) {
                    if let Some(track_name) = track_info.get("name").and_then(Value::as_str) {
                        app.recently_played_names.push(track_name.to_string());
                    }

                    if let Some(track_duration) =
                        track_info.get("duration_ms").and_then(Value::as_u64)
                    {
                        app.recently_played_duration.push(track_duration as i64);
                    }

                    if let Some(artists) = track_info.get("artists").and_then(Value::as_array) {
                        if let Some(first_artist) = artists.first().and_then(Value::as_object) {
                            if let Some(artist_name) =
                                first_artist.get("name").and_then(Value::as_str)
                            {
                                app.recently_played_artist_names
                                    .push(artist_name.to_string());
                            }
                        }
                    }
                    if let Some(albums) = track_info.get("album").and_then(Value::as_object) {
                        if let Some(album_name) = albums.get("name").and_then(Value::as_str) {
                            app.recently_played_album_names.push(album_name.to_string());
                        }
                    }
                    if let Some(external_urls) =
                        track_info.get("external_urls").and_then(Value::as_object)
                    {
                        if let Some(track_link) =
                            external_urls.get("spotify").and_then(Value::as_str)
                        {
                            app.recently_played_links.push(track_link.to_string());
                        }
                    }
                }
            }
        }
    }
}
