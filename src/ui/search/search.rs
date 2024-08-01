use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{block::Title, Block, Borders, Clear, List, Paragraph},
    Frame,
};

use crate::{
    app::App,
    enums::{InputMode, SearchMenu},
    structs::Themes,
    ui::util::convert_to_list,
};

use super::{
    search_album::render_searched_album, search_artist::render_searched_artist,
    search_playlist::render_searched_playlist,
};

/// Renders the search UI section, including search input, category blocks, and search results
pub fn render_search(
    f: &mut Frame,
    header_chunk: &[Rect],
    main_chunk_upper: &[Rect],
    main_chunk_lower: &[Rect],
    content_chunk: &[Rect],
    app: &mut App,
    theme: &mut Themes,
) {
    let search_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Search"))
        .border_style(Style::new().fg(theme.search_border_color))
        .style(Style::default().bg(theme.search_background_color));

    // Create styled blocks for each search category (albums, artists, etc.) with dynamic borders
    let album_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Albums"))
        .border_style(if app.selected_album_in_search_result {
            Style::default().fg(theme.search_border_color)
        } else {
            Style::default().fg(theme.search_inactive_border_color)
        })
        .style(Style::default().bg(theme.search_background_color));
    let artist_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Artists"))
        .border_style(if app.selected_artist_in_search_result {
            Style::default().fg(theme.search_border_color)
        } else {
            Style::default().fg(theme.search_inactive_border_color)
        })
        .style(Style::default().bg(theme.search_background_color));
    let song_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Songs"))
        .border_style(if app.selected_track_in_search_result {
            Style::default().fg(theme.search_border_color)
        } else {
            Style::default().fg(theme.search_inactive_border_color)
        })
        .style(Style::default().bg(theme.search_background_color));
    let playlist_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Playlists"))
        .border_style(if app.selected_playlist_in_search_result {
            Style::default().fg(theme.search_border_color)
        } else {
            Style::default().fg(theme.search_inactive_border_color)
        })
        .style(Style::default().bg(theme.search_background_color));

    // Create a Paragraph widget for displaying the search input text
    let search_input = Paragraph::new(app.input.as_str())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(theme.search_border_color),
            InputMode::SearchResults => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL).title("Search"))
        .style(
            Style::default()
                .bg(theme.search_background_color)
                .fg(theme.search_inactive_border_color),
        );

    f.render_widget(search_block, header_chunk[0]);

    // Render search input or search results depending on the input mode
    match app.input_mode {
        InputMode::Normal => {}
        InputMode::Editing => {
            // Render the search input with a cursor at the current position
            f.render_widget(search_input, header_chunk[0]);
            f.set_cursor(
                header_chunk[0].x + app.cursor_position as u16 + 1,
                header_chunk[0].y + 1,
            );
        }
        InputMode::SearchResults if app.search_results_rendered => {
            f.render_widget(Clear, content_chunk[1]);
            f.render_widget(Clear, main_chunk_upper[0]);

            let album_names_list = convert_to_list(&app.album_names_search_results);
            let track_names_list = convert_to_list(&app.track_names_search_results);
            let artist_names_list = convert_to_list(&app.artist_names_search_results);
            let playlist_names_list = convert_to_list(&app.playlist_names_search_results);

            let album_list = List::new(album_names_list)
                .block(album_block.clone())
                .highlight_style(Style::default().fg(theme.search_highlight_color));

            let song_list = List::new(track_names_list)
                .block(song_block.clone())
                .highlight_style(Style::default().fg(theme.search_highlight_color));

            let playlist_list = List::new(playlist_names_list)
                .block(playlist_block.clone())
                .highlight_style(Style::default().fg(theme.search_highlight_color));

            let artist_list = List::new(artist_names_list)
                .block(artist_block.clone())
                .highlight_style(Style::default().fg(theme.search_highlight_color));

            f.render_stateful_widget(
                song_list,
                main_chunk_upper[0],
                &mut app.track_state_in_search_result,
            );
            f.render_stateful_widget(
                artist_list,
                main_chunk_upper[1],
                &mut app.artist_state_in_search_result,
            );
            f.render_stateful_widget(
                album_list,
                main_chunk_lower[0],
                &mut app.album_state_in_search_result,
            );
            f.render_stateful_widget(
                playlist_list,
                main_chunk_lower[1],
                &mut app.playlist_state_in_search_result,
            );
        }
        _ => {}
    }
    match app.search_menu {
        SearchMenu::Default => {}
        SearchMenu::SearchedTrack => {}
        SearchMenu::SearchedAlbum => {
            f.render_widget(Clear, content_chunk[1]);
            render_searched_album(f, content_chunk, app, theme);
        }
        SearchMenu::SearchedArtist => {
            f.render_widget(Clear, content_chunk[1]);
            render_searched_artist(f, content_chunk, app, theme);
        }
        SearchMenu::SearchedPlaylist => {
            f.render_widget(Clear, content_chunk[1]);
            render_searched_playlist(f, content_chunk, app, theme);
        }
    }
}

/// Renders a simplified search bar
pub fn render_default_search(f: &mut Frame, header_chunk: &[Rect], theme: &mut Themes) {
    let search_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Search"))
        .style(
            Style::default()
                .bg(theme.search_background_color)
                .fg(theme.search_inactive_border_color),
        );

    f.render_widget(search_block, header_chunk[0]);
}
