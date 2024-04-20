use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{block::Title, Block, Borders, Clear, List, Paragraph},
    Frame,
};

use crate::{app::App, enums::InputMode};

use super::util::convert_to_list;

pub fn render_search(
    f: &mut Frame,
    header_chunk: &[Rect],
    main_chunk_upper: &[Rect],
    main_chunk_lower: &[Rect],
    app: &mut App,
) {
    let search_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Search"))
        .border_style(Style::new().fg(app.border_color))
        .style(Style::default().bg(app.background_color));

    let album_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Albums"))
        .border_style(if app.selected_album {
            Style::default().fg(app.border_color)
        } else {
            Style::default()
        })
        .style(Style::default().bg(app.background_color));
    let artist_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Artists"))
        .border_style(if app.selected_artist {
            Style::default().fg(app.border_color)
        } else {
            Style::default()
        })
        .style(Style::default().bg(app.background_color));
    let song_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Songs"))
        .border_style(if app.selected_track {
            Style::default().fg(app.border_color)
        } else {
            Style::default()
        })
        .style(Style::default().bg(app.background_color));
    let playlist_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Playlists"))
        .border_style(if app.selected_playlist {
            Style::default().fg(app.border_color)
        } else {
            Style::default()
        })
        .style(Style::default().bg(app.background_color));

    // Create the search input paragraph
    let search_input = Paragraph::new(app.input.as_str())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(app.border_color),
            InputMode::SearchResults => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL).title("Search"))
        .style(Style::default().bg(app.background_color));

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
            f.render_widget(Clear, main_chunk_upper[0]);

            let album_names_list = convert_to_list(&app.album_names);
            let track_names_list = convert_to_list(&app.track_names);
            let artist_names_list = convert_to_list(&app.artist_names);
            let playlist_names_list = convert_to_list(&app.playlist_names);

            let album_list = List::new(album_names_list)
                .block(album_block.clone())
                .highlight_style(Style::default().fg(app.highlight_color));

            let song_list = List::new(track_names_list)
                .block(song_block.clone())
                .highlight_style(Style::default().fg(app.highlight_color));

            let playlist_list = List::new(playlist_names_list)
                .block(playlist_block.clone())
                .highlight_style(Style::default().fg(app.highlight_color));

            let artist_list = List::new(artist_names_list)
                .block(artist_block.clone())
                .highlight_style(Style::default().fg(app.highlight_color));

            f.render_stateful_widget(song_list, main_chunk_upper[0], &mut app.track_state);
            f.render_stateful_widget(artist_list, main_chunk_upper[1], &mut app.artist_state);
            f.render_stateful_widget(album_list, main_chunk_lower[0], &mut app.album_state);
            f.render_stateful_widget(playlist_list, main_chunk_lower[1], &mut app.playlist_state);
        }
        _ => {}
    }
}

pub fn render_default_search(f: &mut Frame, header_chunk: &[Rect], app: &mut App) {
    let search_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Search"))
        .style(Style::default().bg(app.background_color));

    f.render_widget(search_block, header_chunk[0]);
}
