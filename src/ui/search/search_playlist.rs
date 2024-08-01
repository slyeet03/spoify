use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{block::Title, Block, Borders, Clear},
    Frame,
};

use crate::{app::App, structs::Themes, ui::util::track_table_ui};

pub fn render_searched_playlist(
    f: &mut Frame,
    content_chunk: &[Rect],
    app: &mut App,
    theme: &mut Themes,
) {
    f.render_widget(Clear, content_chunk[1]);
    let current_playlist = &app.playlist_names_search_results[app.playlist_index];

    let playlist_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from(current_playlist.to_string()))
        .border_style(if app.searched_playlist_selected {
            Style::default().fg(theme.main_border_color)
        } else {
            Style::default().fg(theme.main_inactive_border_color)
        })
        .style(Style::default().bg(theme.main_background_color));

    let track_table = track_table_ui(
        app.selected_playlist_tracks_names.clone(),
        app.selected_playlist_tracks_artists.clone(),
        app.selected_playlist_tracks_albums.clone(),
        app.selected_playlist_tracks_duration.clone(),
        playlist_block,
        theme.main_highlight_color,
        theme.main_background_color,
        theme.main_inactive_border_color,
    );

    f.render_widget(Clear, content_chunk[1]);

    f.render_stateful_widget(
        track_table,
        content_chunk[1],
        &mut app.searched_playlist_state,
    );
}
