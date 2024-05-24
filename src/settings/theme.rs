use ratatui::style::Color;
use serde::Deserialize;
use serde_json::Value;
use serde_yaml;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::app::App;

#[derive(Deserialize, Debug)]
struct Theme(HashMap<String, Value>);

/// Reads the theme configuration file and returns the parsed theme data as a HashMap
pub fn read_theme() -> HashMap<String, Value> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("..");
    path.push("spoify-tui");
    path.push("configure");
    path.push("theme.yml");

    let file = File::open(&path).expect("Unable to open theme file");
    let reader = BufReader::new(file);

    let theme: Theme = serde_yaml::from_reader(reader).expect("Unable to parse theme from YAML");

    theme.0
}

/// Sets the application theme based on the configuration loaded from the theme file
pub fn set_theme(app: &mut App) {
    let theme = read_theme();

    // Iterate over all entries in the theme HashMap
    for (key, value) in theme.iter() {
        if let Some(rgb) = value.as_str() {
            let (r, g, b) = parse_color(rgb);
            match key.as_str() {
                "Player Border Color" => app.player_border_color = Color::Rgb(r, g, b),
                "Player Background Color" => app.player_background_color = Color::Rgb(r, g, b),
                "Player Highlight Color" => app.player_highlight_color = Color::Rgb(r, g, b),

                "Library Border Color" => app.library_border_color = Color::Rgb(r, g, b),
                "Library Background Color" => app.library_background_color = Color::Rgb(r, g, b),
                "Library Highlight Color" => app.library_highlight_color = Color::Rgb(r, g, b),

                "Playlist Border Color" => app.playlist_border_color = Color::Rgb(r, g, b),
                "Playlist Background Color" => app.playlist_background_color = Color::Rgb(r, g, b),
                "Playlist Highlight Color" => app.playlist_highlight_color = Color::Rgb(r, g, b),

                "New Release Border Color" => app.new_release_border_color = Color::Rgb(r, g, b),
                "New Release Background Color" => {
                    app.new_release_background_color = Color::Rgb(r, g, b)
                }
                "New Release Highlight Color" => {
                    app.new_release_highlight_color = Color::Rgb(r, g, b)
                }

                "Main Section Border Color" => app.main_border_color = Color::Rgb(r, g, b),
                "Main Section Background Color" => app.main_background_color = Color::Rgb(r, g, b),
                "Main Section Highlight Color" => app.main_highlight_color = Color::Rgb(r, g, b),

                "Search Border Color" => app.search_border_color = Color::Rgb(r, g, b),
                "Search Background Color" => app.search_background_color = Color::Rgb(r, g, b),
                "Search Highlight Color" => app.search_highlight_color = Color::Rgb(r, g, b),

                "Help Border Color" => app.help_border_color = Color::Rgb(r, g, b),
                "Help Background Color" => app.help_background_color = Color::Rgb(r, g, b),
                "Help Highlight Color" => app.help_highlight_color = Color::Rgb(r, g, b),
                _ => (),
            }
        }
    }
}

/// Parses an RGB color string in the format "Color::Rgb(r, g, b)" into a tuple of (u8, u8, u8)
fn parse_color(value: &str) -> (u8, u8, u8) {
    let binding = value.replace("Color::Rgb(", "").replace(")", "");
    let rgb: Vec<&str> = binding.split(", ").collect();

    (
        rgb[0].parse().unwrap(),
        rgb[1].parse().unwrap(),
        rgb[2].parse().unwrap(),
    )
}
