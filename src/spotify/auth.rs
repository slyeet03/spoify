use dotenv::dotenv;
use rspotify::{scopes, AuthCodeSpotify, ClientError, Credentials, OAuth, Token};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{stdin, Read, Write};
use std::path::PathBuf;
use url::Url;
use webbrowser;

extern crate rspotify;

use rspotify::prelude::OAuthClient;

const TOKEN_FILE: &str = "token.txt";

#[derive(Debug)]
pub struct SpotifyClient {
    pub spotify: AuthCodeSpotify,
    pub token: Option<Token>,
}

pub async fn get_spotify_client() -> Result<SpotifyClient, ClientError> {
    dotenv().ok();
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
    let spotify = AuthCodeSpotify::with_config(creds, oauth, config);

    let token = read_token_from_file().await;

    let mut spotify_client = SpotifyClient { spotify, token };

    if spotify_client.token.is_none() {
        let auth_url = spotify_client.spotify.get_authorize_url(true).unwrap();

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

        spotify_client.spotify.request_token(&code.trim()).await?;
        spotify_client.token = spotify_client.spotify.token.lock().await.unwrap().clone();
        if let Some(t) = &spotify_client.token {
            write_token_to_file(t).await;
        }
    } else {
        let token = read_token_from_file().await;
        spotify_client.token = token;
    }

    Ok(spotify_client)
}

async fn read_token_from_file() -> Option<Token> {
    let path = PathBuf::from(TOKEN_FILE);
    if path.exists() {
        let mut file = File::open(path).expect("Failed to open token file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read token file");
        let token: Token = serde_json::from_str(&contents).unwrap();
        Some(token)
    } else {
        None
    }
}

async fn write_token_to_file(token: &Token) {
    let path = PathBuf::from(TOKEN_FILE);
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .expect("Failed to open token file");
    let json = serde_json::to_string(token).unwrap();
    file.write_all(json.as_bytes())
        .expect("Failed to write token to file");
}
