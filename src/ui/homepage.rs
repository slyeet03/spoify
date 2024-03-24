use ratatui::prelude::*;
use ratatui::style::{Color, Style};
use ratatui::widgets::{block::*, *};

use crate::enums::Menu;

pub fn render_frame(f: &mut Frame, selected_menu: Menu, selected_index: usize) {
    let library_items = vec![
        String::from("Made For You"),
        String::from("Recently Played"),
        String::from("Liked Songs"),
        String::from("Albums"),
        String::from("Artists"),
        String::from("Podcasts"),
    ];
    let search_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Search"));
    let library_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Library"));
    let playlist_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Playlist"));
    let player_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Player"));
    let content_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Welcome!"));

    let mut library_list = List::new(library_items.clone()).block(library_block);

    let size = f.size();
    // main display
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(70),
            Constraint::Percentage(20),
        ])
        .split(size);
    // search and help display
    let header_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .split(chunks[0]);
    // library, playlist and main content display
    let content_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(chunks[1]);
    // library and playlist
    let content_sub_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(content_chunk[0]);

    match selected_menu {
        Menu::Default => {
            f.render_widget(search_block.clone(), header_chunk[0]);
            f.render_widget(&library_list, content_sub_chunk[0]);
            f.render_widget(playlist_block.clone(), content_sub_chunk[1]);
            f.render_widget(player_block.clone(), chunks[2]);
            f.render_widget(content_block.clone(), content_chunk[1]);
        }
        Menu::Main => {
            let content_block = Block::default()
                .borders(Borders::ALL)
                .title(Title::from("Welcome!"))
                .border_style(Style::new().fg(Color::Yellow));
            f.render_widget(content_block, content_chunk[1]);
        }
        Menu::Library => {
            // Render library with highlight
            let library_block = Block::default()
                .borders(Borders::ALL)
                .title(Title::from("Library"))
                .border_style(Style::new().fg(Color::Yellow));

            let library_list = List::new(library_items).block(library_block);

            f.render_widget(library_list, content_sub_chunk[0]);
        }
        Menu::Playlists => {
            // Render playlists with highlight
            let playlist_block = Block::default()
                .borders(Borders::ALL)
                .title(Title::from("Playlist"))
                .border_style(Style::new().fg(Color::Yellow));
            f.render_widget(playlist_block, content_sub_chunk[1]);
        }
        Menu::Search => {
            // Render search with highlight
            let search_block = Block::default()
                .borders(Borders::ALL)
                .title(Title::from("Search"))
                .border_style(Style::new().fg(Color::Yellow));
            f.render_widget(search_block, header_chunk[0]);
        }
    }

    f.render_widget(search_block, header_chunk[0]);
    f.render_widget(&library_list, content_sub_chunk[0]);
    f.render_widget(playlist_block, content_sub_chunk[1]);
    f.render_widget(player_block, chunks[2]);
    f.render_widget(content_block, content_chunk[1]);
}
