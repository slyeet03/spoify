use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

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

    let mut track_names: Vec<String> = Vec::new();
    let mut track_links: Vec<String> = Vec::new();

    for track in tracks {
        track_names.push(track.name.clone());
        track_links.push(track.external_urls.spotify.clone());
    }

    Ok((track_names, track_links))
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

    let mut playlist_names: Vec<String> = Vec::new();
    let mut playlist_links: Vec<String> = Vec::new();

    for playlist in playlists {
        playlist_names.push(playlist.name.clone());
        playlist_links.push(playlist.external_urls.spotify.clone());
    }

    Ok((playlist_names, playlist_links))
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

    let mut artist_names: Vec<String> = Vec::new();
    let mut artist_links: Vec<String> = Vec::new();

    for artist in artists {
        artist_names.push(artist.name.clone());
        artist_links.push(artist.external_urls.spotify.clone());
    }

    Ok((artist_names, artist_links))
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

    let mut album_names: Vec<String> = Vec::new();
    let mut album_links: Vec<String> = Vec::new();

    for album in albums {
        album_names.push(album.name.clone());
        album_links.push(album.external_urls.spotify.clone());
    }

    Ok((album_names, album_links))
}
