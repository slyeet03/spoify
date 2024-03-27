use ratatui::prelude::*;
use ratatui::style::{Color, Style};
use ratatui::widgets::block::*;
use ratatui::widgets::ListItem;
use ratatui::widgets::{Block, Borders, List, Paragraph};

use crate::app::App;
use crate::enums::{InputMode, Menu};

pub fn render_frame(f: &mut Frame, selected_menu: Menu, app: &mut App) {
    //define library items
    let library_items = vec![
        String::from("Made For You"),
        String::from("Recently Played"),
        String::from("Liked Songs"),
        String::from("Albums"),
        String::from("Artists"),
        String::from("Podcasts"),
    ];
    //creating all the ui blocks
    let search_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Search"));
    let library_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Library"));
    let playlist_block_user = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Playlist"));
    let player_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Player"));
    let content_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Welcome!"));

    let album_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Albums"));
    let artist_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Artists"));
    let song_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Songs"));
    let playlist_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Playlists"));

    let search_input = Paragraph::new(app.input.as_str())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
            InputMode::SearchResults => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL).title("Search"));

    //list widget for library items
    let library_list = List::new(library_items.clone()).block(library_block);
    let size = f.size();
    // main display layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(8),
            Constraint::Percentage(72),
            Constraint::Percentage(20),
        ])
        .split(size);
    // search layout
    let header_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
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

    //rendering the default ui
    f.render_widget(search_block, header_chunk[0]);
    f.render_widget(&library_list, content_sub_chunk[0]);
    f.render_widget(playlist_block_user, content_sub_chunk[1]);
    f.render_widget(player_block, chunks[2]);
    f.render_widget(content_block, content_chunk[1]);
    //rendering different sections based on the selected menu
    match selected_menu {
        Menu::Default => {}
        Menu::Main => {
            let content_block = Block::default()
                .borders(Borders::ALL)
                .title(Title::from("Welcome!"))
                .border_style(Style::new().fg(Color::Yellow));
            f.render_widget(content_block, content_chunk[1]);
        }
        Menu::Library => {
            let library_block = Block::default()
                .borders(Borders::ALL)
                .title(Title::from("Library"))
                .border_style(Style::new().fg(Color::Yellow));

            let library_items = vec![
                String::from("Made For You"),
                String::from("Recently Played"),
                String::from("Liked Songs"),
                String::from("Albums"),
                String::from("Artists"),
                String::from("Podcasts"),
            ];
            //rendering currently selected menu
            let library_list = List::new(library_items)
                .block(library_block)
                .highlight_style(Style::default().fg(Color::Yellow))
                .highlight_symbol("> ");

            f.render_stateful_widget(
                library_list.highlight_symbol(">>"),
                content_sub_chunk[0],
                &mut app.library_state,
            );
        }
        Menu::Playlists => {
            let playlist_block_user = Block::default()
                .borders(Borders::ALL)
                .title(Title::from("Playlist"))
                .border_style(Style::new().fg(Color::Yellow));
            f.render_widget(playlist_block_user, content_sub_chunk[1]);
        }
        Menu::Search => {
            let search_block = Block::default()
                .borders(Borders::ALL)
                .title(Title::from("Search"))
                .border_style(Style::new().fg(Color::Yellow));

            f.render_widget(search_block, header_chunk[0]);

            match app.input_mode {
                InputMode::Normal => {}
                InputMode::Editing => {
                    f.render_widget(search_input, header_chunk[0]);
                    f.set_cursor(
                        header_chunk[0].x + app.cursor_position as u16 + 1,
                        header_chunk[0].y + 1,
                    );
                }
                InputMode::SearchResults if app.search_results_rendered => {
                    let album_search_results = search_results_album(&app.album_names);
                    let artist_search_results = search_results_artist(&app.artist_names);
                    let song_search_results = search_results_songs(&app.track_names);
                    let playlist_search_results = search_results_playlist(&app.playlist_names);

                    let album_list = List::new(album_search_results).block(album_block.clone());

                    let song_list = List::new(song_search_results).block(song_block.clone());

                    let playlist_list =
                        List::new(playlist_search_results).block(playlist_block.clone());

                    let artist_list = List::new(artist_search_results).block(artist_block.clone());

                    f.render_widget(song_list, main_chunk_upper[0]);
                    f.render_widget(artist_list, main_chunk_upper[1]);
                    f.render_widget(album_list, main_chunk_lower[0]);
                    f.render_widget(playlist_list, main_chunk_lower[1]);
                }
                _ => {}
            }
        }
    }
}

fn search_results_album<'a>(album_names: &'a [String]) -> Vec<ListItem<'a>> {
    let mut search_results = Vec::new();

    for name in album_names {
        search_results.push(ListItem::new(format!("{}", name)));
    }
    search_results
}
fn search_results_songs<'a>(track_names: &'a [String]) -> Vec<ListItem<'a>> {
    let mut search_results = Vec::new();

    for name in track_names {
        search_results.push(ListItem::new(format!("{}", name)));
    }

    search_results
}
fn search_results_playlist<'a>(playlist_names: &'a [String]) -> Vec<ListItem<'a>> {
    let mut search_results = Vec::new();

    for name in playlist_names {
        search_results.push(ListItem::new(format!("{}", name)));
    }

    search_results
}
fn search_results_artist<'a>(artist_names: &'a [String]) -> Vec<ListItem<'a>> {
    let mut search_results = Vec::new();

    for name in artist_names {
        search_results.push(ListItem::new(format!("{}", name)));
    }

    search_results
}
