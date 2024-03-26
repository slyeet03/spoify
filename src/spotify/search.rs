extern crate rspotify;
extern crate serde_json;

use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use rspotify::{
    model::{Country, Market, SearchType},
    prelude::*,
    ClientCredsSpotify, Credentials,
};

#[tokio::main]
pub async fn search(user_query: &str, data_dir: &Path) -> Result<(), std::io::Error> {
    dotenv().ok();

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

    let album_query = user_query;
    let result = spotify
        .search(album_query, SearchType::Album, None, None, Some(10), None)
        .await;
    match result {
        Ok(albums) => {
            // Convert album to JSON
            let json_data = serde_json::to_string(&albums).unwrap();

            // Write JSON data to file
            let mut file = File::create(data_dir.join("album_search_results.json")).unwrap();
            write!(file, "{}", json_data).unwrap();
        }
        Err(err) => println!("Search error! {err:?}"),
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
    match result {
        Ok(artists) => {
            // Convert artists to JSON
            let json_data = serde_json::to_string(&artists).unwrap();

            // Write JSON data to file
            let mut file = File::create(data_dir.join("artist_search_results.json")).unwrap();
            write!(file, "{}", json_data).unwrap();
        }
        Err(err) => println!("Search error! {err:?}"),
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
    match result {
        Ok(playlists) => {
            // Convert playlist to JSON
            let json_data = serde_json::to_string(&playlists).unwrap();

            // Write JSON data to file
            let mut file = File::create(data_dir.join("playlist_search_results.json")).unwrap();
            write!(file, "{}", json_data).unwrap();
        }
        Err(err) => println!("Search error! {err:?}"),
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
    match result {
        Ok(tracks) => {
            // Convert tracks to JSON
            let json_data = serde_json::to_string(&tracks).unwrap();

            // Write JSON data to file
            let mut file = File::create(data_dir.join("tracks_search_results.json")).unwrap();
            write!(file, "{}", json_data).unwrap();
        }
        Err(err) => println!("Search error! {err:?}"),
    }

    Ok(())
}
