extern crate rspotify;
extern crate serde_json;

use dotenvy::dotenv;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use rspotify::{
    model::{Country, Market, SearchType},
    prelude::*,
    ClientCredsSpotify, Credentials,
};

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

use crate::spotify::query_storage::query_storage;

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
