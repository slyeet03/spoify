extern crate rspotify;

use chrono::Utc;
use rspotify::clients::BaseClient;
use rspotify::prelude::OAuthClient;
use rspotify::{scopes, AuthCodeSpotify, ClientError, Credentials, OAuth, Token};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::{stdin, Read, Write};
use std::path::PathBuf;
use url::Url;
use webbrowser;

use crate::app::App;

// Defining a struct to hold the Spotify client and its token
#[derive(Debug, Serialize, Deserialize)]
pub struct SpotifyClient {
    pub token: Option<Token>,
}

// Function to get the Spotify client, either from a cached token or through the authorization flow
pub async fn get_spotify_client(app: &mut App) -> Result<SpotifyClient, ClientError> {
    let client_id = &app.client_id;
    let client_secret_id = &app.client_secret;

    // Defining the scopes (permissions) required for the application
    let scopes = scopes!(
        "user-read-email",
        "user-read-private",
        "user-top-read",
        "user-read-recently-played",
        "user-follow-read",
        "user-library-read",
        "user-read-currently-playing",
        "user-read-playback-state",
        "user-read-playback-position",
        "playlist-read-collaborative",
        "playlist-read-private",
        "user-follow-modify",
        "user-library-modify",
        "user-modify-playback-state",
        "playlist-modify-public",
        "playlist-modify-private",
        "ugc-image-upload"
    );

    let mut oauth = OAuth::default();
    oauth.scopes = scopes;
    oauth.redirect_uri = "http://localhost:8888/callback".to_owned();

    let creds = Credentials::new(&client_id, &client_secret_id);
    let config = rspotify::Config::default();
    let mut spotify = AuthCodeSpotify::with_config(creds, oauth, config);

    let mut spotify_client = SpotifyClient { token: None };
    let _var = spotify_client;

    let client_from_file = read_client_from_file().await; // Checking if a cached token exists

    if let Some(mut client) = client_from_file {
        let now = Utc::now();
        if let Some(expires_at) = client.token.as_ref().and_then(|t| t.expires_at) {
            if expires_at < now {
                // Token is expired, try to refresh it
                match spotify.refresh_token().await {
                    Ok(_) => {
                        client.token = spotify.token.lock().await.unwrap().clone();
                        write_client_to_file(&client).await;
                    }
                    Err(_) => {
                        // Token refresh failed, enter the authorization flow
                        client = handle_authorization_flow(&mut spotify).await?;
                    }
                }
            }
        }
        spotify_client = client;
    } else {
        // No cached token found, enter the authorization flow
        spotify_client = handle_authorization_flow(&mut spotify).await?;
    }

    Ok(spotify_client)
}

// Function to handle the authorization flow with Spotify
async fn handle_authorization_flow(
    spotify: &mut AuthCodeSpotify,
) -> Result<SpotifyClient, ClientError> {
    let auth_url = spotify.get_authorize_url(true).unwrap(); // Getting the authorization UR

    if webbrowser::open(&auth_url).is_err() {
        // Attempting to open the authorization URL in the default browser
        println!(
            "Failed to open the authorization URL. Please visit the URL manually: {}",
            auth_url
        );
    }

    // Prompting the user to enter the redirected URL after authorization
    println!("Enter redirected url:");
    let mut url_input = String::new();
    stdin().read_line(&mut url_input).unwrap();
    let url_string = &url_input.as_str();

    // Parsing the redirected URL
    let url = Url::parse(url_string).expect("Failed to parse URL");
    let query_pairs = url.query_pairs();

    let mut code = String::new();
    let mut _state = String::new();
    for (key, value) in query_pairs {
        if key == "code" {
            code = value.to_string();
        } else if key == "state" {
            _state = value.to_string();
        }
    }

    // Requesting the access token using the authorization code
    spotify.request_token(&code.trim()).await?;
    let token = spotify.token.lock().await.unwrap().clone();
    let spotify_client = SpotifyClient { token: token };

    // Saving the client and token to a file
    write_client_to_file(&spotify_client).await;

    Ok(spotify_client)
}

// Function to read the cached Spotify client from a file
async fn read_client_from_file() -> Option<SpotifyClient> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("spotify_cache");
    path.push("client.json");

    if path.exists() {
        let mut file = File::open(path).expect("Failed to open client file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)
            .expect("Failed to read client file");
        let client: SpotifyClient = serde_json::from_slice(&contents).unwrap();
        Some(client)
    } else {
        None
    }
}

// Function to write the Spotify client and token to a file
async fn write_client_to_file(client: &SpotifyClient) {
    let json_data = serde_json::to_vec_pretty(client).unwrap();

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("spotify_cache");

    // Creating the directory if it doesn't exist
    std::fs::create_dir_all(&path).unwrap();
    path.push("client.json");

    let mut file = File::create(&path).unwrap();
    file.write_all(&json_data).unwrap();
}
