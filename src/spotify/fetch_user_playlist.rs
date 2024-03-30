use dotenv::dotenv;
use env_logger::{Builder, Env};
use futures_util::StreamExt;
use futures_util::TryStreamExt;
use log::LevelFilter;
use rspotify::{
    clients::OAuthClient, model::SimplifiedPlaylist, scopes, AuthCodeSpotify, ClientCredsSpotify,
    ClientError, Config, Credentials, OAuth,
};
use std::env;
use std::fs::File;
use std::io::{stdin, Read};

use reqwest::Client;
use webbrowser;

async fn fetch_user_playlists() -> Result<(), ClientError> {
    dotenv().ok();
    let client_id = env::var("CLIENT_ID").expect("You've not set the CLIENT_ID");
    let client_secret_id =
        env::var("CLIENT_SECRET_ID").expect("You've not set the CLIENT_SECRET_ID");

    // Using every possible scope
    let scopes = scopes!(
        "user-read-private",
        "user-read-email",
        "playlist-read-collaborative",
        "playlist-read-private",
        "playlist-modify-public",
        "playlist-modify-private"
    );

    let mut oauth = OAuth::default();
    oauth.scopes = scopes;
    oauth.redirect_uri = "http://localhost:8888/callback".to_owned();

    let creds = Credentials::new(&client_id, &client_secret_id);
    let config = Config::default();
    let spotify = AuthCodeSpotify::with_config(creds, oauth, config);

    // Get the URL to request user authorization
    let auth_url = spotify.get_authorize_url(true).unwrap();

    // Open the authorization URL in the default browser
    if webbrowser::open(&auth_url).is_err() {
        println!(
            "Failed to open the authorization URL. Please visit the URL manually: {}",
            auth_url
        );
    }
    // Get the authorization code from the redirected URL
    let client = Client::new();
    let mut code = String::new();
    loop {
        let res = client.get(&auth_url).send().await.unwrap();
        let redirect_url = res.url().as_str();
        if redirect_url.contains("http://localhost:8888/callback?code=") {
            code = redirect_url
                .split("code=")
                .nth(1)
                .unwrap()
                .split("&")
                .next()
                .unwrap()
                .to_string();
            break;
        }
    }
    log::info!("Authorization code: {}", code); // Log the authorization code
                                                /*
                                                    // Initialize logging to a file named "spotify.log"
                                                    let log_file = File::create("spotify.log").unwrap();
                                                    let env = Env::default().default_filter_or("info");
                                                    let mut builder = Builder::from_env(env);
                                                    builder.target(env_logger::Target::Pipe(Box::new(log_file)));
                                                    builder.init();

                                                    println!("Open this URL in your browser: {}", auth_url);
                                                    log::info!("Authorization URL: {}", auth_url); // Log the URL

                                                    println!("After authorizing the application, enter the code:");


                                                    let mut code = String::new();
                                                    stdin().read_line(&mut code).unwrap();
                                                */
    // Request user token
    let token = spotify.request_token(&code.trim()).await;
    log::info!("Requesting user token..."); // Log token request

    match token {
        Ok(token) => {
            // Get the user's playlists
            let mut playlists = Vec::new();
            let mut stream = spotify.current_user_playlists();
            while let Ok(result) = stream.try_next().await {
                match result {
                    Some(playlist) => playlists.push(playlist),
                    None => break, // End of stream
                }
            }

            // Print the playlists
            for playlist in playlists {
                println!("Playlist: {}", playlist.name);
            }
        }
        Err(err) => {
            println!("Error requesting token: {:#?}", err);
        }
    }

    Ok(())
}

pub fn spo() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let _ = rt.block_on(fetch_user_playlists());
}
