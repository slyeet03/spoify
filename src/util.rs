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
pub fn startup(app: &mut App) {
    // Set the keybindings from the configure files
    read_keybindings();
    set_keybindings(app);
    parse_keybindings(app);

    // Set the theme from the configure files
    read_theme();
    set_theme(app);

    // Set the volume increament and decreament values
    read_volume_values();
    set_volume_values(app);

    // Fetch the new released albums from spotify
    let _ = new_releases(app);
    process_new_releases(app);

    // Fetch user playlists from spotify
    get_playlists(app);
    process_user_playlists(app);
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
        7. Go to the spoify folder, and inside 'configure' folder go to 'creds.yml'.
        8. Enter you 'Client ID' and 'Client Secret'.
        9. Run spoify
        10. You will be redirected to an official Spotify webpage to ask you for permissions.
        11. After accepting the permissions, you'll be redirected to localhost.
            You'll be redirected to a blank webpage that might say something like 'Connection Refused' since no server is running. 
            Regardless, copy the URL and paste into the prompt in the terminal.
        
        There we go, now you can use spoify.
    ");
}
