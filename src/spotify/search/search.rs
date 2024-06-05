/*
fetches the search results
stores the search results in json files
reads the json file and store the data in required variables
*/

extern crate rspotify;
extern crate serde_json;

use rspotify::{
    model::{Country, Market, SearchType},
    prelude::*,
    ClientCredsSpotify, Credentials,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::app::App;

// Main function to perform the search and store the results in JSON files
#[tokio::main]
pub async fn search(user_query: &str, app: &mut App) -> Result<(), std::io::Error> {
    let client_id = &app.client_id;
    let client_secret_id = &app.client_secret;

    let creds = Credentials {
        id: client_id.to_string(),
        secret: Some(client_secret_id.to_string()),
    };
    let spotify = ClientCredsSpotify::new(creds);

    // Obtaining the access token
    spotify.request_token().await.unwrap();

    // Setting up the directory to store the search results
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("..");
    path.push(app.file_name.clone());
    path.push("spotify_cache");
    std::fs::create_dir_all(&path).unwrap();

    // Performing search and storing the results in JSON files
    let album_query = user_query;
    let result = spotify
        .search(album_query, SearchType::Album, None, None, Some(10), None)
        .await;
    if let Ok(albums) = result {
        let json_data = serde_json::to_string(&albums).unwrap();
        let mut file = File::create(path.join("album_search_results.json")).unwrap();
        write!(file, "{}", json_data).unwrap();
    }

    let artist_query = user_query;
    let result = spotify
        .search(
            artist_query,
            SearchType::Artist,
            Some(Market::Country(Country::UnitedStates)),
            None,
            Some(10),
            None,
        )
        .await;
    if let Ok(artists) = result {
        let json_data = serde_json::to_string(&artists).unwrap();
        let mut file = File::create(path.join("artist_search_results.json")).unwrap();
        write!(file, "{}", json_data).unwrap();
    }

    let formated_query = format!("\"{}\"", user_query);
    let playlist_query: &str = formated_query.as_str();
    let result = spotify
        .search(
            playlist_query,
            SearchType::Playlist,
            Some(Market::Country(Country::UnitedStates)),
            None,
            Some(10),
            None,
        )
        .await;
    if let Ok(playlists) = result {
        let json_data = serde_json::to_string(&playlists).unwrap();
        let mut file = File::create(path.join("playlist_search_results.json")).unwrap();
        write!(file, "{}", json_data).unwrap();
    }

    let track_query = user_query;
    let result = spotify
        .search(
            track_query,
            SearchType::Track,
            Some(Market::Country(Country::UnitedStates)),
            None,
            Some(10),
            None,
        )
        .await;
    if let Ok(tracks) = result {
        let json_data = serde_json::to_string(&tracks).unwrap();
        let mut file = File::create(path.join("tracks_search_results.json")).unwrap();
        write!(file, "{}", json_data).unwrap();
    }

    Ok(())
}

// Function to process the search results and store them in the application state
pub fn process_search(app: &mut App, query: &str) -> io::Result<()> {
    app.album_names_search_results.clear();
    app.artist_names_search_results.clear();
    app.track_names_search_results.clear();
    app.playlist_names_search_results.clear();

    app.album_links_search_results.clear();
    app.artist_links_search_results.clear();
    app.track_links_search_results.clear();
    app.playlist_links_search_results.clear();

    let mut spotify_cache_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    spotify_cache_path.push("..");
    spotify_cache_path.push(app.file_name.clone());
    spotify_cache_path.push("spotify_cache");

    if search(query, app).is_ok() {
        (
            app.album_names_search_results,
            app.album_links_search_results,
        ) = match album_storage(&spotify_cache_path) {
            Ok(result) => result,
            Err(err) => {
                println!("Error reading album data: {}", err);
                return Err(err);
            }
        };

        (
            app.track_names_search_results,
            app.track_links_search_results,
        ) = match track_storage(&spotify_cache_path) {
            Ok(result) => result,
            Err(err) => {
                println!("Error reading track data: {}", err);
                return Err(err);
            }
        };

        (
            app.artist_names_search_results,
            app.artist_links_search_results,
        ) = match artist_storage(&spotify_cache_path) {
            Ok(result) => result,
            Err(err) => {
                println!("Error reading artist data: {}", err);
                return Err(err);
            }
        };

        (
            app.playlist_names_search_results,
            app.playlist_links_search_results,
        ) = match playlist_storage(&spotify_cache_path) {
            Ok(result) => result,
            Err(err) => {
                println!("Error reading playlist data: {}", err);
                return Err(err);
            }
        };
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct TrackResponse {
    tracks: Tracks,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tracks {
    items: Vec<Track>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Track {
    artists: Vec<Artist>,
    href: String,
    id: String,
    name: String,
    external_urls: TrackExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct TrackExternalUrls {
    spotify: String,
}

pub fn track_storage(spotify_cache_path: &Path) -> Result<(Vec<String>, Vec<String>), io::Error> {
    let json_file_path = spotify_cache_path.join("tracks_search_results.json");

    if !json_file_path.exists() {
        println!("tracks_search_results.json file does not exist");
        return Ok((Vec::new(), Vec::new()));
    }

    let metadata = fs::metadata(&json_file_path)?;
    if metadata.len() == 0 {
        println!("tracks_search_results.json file is empty");
        return Ok((Vec::new(), Vec::new()));
    }

    let data = fs::read_to_string(json_file_path)?;

    let track_response: TrackResponse = serde_json::from_str(&data).map_err(|e| {
        println!("Deserialization error: {}", e);
        e
    })?;

    let tracks = &track_response.tracks.items;

    let mut track_names_search_results: Vec<String> = Vec::new();
    let mut track_links_search_results: Vec<String> = Vec::new();

    for track in tracks {
        track_names_search_results.push(track.name.clone());
        track_links_search_results.push(track.id.clone());
    }

    Ok((track_names_search_results, track_links_search_results))
}

#[derive(Serialize, Deserialize, Debug)]
struct PlaylistResponse {
    playlists: Playlists,
}

#[derive(Serialize, Deserialize, Debug)]
struct Playlists {
    items: Vec<Playlist>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Playlist {
    href: String,
    id: String,
    name: String,
    external_urls: PlaylistExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct PlaylistExternalUrls {
    spotify: String,
}

pub fn playlist_storage(
    spotify_cache_path: &Path,
) -> Result<(Vec<String>, Vec<String>), io::Error> {
    let json_file_path = spotify_cache_path.join("playlist_search_results.json");

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

    let playlist_response: PlaylistResponse = serde_json::from_str(&data).map_err(|e| {
        println!("Deserialization error: {}", e);
        e
    })?;

    let playlists = &playlist_response.playlists.items;

    let mut playlist_names_search_results: Vec<String> = Vec::new();
    let mut playlist_links_search_results: Vec<String> = Vec::new();

    for playlist in playlists {
        playlist_names_search_results.push(playlist.name.clone());
        playlist_links_search_results.push(playlist.id.clone());
    }

    Ok((playlist_names_search_results, playlist_links_search_results))
}

#[derive(Serialize, Deserialize, Debug)]
struct ArtistResponse {
    artists: Artists,
}

#[derive(Serialize, Deserialize, Debug)]
struct Artists {
    items: Vec<Artist>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Artist {
    id: String,
    href: String,
    external_urls: ArtistExternalUrls,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ArtistExternalUrls {
    spotify: String,
}

pub fn artist_storage(spotify_cache_path: &Path) -> Result<(Vec<String>, Vec<String>), io::Error> {
    let json_file_path = spotify_cache_path.join("artist_search_results.json");

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

    let artist_response: ArtistResponse = serde_json::from_str(&data).map_err(|e| {
        println!("Deserialization error: {}", e);
        e
    })?;

    let artists = &artist_response.artists.items;

    let mut artist_names_search_results: Vec<String> = Vec::new();
    let mut artist_links_search_results: Vec<String> = Vec::new();

    for artist in artists {
        artist_names_search_results.push(artist.name.clone());
        artist_links_search_results.push(artist.id.clone());
    }

    Ok((artist_names_search_results, artist_links_search_results))
}

#[derive(Serialize, Deserialize, Debug)]
struct AlbumResponse {
    albums: Albums,
}

#[derive(Serialize, Deserialize, Debug)]
struct Albums {
    items: Vec<Album>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Album {
    artists: Vec<Artist>,
    href: String,
    id: String,
    available_markets: Vec<String>,
    name: String,
    external_urls: AlbumExternalUrls,
    album_type: String,
    release_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AlbumExternalUrls {
    spotify: String,
}

pub fn album_storage(spotify_cache_path: &Path) -> Result<(Vec<String>, Vec<String>), io::Error> {
    let json_file_path = spotify_cache_path.join("album_search_results.json");

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

    let album_response: AlbumResponse = serde_json::from_str(&data).map_err(|e| {
        println!("Deserialization error: {}", e);
        e
    })?;

    let albums = &album_response.albums.items;

    let mut album_names_search_results: Vec<String> = Vec::new();
    let mut album_links_search_results: Vec<String> = Vec::new();

    for album in albums {
        album_names_search_results.push(album.name.clone());
        album_links_search_results.push(album.id.clone());
    }

    Ok((album_names_search_results, album_links_search_results))
}
