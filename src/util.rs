extern crate serde_json;
extern crate serde_yaml;
extern crate yaml_rust;

use crate::app::App;
use crate::settings::keybindings::{parse_keybindings, read_keybindings, set_keybindings};
use crate::settings::theme::{read_theme, set_theme};
use crate::settings::volume::{read_volume_values, set_volume_values};
use crate::spotify::new_release_section::new_releases::{new_releases, process_new_releases};
use crate::spotify::player::player::{currently_playing, process_currently_playing};
use crate::spotify::user_playlist::user_playlist::{get_playlists, process_user_playlists};
use crate::spotify::user_stats::top_tracks::top_tracks;
use crate::structs::Key;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Function to update the player information in a separate thread
pub fn update_player_info(tx: mpsc::Sender<()>, app: &mut App) {
    loop {
        // Get the user's current playback
        currently_playing(app).unwrap();
        process_currently_playing(app);

        // Send a message to the main thread to update the UI
        if tx.send(()).is_err() {
            break;
        }

        // Wait one second before fetching playback again
        thread::sleep(Duration::from_millis(900));
    }
}

/// Function to run before starting the main app loop
pub fn startup(app: &mut App, key: &mut Key) {
    // Set the keybindings from the configure files
    read_keybindings(app);
    set_keybindings(app, key);
    parse_keybindings(app, key);

    // Set the theme from the configure files
    read_theme(app);
    set_theme(app);

    // Set the volume increament and decreament values
    read_volume_values(app);
    set_volume_values(app);

    // Fetch the new released albums from spotify
    let _ = new_releases(app);
    process_new_releases(app);

    // Fetch user playlists from spotify
    get_playlists(app);
    process_user_playlists(app);

    let _ = top_tracks(app);
}

pub fn instruction() {
    println!("
    In order for spoify to work it needs to be connected to Spotify's API.

    Instruction:
        1. Go to the (https://developer.spotify.com/dashboard/applications)
        2. Click 'Create an app'
            - You now can see your 'Client ID' and 'Client Secret'
        3. Now click 'Edit Settings'
        4. Add 'http://localhost:8888/callback' to the Redirect URIs
        5. Scroll down and click 'Save'
        6. You are now ready to authenticate with Spotify!
        7. Enter you 'Client ID' and 'Client Secret'.
        8. Run spoify
        9. You will be redirected to an official Spotify webpage to ask you for permissions.
        10. After accepting the permissions, you'll be redirected to localhost.
            You'll be redirected to a blank webpage that might say something like 'Connection Refused' since no server is running. 
            Regardless, copy the URL and paste into the prompt in the terminal.
    ");
}

pub fn save_creds_to_yml(app: &mut App) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push(app.file_name.clone());
    path.push("configure");
    path.push("creds.yml");

    // Prompt the user for client ID and secret key
    println!("Enter Client ID:");
    let mut client_id = String::new();
    std::io::stdin()
        .read_line(&mut client_id)
        .expect("Failed to read client ID");
    client_id = client_id.trim().to_string();

    println!("Enter Client Secret:");
    let mut client_secret = String::new();
    std::io::stdin()
        .read_line(&mut client_secret)
        .expect("Failed to read client secret");
    client_secret = client_secret.trim().to_string();

    // Create a new YAML string with the updated credentials
    let yaml_content = format!(
        "Client ID: \"{}\"\nClient Secret: \"{}\"",
        client_id, client_secret
    );

    // Open the file for writing
    let file = File::create(&path).expect("Unable to create creds file");
    let mut writer = BufWriter::new(file);

    // Write the updated YAML content to the file
    writer
        .write_all(yaml_content.as_bytes())
        .expect("Unable to write to creds file");

    println!("Please run spoify again.");
}
