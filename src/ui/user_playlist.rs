use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{block::Title, Block, Borders, Clear, List},
    Frame,
};

use crate::app::App;

use super::util::{convert_to_list, track_table_ui};

/// Renders the user playlist UI section, including playlist selection list and track table
pub fn render_user_playlist(f: &mut Frame, content_chunk: &[Rect], app: &mut App) {
    let current_playlist_name = (&app.current_user_playlist).to_string();

    let playlist_block_user = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Playlist"))
        .border_style(Style::new().fg(app.playlist_border_color))
        .style(Style::default().bg(app.playlist_background_color));

    let user_playlist_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from(current_playlist_name))
        .border_style(if app.user_playlist_tracks_selected {
            Style::default().fg(app.playlist_border_color)
        } else {
            Style::default()
        })
        .style(Style::default().bg(app.playlist_background_color));

    // Convert app data (user playlist names) to a List widget
    let user_playlist_names = convert_to_list(&app.user_playlist_names);
    let user_playlist_list = List::new(user_playlist_names)
        .block(playlist_block_user.clone())
        .highlight_style(Style::default().fg(app.playlist_highlight_color));

    f.render_widget(Clear, content_chunk[2]);

    f.render_stateful_widget(
        user_playlist_list,
        content_chunk[2],
        &mut app.user_playlist_state,
    );

    // Conditionally render the user playlist track table
    if app.user_playlist_display {
        f.render_widget(Clear, content_chunk[1]);

        let user_playlist_tracks_table = track_table_ui(
            app.user_playlist_track_names.clone(),
            app.user_playlist_artist_names.clone(),
            app.user_playlist_album_names.clone(),
            app.user_playlist_track_duration.clone(),
            user_playlist_block,
            app.playlist_highlight_color.clone(),
            app.playlist_background_color.clone(),
        );

        f.render_widget(Clear, content_chunk[1]);

        f.render_stateful_widget(
            user_playlist_tracks_table,
            content_chunk[1],
            &mut app.user_playlist_tracks_state,
        );
    }
}

/// Renders a default UI for the user playlist section when no playlist is selected
pub fn render_default_user_playlist(f: &mut Frame, content_chunk: &[Rect], app: &mut App) {
    let playlist_block_user = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Playlist"))
        .style(Style::default().bg(app.playlist_background_color));

    // Convert app data (user playlist names) to a List widget
    let user_playlist_names = convert_to_list(&app.user_playlist_names);
    let user_playlist_list = List::new(user_playlist_names).block(playlist_block_user.clone());

    f.render_widget(user_playlist_list, content_chunk[2]);
}
