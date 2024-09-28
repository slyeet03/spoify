extern crate rspotify;

use rspotify::prelude::OAuthClient;
use rspotify::{scopes, AuthCodeSpotify, ClientError, Credentials, OAuth};
use std::fs;
use std::io::stdin;
use url::Url;
use webbrowser;

use crate::app::App;
use crate::util::get_project_dir;

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
        "ugc-image-upload"
    );

    let mut oauth = OAuth::default();
    oauth.scopes = scopes;
    oauth.redirect_uri = "http://localhost:8888/callback".to_owned();

    let creds = Credentials::new(client_id, client_secret_id);

    let project_dir = get_project_dir(&app.file_name);
    let mut path = project_dir.join("spotify_cache");
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
    let auth_url = spotify.get_authorize_url(true).unwrap(); // Getting the authorization URL

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
    spotify.request_token(code.trim()).await?;

    Ok(())
}
