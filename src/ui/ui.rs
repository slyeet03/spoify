use crate::app::App;
use crate::enums::Menu;

use ratatui::prelude::*;

use super::help::render_default_help;
use super::library::{render_default_library, render_library};
use super::main_area::render_main_area;
use super::player::render_player;
use super::search::{render_default_search, render_search};
use super::user_playlist::{render_default_user_playlist, render_user_playlist};

/// Renders the main frame of the application's user interface
pub fn render_frame(f: &mut Frame, selected_menu: Menu, app: &mut App) {
    // Calculate the layout constraints
    let size = f.size();

    // main display layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(8),
            Constraint::Percentage(82),
            Constraint::Percentage(10),
        ])
        .split(size);

    // search layout
    let header_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(chunks[0]);

    // library, playlist and main content display layout
    let content_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(chunks[1]);

    // library and playlist layout
    let content_sub_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(content_chunk[0]);

    let main_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(content_chunk[1]);

    let main_chunk_upper = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunk[0]);

    let main_chunk_lower = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunk[1]);

    // Create the player section layout
    let player_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    // Render the default UI
    render_default_search(f, &header_chunk, app);
    render_default_library(f, &content_sub_chunk, app);
    render_default_user_playlist(f, &content_sub_chunk, app);
    render_player(f, &player_layout, app);
    render_main_area(f, &content_chunk, app);
    render_default_help(f, &header_chunk, app);

    // Render different sections based on the selected menu
    match selected_menu {
        Menu::Default => {}
        Menu::Main => {
            // TODO: Add tabbing function through artists, albums, songs, and playlists
            // TODO: Add menu navigation inside those blocks
        }
        Menu::Library => {
            render_library(f, &content_sub_chunk, &content_chunk, app);
        }
        Menu::Playlists => {
            render_user_playlist(f, &content_sub_chunk, &content_chunk, app);
        }
        Menu::Search => {
            render_search(f, &header_chunk, &main_chunk_upper, &main_chunk_lower, app);
        }
        Menu::Help => {
            //TODO: Add Help Page
        }
    }
}
