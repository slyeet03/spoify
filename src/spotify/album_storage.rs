use serde::{Deserialize, Serialize};

use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct AlbumResponse {
    albums: Albums,
}

#[derive(Serialize, Deserialize, Debug)]
struct Albums {
    items: Vec<Album>,
    // Add other fields from the JSON if needed
}

#[derive(Serialize, Deserialize, Debug)]
struct Album {
    artists: Vec<Artist>,
    href: String,
    id: String,
    available_markets: Vec<String>,
    images: Vec<Image>,
    name: String,
    external_urls: ExternalUrls,
    album_type: String,
    release_date: String,
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

pub fn album_storage() -> Result<(Vec<String>, Vec<String>), std::io::Error> {
    let json_dir = Path::new("./data");
    let json_file_path = json_dir.join("album_search_results.json");

    if !json_file_path.exists() {
        println!("album_search_results.json file does not exist");
        return Ok((Vec::new(), Vec::new()));
    }

    let metadata = fs::metadata(&json_file_path)?;
    if metadata.len() == 0 {
        println!("album_search_results.json file is empty");
        return Ok((Vec::new(), Vec::new()));
    }

    let data = fs::read_to_string(json_file_path)?;

    // Parse the JSON data into an AlbumResponse struct
    let album_response: AlbumResponse = serde_json::from_str(&data).map_err(|e| {
        println!("Deserialization error: {}", e);
        e
    })?;

    let albums = &album_response.albums.items;

    // Create lists to store album names and links
    let mut album_names: Vec<String> = Vec::new();
    let mut album_links: Vec<String> = Vec::new();

    for album in albums {
        album_names.push(album.name.clone());
        album_links.push(album.external_urls.spotify.clone());
    }

    for album in albums {
        album_names.push(album.name.clone());
        album_links.push(album.external_urls.spotify.clone());
    }

    Ok((album_names, album_links))
}
