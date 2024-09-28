use crate::app::App;
use crate::spotify::auth::get_spotify_client;
use crate::util::get_project_dir;
use futures::FutureExt;
use futures_util::TryStreamExt;
use rspotify::model::Show;
use rspotify::prelude::OAuthClient;
use rspotify::ClientError;
use serde_json::{json, Value};
use std::fs::File;
use std::io::{BufReader, Write};

/// Fetches a user's saved podcasts from Spotify
#[tokio::main]
pub async fn user_podcast(app: &mut App) -> Result<(), ClientError> {
    // Get a Spotify client using an existing access token (if available).
    let spotify = get_spotify_client(app).await?;

    // Collect all the user's saved podcasts from Spotify
    let mut podcasts = Vec::new();
    // Executing the futures sequentially
    let stream = spotify
        .get_saved_show()
        .try_for_each(|item| {
            podcasts.push(item);
            futures::future::ok(())
        })
        .boxed();

    stream.await?;

    save_podcasts_to_json(app, podcasts);

    Ok(())
}

/// Saves a vector of saved podcasts to a JSON file in the Spotify cache directory
fn save_podcasts_to_json(app: &mut App, podcasts: Vec<Show>) {
    let json_data = json!(podcasts);

    let project_dir = get_project_dir(&app.file_name);
    let mut path = project_dir.join("spotify_cache");
    std::fs::create_dir_all(&path).unwrap();
    path = path.join("podcasts.json");

    let mut file = File::create(&path).unwrap();
    let _ = file.write_all(json_data.to_string().as_bytes());
}

/// Processes the saved podcasts data stored in the cache file and populates the app's data structures
pub fn process_podcasts(app: &mut App) {
    // Clear any existing podcast data in the app before processing
    app.podcast_names.clear();
    app.podcast_links.clear();
    app.podcast_publisher.clear();

    let project_dir = get_project_dir(&app.file_name);
    let mut path = project_dir.join("spotify_cache");
    path = path.join("podcasts.json");

    let file = File::open(&path).expect("Failed to open podcasts.json");
    let reader = BufReader::new(file);
    let json_data: Value = serde_json::from_reader(reader).expect("Failed to parse podcasts.json");

    // Extract information about each saved podcast from the JSON data
    if let Value::Array(shows) = json_data {
        for show in shows {
            if let Value::Object(show_obj) = show {
                if let Some(show_info) = show_obj.get("show").and_then(Value::as_object) {
                    if let Some(show_name) = show_info.get("name").and_then(Value::as_str) {
                        app.podcast_names.push(show_name.to_string());
                    }

                    if let Some(external_urls) =
                        show_info.get("external_urls").and_then(Value::as_object)
                    {
                        if let Some(show_link) =
                            external_urls.get("spotify").and_then(Value::as_str)
                        {
                            app.podcast_links.push(show_link.to_string());
                        }
                    }
                    if let Some(show_publisher) = show_info.get("publisher").and_then(Value::as_str)
                    {
                        app.podcast_publisher.push(show_publisher.to_string());
                    }
                }
            }
        }
    }
}
