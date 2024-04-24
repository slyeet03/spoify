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

    // Try to get the theme color values from the configuration.
    // If a value is not found or invalid, use default values.
    if let Some(border_color) = theme.get("Border Color") {
        if let Some(rgb) = border_color.as_str() {
            let (r, g, b) = parse_color(rgb);
            app.border_color = Color::Rgb(r, g, b);
        }
    }

    if let Some(highlight_color) = theme.get("Highlight Color") {
        if let Some(rgb) = highlight_color.as_str() {
            let (r, g, b) = parse_color(rgb);
            app.highlight_color = Color::Rgb(r, g, b);
        }
    }

    if let Some(background_color) = theme.get("Background Color") {
        if let Some(rgb) = background_color.as_str() {
            let (r, g, b) = parse_color(rgb);
            app.background_color = Color::Rgb(r, g, b);
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
