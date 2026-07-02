extern crate rspotify;

use rspotify::prelude::OAuthClient;
use rspotify::{scopes, AuthCodeSpotify, ClientError, Credentials, OAuth};
use std::env;
use std::fs;
use std::io::stdin;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use url::Url;
use webbrowser;

use crate::app::App;

// Function to get the Spotify client, either from a cached token or through the authorization flow
pub async fn get_spotify_client(app: &mut App) -> Result<AuthCodeSpotify, ClientError> {
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
        "ugc-image-upload",
        "app-remote-control",
        "streaming"
    );

    let mut oauth = OAuth::default();
    oauth.scopes = scopes;
    oauth.redirect_uri = "http://127.0.0.1:8888/callback".to_owned();

    let creds = Credentials::new(client_id, client_secret_id);

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push(app.file_name.clone());
    path.push("spotify_cache");

    fs::create_dir_all(&path).unwrap();

    let config = rspotify::Config {
        token_cached: true,
        token_refreshing: true,
        cache_path: path.join(".spotify_token_cache.json"),
        ..Default::default()
    };

    let mut spotify = AuthCodeSpotify::with_config(creds, oauth, config);

    match spotify.read_token_cache(true).await {
        Ok(Some(token)) => {
            *spotify.token.lock().await.unwrap() = Some(token);
        }
        Ok(None) => {
            // No cached token found, enter the authorization flow
            handle_authorization_flow(&mut spotify).await?;
        }
        Err(e) => {
            println!("Failed to read token cache: {}", e);
            // Handle the error, e.g., by entering the authorization flow
            handle_authorization_flow(&mut spotify).await?;
        }
    }

    Ok(spotify)
}

// Function to handle the authorization flow with Spotify
async fn handle_authorization_flow(spotify: &mut AuthCodeSpotify) -> Result<(), ClientError> {
    let auth_url = spotify.get_authorize_url(true).unwrap();

    if webbrowser::open(&auth_url).is_err() {
        println!(
            "Failed to open the authorization URL. Please visit the URL manually: {}",
            auth_url
        );
    }

    println!("Waiting for Spotify to redirect to http://127.0.0.1:8888/callback ...");

    let listener = TcpListener::bind("127.0.0.1:8888")
        .expect("Could not bind 127.0.0.1:8888 — is another spoify instance already running?");
    let (mut stream, _) = listener.accept().expect("Failed to accept connection");

    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();
    reader
        .read_line(&mut request_line)
        .expect("Failed to read request");

    // request_line looks like: "GET /callback?code=...&state=... HTTP/1.1"
    let path = request_line.split_whitespace().nth(1).unwrap_or("");
    let full_url = format!("http://127.0.0.1:8888{}", path);
    let url = Url::parse(&full_url).expect("Failed to parse redirected URL");

    let mut code = String::new();
    for (key, value) in url.query_pairs() {
        if key == "code" {
            code = value.to_string();
        }
    }

    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n\
        <html><body><h2>spoify authenticated you can close this tab.</h2></body></html>";
    let _ = stream.write_all(response.as_bytes());

    spotify.request_token(code.trim()).await?;

    Ok(())
}
