use serde::{Deserialize, Serialize};

use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct PlaylistResponse {
    playlists: Playlists,
}

#[derive(Serialize, Deserialize, Debug)]
struct Playlists {
    items: Vec<Playlist>,
    // Add other fields from the JSON if needed
}

#[derive(Serialize, Deserialize, Debug)]
struct Playlist {
    href: String,
    id: String,
    name: String,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExternalUrls {
    spotify: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Image {
    height: u32,
    url: String,
    width: u32,
}

pub fn playlist_storage() -> Result<(Vec<String>, Vec<String>), std::io::Error> {
    let json_dir = Path::new("./data");
    let json_file_path = json_dir.join("playlist_search_results.json");

    if !json_file_path.exists() {
        println!("playlist_search_results.json file does not exist");
        return Ok((Vec::new(), Vec::new()));
    }

    let metadata = fs::metadata(&json_file_path)?;
    if metadata.len() == 0 {
        println!("playlist_search_results.json file is empty");
        return Ok((Vec::new(), Vec::new()));
    }

    let data = fs::read_to_string(json_file_path)?;

    // Parse the JSON data into an PlaylistResponse struct
    let playlist_response: PlaylistResponse = serde_json::from_str(&data).map_err(|e| {
        println!("Deserialization error: {}", e);
        e
    })?;

    let playlists = &playlist_response.playlists.items;

    // Create lists to store playlist names and links
    let mut playlist_names: Vec<String> = Vec::new();
    let mut playlist_links: Vec<String> = Vec::new();

    for playlist in playlists {
        playlist_names.push(playlist.name.clone());
        playlist_links.push(playlist.external_urls.spotify.clone());
    }

    Ok((playlist_names, playlist_links))
}
