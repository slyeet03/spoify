extern crate rspotify;

use dotenv::dotenv;
use std::env;
use std::io;

use rspotify::{prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth};

pub async fn spoi() {
    dotenv().ok();

    let client_id = env::var("CLIENT_ID").expect("You've not set the CLIENT_ID");
    let client_secret_id =
        env::var("CLIENT_SECRET_ID").expect("You've not set the CLIENT_SECRET_ID");

    let creds = Credentials {
        id: client_id.to_string(),
        secret: Some(client_secret_id.to_string()),
    };

    let oauth = OAuth::from_env(scopes!(
        "user-read-playback-state",
        "user-read-currently-playing"
    ))
    .unwrap();
    let spotify = AuthCodeSpotify::new(creds, oauth);

    // Obtaining the access token
    let url = spotify.get_authorize_url(false).unwrap();
    println!(
        "Please open the following URL in your browser and authenticate with Spotify: {}",
        url
    );
    spotify.prompt_for_token(&url).await.unwrap();

    let devices = spotify.device().await;

    match devices {
        Ok(devices) => {
            if devices.is_empty() {
                println!("No devices found for the authenticated user.");
            } else {
                println!("Devices:");
                for device in devices {
                    println!("- {}", device.name);
                }
            }
        }
        Err(err) => {
            println!("Error getting devices: {}", err);
        }
    }
}
