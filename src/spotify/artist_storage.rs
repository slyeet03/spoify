use serde::{Deserialize, Serialize};

use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct ArtistResponse {
    artists: Artists,
}

#[derive(Serialize, Deserialize, Debug)]
struct Artists {
    items: Vec<Artist>,
    // Add other fields from the JSON if needed
}

#[derive(Serialize, Deserialize, Debug)]
struct Artist {
    id: String,
    href: String,
    external_urls: ExternalUrls,
    name: String,
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

pub fn artist_storage() -> Result<(Vec<String>, Vec<String>), std::io::Error> {
    let json_dir = Path::new("./data");
    let json_file_path = json_dir.join("artist_search_results.json");

    if !json_file_path.exists() {
        println!("artist_search_results.json file does not exist");
        return Ok((Vec::new(), Vec::new()));
    }

    let metadata = fs::metadata(&json_file_path)?;
    if metadata.len() == 0 {
        println!("artist_search_results.json file is empty");
        return Ok((Vec::new(), Vec::new()));
    }

    let data = fs::read_to_string(json_file_path)?;

    // Parse the JSON data into an ArtistResponse struct
    let artist_response: ArtistResponse = serde_json::from_str(&data).map_err(|e| {
        println!("Deserialization error: {}", e);
        e
    })?;

    let artists = &artist_response.artists.items;

    // Create lists to store artist names and links
    let mut artist_names: Vec<String> = Vec::new();
    let mut artist_links: Vec<String> = Vec::new();

    for artist in artists {
        artist_names.push(artist.name.clone());
        artist_links.push(artist.external_urls.spotify.clone());
    }

    Ok((artist_names, artist_links))
}
