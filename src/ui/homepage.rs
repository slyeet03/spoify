use ratatui::layout::Corner;
use ratatui::prelude::*;
use ratatui::style::{Color, Style};
use ratatui::widgets::ListItem;
use ratatui::widgets::{block::*, *};
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
    let playlist_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Playlist"));
    let player_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Player"));
    let content_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Welcome!"));

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

    //rendering the default ui
    f.render_widget(search_block, header_chunk[0]);
    f.render_widget(&library_list, content_sub_chunk[0]);
    f.render_widget(playlist_block, content_sub_chunk[1]);
    f.render_widget(player_block, chunks[2]);
    f.render_widget(content_block, content_chunk[1]);

    //rendering different sections based on the selected menu
    match selected_menu {
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
            let playlist_block = Block::default()
                .borders(Borders::ALL)
                .title(Title::from("Playlist"))
                .border_style(Style::new().fg(Color::Yellow));
            f.render_widget(playlist_block, content_sub_chunk[1]);
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
                InputMode::SearchResults => {
                    let search_results = create_search_results_list(
                        &app.album_names,
                        &app.track_names,
                        &app.playlist_names,
                    );
                    let content_block = Block::default()
                        .borders(Borders::ALL)
                        .title(Title::from("Search Result"))
                        .border_style(Style::new().fg(Color::Yellow));

                    let search_results_list = List::new(search_results)
                        .block(content_block.clone())
                        .start_corner(Corner::TopLeft);

                    f.render_widget(search_results_list, content_chunk[1]);
                }
            }
        }
    }
}

fn create_search_results_list<'a>(
    album_names: &'a [String],
    track_names: &'a [String],
    playlist_names: &'a [String],
) -> Vec<ListItem<'a>> {
    let mut search_results = Vec::new();

    for name in album_names {
        search_results.push(ListItem::new(format!("Album: {}", name)));
    }

    for name in track_names {
        search_results.push(ListItem::new(format!("Track: {}", name)));
    }

    for name in playlist_names {
        search_results.push(ListItem::new(format!("Playlist: {}", name)));
    }

    search_results
}
