extern crate lyric_finder;

use crate::app::App;
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::{json, Value};
use std::fs;
use std::io::Read;
use std::{fs::File, io::Write, path::PathBuf};

lazy_static! {
    static ref RE: Regex = Regex::new(r#"(?:"((?:\\.|[^\\"])*)")|([^"]+)"#).unwrap();
}

#[tokio::main]
pub async fn lyric(app: &mut App) -> anyhow::Result<()> {
    let client = lyric_finder::Client::new();
    let result = client.get_lyric(&app.argument_for_lyric).await?;

    match result {
        lyric_finder::LyricResult::Some {
            track: _,
            artists: _,
            lyric,
        } => {
            save_data_to_json(app, lyric.clone());
        }
        lyric_finder::LyricResult::None => {
            let err: String = "lyric not found!".to_string();
            save_data_to_json(app, err);
        }
    }

    Ok(())
}

// Function to save the lyrics of currently playing track data to a text file
fn save_data_to_json(app: &mut App, items: String) {
    let json_data: Value = json!(items);
    let mut path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("spotify_cache");
    std::fs::create_dir_all(&path).unwrap();
    path.push("lyrics.txt");

    let mut file: File = File::create(&path).unwrap();
    let _ = file.write_all(json_data.to_string().as_bytes());
    process_lyrics(app);
}

fn process_lyrics(app: &mut App) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify-tui");
    path.push("spotify_cache");
    path.push("lyrics.txt");

    let mut file_contents = String::new();
    let mut err: Option<std::io::Error> = None;

    if let Ok(mut file) = fs::File::open(&path) {
        if let Err(e) = file.read_to_string(&mut file_contents) {
            err = Some(e);
        }
    } else {
        err = Some(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Failed to open the file",
        ));
    }

    if let Some(e) = err {
        app.error_text = format!("Error: {}", e);
    } else {
        render_lyrics_data(app, file_contents);
    }
}

fn render_lyrics_data(app: &mut App, text: String) {
    let lines: Vec<String> = text
        .replace("\\n", "\n") // Replace \n with actual newline
        .lines() // Split the text into lines
        .flat_map(|line| {
            RE.captures_iter(&line)
                .map(|cap| {
                    let captured = cap.get(1).unwrap_or_else(|| cap.get(2).unwrap()).as_str();
                    captured.replace(r#"\""#, "\"").to_string()
                })
                .collect::<Vec<_>>()
        })
        .collect();

    app.lyrics = lines.clone();
}
