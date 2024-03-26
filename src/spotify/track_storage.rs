use serde::{Deserialize, Serialize};

use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct TrackResponse {
    tracks: Tracks,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tracks {
    items: Vec<Track>,
    // Add other fields from the JSON if needed
}

#[derive(Serialize, Deserialize, Debug)]
struct Track {
    artists: Vec<Artist>,
    href: String,
    id: String,
    name: String,
    external_urls: ExternalUrls,
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

pub fn track_storage() -> Result<(), std::io::Error> {
    let json_dir = Path::new("./data");
    let json_file_path = json_dir.join("tracks_search_results.json");

    if !json_file_path.exists() {
        println!("tracks_search_results.json file does not exist");
        return Ok(());
    }

    let metadata = fs::metadata(&json_file_path)?;
    if metadata.len() == 0 {
        println!("tracks_search_results.json file is empty");
        return Ok(());
    }

    let data = fs::read_to_string(json_file_path)?;

    // Parse the JSON data into an TrackResponse struct
    let track_response: TrackResponse = serde_json::from_str(&data).map_err(|e| {
        println!("Deserialization error: {}", e);
        e
    })?;

    let tracks = &track_response.tracks.items;

    // Create lists to store track names and links
    let mut track_names: Vec<String> = Vec::new();
    let mut track_links: Vec<String> = Vec::new();

    for track in tracks {
        track_names.push(track.name.clone());
        track_links.push(track.external_urls.spotify.clone());
    }

    println!("Track names:");
    for name in &track_names {
        println!("{}", name);
    }

    println!("Track links:");
    for link in &track_links {
        println!("{}", link);
    }

    Ok(())
}
