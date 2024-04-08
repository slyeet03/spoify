/*
fetches the tracks of selected playlists
stores their data in json file
store their names,artists,duration and links in their respective variables
*/

use crate::app::App;
use crate::init_logger;
use crate::spotify::auth::{get_spotify_client, SpotifyClient};
use futures::FutureExt;
use futures_util::TryStreamExt;
use log::info;
use regex::Regex;
use rspotify::clients::OAuthClient;
use rspotify::model::{playlist, PlaylistId, PlaylistItem, SimplifiedPlaylist};
use rspotify::prelude::BaseClient;
use rspotify::{AuthCodeSpotify, ClientError};
use serde_json::json;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[tokio::main]
pub async fn fetch_playlists_tracks(app: &mut App) -> Result<(), ClientError> {
    let spotify_client = get_spotify_client().await;
    let spotify = match &spotify_client.unwrap().token {
        Some(token) => AuthCodeSpotify::from_token(token.clone()),
        None => return Err(ClientError::InvalidToken),
    };

    let playlist_url = app.selected_playlist_uri.as_str();
    let re = Regex::new(r"/playlist/(.+)").unwrap();
    let captures = re.captures(playlist_url).unwrap();
    let playlist_uri = captures.get(1).unwrap().as_str();
    let playlist_id = PlaylistId::from_id(playlist_uri).unwrap();

    let mut playlist_items = Vec::new();

    let stream = spotify
        .playlist_items(playlist_id, None, None)
        .try_for_each(|item| {
            playlist_items.push(item);
            futures::future::ok(())
        })
        .boxed();

    stream.await?;

    save_playlists_to_json(playlist_items);

    Ok(())
}

fn save_playlists_to_json(playlist_items: Vec<PlaylistItem>) {
    let json_data = json!(playlist_items);

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("spotify_cache");
    std::fs::create_dir_all(&path).unwrap();
    path.push("playlist_item.json");

    let mut file = File::create(&path).unwrap();
    let _ = file.write_all(json_data.to_string().as_bytes());
}
