use ratatui::style::Color;
use serde::Deserialize;
use serde_json::Value;
use serde_yaml;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::app::App;
use crate::structs::{Settings, Themes};

#[derive(Deserialize, Debug)]
struct Theme(HashMap<String, Value>);

/// Reads the theme configuration file and returns the parsed theme data as a HashMap
pub fn read_theme(app: &mut App, settings: &mut Settings) -> HashMap<String, Value> {
    let file_name = format!("{}.yml", settings.theme_name.clone());

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("..");
    path.push(app.file_name.clone());
    path.push("configure");
    path.push(file_name);

    let file = File::open(&path).expect("Unable to open theme file");
    let reader = BufReader::new(file);

    let theme: Theme = serde_yaml::from_reader(reader).expect("Unable to parse theme from YAML");

    theme.0
}

/// Sets the application theme based on the configuration loaded from the theme file
pub fn set_theme(app: &mut App, themes: &mut Themes, settings: &mut Settings) {
    let theme = read_theme(app, settings);

    // Iterate over all entries in the theme HashMap
    for (key, value) in theme.iter() {
        if let Some(rgb) = value.as_str() {
            let (r, g, b) = parse_color(rgb);
            match key.as_str() {
                "Player Border Color" => themes.player_border_color = Color::Rgb(r, g, b),
                "Player Background Color" => themes.player_background_color = Color::Rgb(r, g, b),
                "Player Highlight Color" => themes.player_highlight_color = Color::Rgb(r, g, b),

                "Library Border Color" => themes.library_border_color = Color::Rgb(r, g, b),
                "Library Background Color" => themes.library_background_color = Color::Rgb(r, g, b),
                "Library Highlight Color" => themes.library_highlight_color = Color::Rgb(r, g, b),

                "Playlist Border Color" => themes.playlist_border_color = Color::Rgb(r, g, b),
                "Playlist Background Color" => {
                    themes.playlist_background_color = Color::Rgb(r, g, b)
                }
                "Playlist Highlight Color" => themes.playlist_highlight_color = Color::Rgb(r, g, b),

                "New Release Border Color" => themes.new_release_border_color = Color::Rgb(r, g, b),
                "New Release Background Color" => {
                    themes.new_release_background_color = Color::Rgb(r, g, b)
                }
                "New Release Highlight Color" => {
                    themes.new_release_highlight_color = Color::Rgb(r, g, b)
                }

                "Main Section Border Color" => themes.main_border_color = Color::Rgb(r, g, b),
                "Main Section Background Color" => {
                    themes.main_background_color = Color::Rgb(r, g, b)
                }
                "Main Section Highlight Color" => themes.main_highlight_color = Color::Rgb(r, g, b),

                "Search Border Color" => themes.search_border_color = Color::Rgb(r, g, b),
                "Search Background Color" => themes.search_background_color = Color::Rgb(r, g, b),
                "Search Highlight Color" => themes.search_highlight_color = Color::Rgb(r, g, b),

                "Help Border Color" => themes.help_border_color = Color::Rgb(r, g, b),
                "Help Background Color" => themes.help_background_color = Color::Rgb(r, g, b),
                "Help Highlight Color" => themes.help_highlight_color = Color::Rgb(r, g, b),

                "Error Border Color" => themes.error_border_color = Color::Rgb(r, g, b),
                "Error Background Color" => themes.error_background_color = Color::Rgb(r, g, b),
                _ => (),
            }
        }
    }
}

/// Parses an RGB color string in the format "Color::Rgb(r, g, b)" into a tuple of (u8, u8, u8)
fn parse_color(value: &str) -> (u8, u8, u8) {
    let binding = value.replace("Color::Rgb(", "").replace(')', "");
    let rgb: Vec<&str> = binding.split(", ").collect();

    (
        rgb[0].parse().unwrap(),
        rgb[1].parse().unwrap(),
        rgb[2].parse().unwrap(),
    )
}
