use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{block::Title, Block, Borders, Clear},
    Frame,
};

use crate::{app::App, ui::util::track_table_ui};

pub fn render_searched_playlist(f: &mut Frame, content_chunk: &[Rect], app: &mut App) {
    let current_playlist = &app.playlist_names_search_results[app.playlist_index];

    let playlist_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from(format!("{}", current_playlist)))
        .border_style(if app.searched_playlist_selected {
            Style::default().fg(app.main_border_color)
        } else {
            Style::default()
        })
        .style(Style::default().bg(app.main_background_color));

    let track_table = track_table_ui(
        app.selected_playlist_tracks_names.clone(),
        app.selected_playlist_tracks_artists.clone(),
        app.selected_playlist_tracks_albums.clone(),
        app.selected_playlist_tracks_duration.clone(),
        playlist_block,
        app.main_highlight_color.clone(),
        app.main_background_color.clone(),
    );

    f.render_widget(Clear, content_chunk[1]);

    f.render_stateful_widget(
        track_table,
        content_chunk[1],
        &mut app.searched_playlist_state,
    );
}
