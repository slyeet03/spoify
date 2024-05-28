use crate::app::App;
use crate::enums::Menu;

use ratatui::prelude::*;

use super::help::{render_default_help, render_help};
use super::library::{render_default_library, render_library};
use super::lyrics::render_lyrics;
use super::main_area::render_main_area;
use super::new_release::{render_default_new_releases, render_new_releases};
use super::player::render_player;
use super::search::{render_default_search, render_search};
use super::search_album::render_searched_album;
use super::search_artist::render_searched_artist;
use super::user_playlist::{render_default_user_playlist, render_user_playlist};

/// Renders the main frame of the application's user interface
pub fn render_frame(f: &mut Frame, selected_menu: Menu, app: &mut App) {
    // Calculate the layout constraints
    let size = f.size();

    // Whole display layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(8),
            Constraint::Percentage(82),
            Constraint::Percentage(10),
        ])
        .split(size);

    // Dividing the header into two horizontal layouts: Search section and Help section
    let header_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(chunks[0]);

    // Dividing the middle layout into three horizontal layouts: Library/New Release section, Main screen section and User Playlist section
    let content_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(chunks[1]);

    // Dividing the fist portion of middle layout into two vertical layouts: Library section and New Release section
    let content_sub_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(content_chunk[0]);

    // Dividing the middle screen into 2 vertical layout which will be firther divided into two horizontal layouts each to have the middle screen be divided into 4 sections
    let main_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(content_chunk[1]);

    // Dividing the middle screen upper layout into two section: Songs and Artist section
    let main_chunk_upper = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunk[0]);

    // Dividing the midlle screen lower layout into two section: Album and Playlist section
    let main_chunk_lower = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunk[1]);

    // Making the live player layout
    let player_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    // Render the default UI
    render_default_search(f, &header_chunk, app);
    render_default_library(f, &content_sub_chunk, app);
    render_default_user_playlist(f, &content_chunk, app);
    render_player(f, &player_layout, app);
    render_main_area(f, &content_chunk, app);
    render_default_help(f, &header_chunk, app);
    render_default_new_releases(f, &content_sub_chunk, app);

    // Render different sections based on the selected menu
    match selected_menu {
        Menu::Default => {}
        Menu::Main => {}
        Menu::Library => {
            render_library(f, &content_sub_chunk, &content_chunk, app);
        }
        Menu::Playlists => {
            render_user_playlist(f, &content_chunk, app);
        }
        Menu::Search => {
            render_search(f, &header_chunk, &main_chunk_upper, &main_chunk_lower, app);
        }
        Menu::Help => {
            render_help(f, app);
        }
        Menu::NewRelease => {
            render_new_releases(f, &content_sub_chunk, &content_chunk, app);
        }
        Menu::Lyrics => {
            render_lyrics(f, &content_chunk, app);
        }
        Menu::SearchedAlbum => {
            render_searched_album(f, &content_chunk, app);
        }
        Menu::SearchedArtist => {
            render_searched_artist(f, &content_chunk, app);
        }
    }
}
