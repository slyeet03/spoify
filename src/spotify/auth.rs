extern crate rspotify;
use chrono::Utc;
use dotenvy::dotenv;
use log::info;
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

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotifyClient {
    pub token: Option<Token>,
}

pub async fn get_spotify_client() -> Result<SpotifyClient, ClientError> {
    dotenv().expect(".env file not found");
    let client_id = env::var("CLIENT_ID").expect("You've not set the CLIENT_ID");
    let client_secret_id =
        env::var("CLIENT_SECRET_ID").expect("You've not set the CLIENT_SECRET_ID");

    // Using every possible scope
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

    let client_from_file = read_client_from_file().await;

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

async fn handle_authorization_flow(
    spotify: &mut AuthCodeSpotify,
) -> Result<SpotifyClient, ClientError> {
    let auth_url = spotify.get_authorize_url(true).unwrap();

    if webbrowser::open(&auth_url).is_err() {
        println!(
            "Failed to open the authorization URL. Please visit the URL manually: {}",
            auth_url
        );
    }

    println!("Enter redirected url:");
    let mut url_input = String::new();
    stdin().read_line(&mut url_input).unwrap();
    let url_string = &url_input.as_str();

    let url = Url::parse(url_string).expect("Failed to parse URL");
    let query_pairs = url.query_pairs();

    let mut code = String::new();
    let mut state = String::new();
    for (key, value) in query_pairs {
        if key == "code" {
            code = value.to_string();
        } else if key == "state" {
            state = value.to_string();
        }
    }

    spotify.request_token(&code.trim()).await?;
    let token = spotify.token.lock().await.unwrap().clone();
    let spotify_client = SpotifyClient { token: token };
    write_client_to_file(&spotify_client).await;

    Ok(spotify_client)
}

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

async fn write_client_to_file(client: &SpotifyClient) {
    let json_data = serde_json::to_vec_pretty(client).unwrap();

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("spotify_cache");

    std::fs::create_dir_all(&path).unwrap();
    path.push("client.json");

    let mut file = File::create(&path).unwrap();
    file.write_all(&json_data).unwrap();

    info!("Client saved to {}", path.display());
}
