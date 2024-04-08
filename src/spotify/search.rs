/*
fetches the search results
stores the search results in json files
reads the json file and store the data in required variables
*/

extern crate rspotify;
extern crate serde_json;

use dotenvy::dotenv;
use ratatui::widgets::ListItem;

use std::env;
use std::fs::File;
use std::io::Write;

use log::info;
use rspotify::{
    model::{Country, Market, SearchType},
    prelude::*,
    ClientCredsSpotify, Credentials,
};
use serde::{Deserialize, Serialize};

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[tokio::main]
pub async fn search(user_query: &str) -> Result<(), std::io::Error> {
    dotenv().expect(".env file not found");

    let client_id = env::var("CLIENT_ID").expect("You've not set the CLIENT_ID");
    let client_secret_id =
        env::var("CLIENT_SECRET_ID").expect("You've not set the CLIENT_SECRET_ID");

    let creds = Credentials {
        id: client_id.to_string(),
        secret: Some(client_secret_id.to_string()),
    };
    let spotify = ClientCredsSpotify::new(creds);

    // Obtaining the access token
    spotify.request_token().await.unwrap();

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("..");
    path.push("spoify-tui");
    path.push("spotify_cache");
    std::fs::create_dir_all(&path).unwrap();

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

pub struct SearchResults {
    pub album_names: Vec<String>,
    pub album_links: Vec<String>,
    pub track_names: Vec<String>,
    pub track_links: Vec<String>,
    pub playlist_names: Vec<String>,
    pub playlist_links: Vec<String>,
    pub artist_names: Vec<String>,
    pub artist_links: Vec<String>,
}

pub fn perform_search(query: &str) -> SearchResults {
    let mut spotify_cache_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    spotify_cache_path.push("..");
    spotify_cache_path.push("spoify-tui");
    spotify_cache_path.push("spotify_cache");

    let (
        album_names,
        album_links,
        track_names,
        track_links,
        playlist_names,
        playlist_links,
        artist_names,
        artist_links,
    ) = query_storage(query).unwrap_or_default();

    SearchResults {
        album_names,
        album_links,
        track_names,
        track_links,
        playlist_names,
        playlist_links,
        artist_names,
        artist_links,
    }
}

pub fn search_results_album<'a>(album_names: &'a [String]) -> Vec<ListItem<'a>> {
    let mut search_results = Vec::new();

    for name in album_names {
        search_results.push(ListItem::new(format!("{}", name)));
    }
    search_results
}
pub fn search_results_songs<'a>(track_names: &'a [String]) -> Vec<ListItem<'a>> {
    let mut search_results = Vec::new();

    for name in track_names {
        search_results.push(ListItem::new(format!("{}", name)));
    }

    search_results
}
pub fn search_results_playlist<'a>(playlist_names: &'a [String]) -> Vec<ListItem<'a>> {
    let mut search_results = Vec::new();

    for name in playlist_names {
        search_results.push(ListItem::new(format!("{}", name)));
    }

    search_results
}
pub fn search_results_artist<'a>(artist_names: &'a [String]) -> Vec<ListItem<'a>> {
    let mut search_results = Vec::new();

    for name in artist_names {
        search_results.push(ListItem::new(format!("{}", name)));
    }

    search_results
}
pub fn user_playlist<'a>(user_playlist_names: &'a [String]) -> Vec<ListItem<'a>> {
    let mut search_results = Vec::new();

    for name in user_playlist_names {
        search_results.push(ListItem::new(format!("{}", name)));
    }

    search_results
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
        info!("tracks_search_results.json file does not exist");
        return Ok((Vec::new(), Vec::new()));
    }

    let metadata = fs::metadata(&json_file_path)?;
    if metadata.len() == 0 {
        info!("tracks_search_results.json file is empty");
        return Ok((Vec::new(), Vec::new()));
    }

    let data = fs::read_to_string(json_file_path)?;

    let track_response: TrackResponse = serde_json::from_str(&data).map_err(|e| {
        info!("Deserialization error: {}", e);
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
        info!("playlist_search_results.json file does not exist");
        return Ok((Vec::new(), Vec::new()));
    }

    let metadata = fs::metadata(&json_file_path)?;
    if metadata.len() == 0 {
        info!("playlist_search_results.json file is empty");
        return Ok((Vec::new(), Vec::new()));
    }

    let data = fs::read_to_string(json_file_path)?;

    let playlist_response: PlaylistResponse = serde_json::from_str(&data).map_err(|e| {
        info!("Deserialization error: {}", e);
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
        info!("artist_search_results.json file does not exist");
        return Ok((Vec::new(), Vec::new()));
    }

    let metadata = fs::metadata(&json_file_path)?;
    if metadata.len() == 0 {
        info!("artist_search_results.json file is empty");
        return Ok((Vec::new(), Vec::new()));
    }

    let data = fs::read_to_string(json_file_path)?;

    let artist_response: ArtistResponse = serde_json::from_str(&data).map_err(|e| {
        info!("Deserialization error: {}", e);
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
        info!("album_search_results.json file does not exist");
        return Ok((Vec::new(), Vec::new()));
    }

    let metadata = fs::metadata(&json_file_path)?;
    if metadata.len() == 0 {
        info!("album_search_results.json file is empty");
        return Ok((Vec::new(), Vec::new()));
    }

    let data = fs::read_to_string(json_file_path)?;

    let album_response: AlbumResponse = serde_json::from_str(&data).map_err(|e| {
        info!("Deserialization error: {}", e);
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

pub fn query_storage(
    query: &str,
) -> Result<
    (
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
    ),
    io::Error,
> {
    let mut spotify_cache_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    spotify_cache_path.push("..");
    spotify_cache_path.push("spoify-tui");
    spotify_cache_path.push("spotify_cache");

    if search(query).is_ok() {
        let (album_names, album_links) = match album_storage(&spotify_cache_path) {
            Ok(result) => result,
            Err(err) => {
                info!("Error reading album data: {}", err);
                return Err(err);
            }
        };

        let (track_names, track_links) = match track_storage(&spotify_cache_path) {
            Ok(result) => result,
            Err(err) => {
                info!("Error reading track data: {}", err);
                return Err(err);
            }
        };

        let (artist_names, artist_links) = match artist_storage(&spotify_cache_path) {
            Ok(result) => result,
            Err(err) => {
                info!("Error reading artist data: {}", err);
                return Err(err);
            }
        };

        let (playlist_names, playlist_links) = match playlist_storage(&spotify_cache_path) {
            Ok(result) => result,
            Err(err) => {
                info!("Error reading playlist data: {}", err);
                return Err(err);
            }
        };

        return Ok((
            album_names,
            album_links,
            track_names,
            track_links,
            playlist_names,
            playlist_links,
            artist_names,
            artist_links,
        ));
    }

    Err(io::Error::new(io::ErrorKind::Other, "Search failed"))
}
