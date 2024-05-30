use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{block::Title, Block, Borders, Clear},
    Frame,
};

use super::util::searched_track_table_for_artist_ui;
use crate::app::App;

pub fn render_searched_artist(f: &mut Frame, content_chunk: &[Rect], app: &mut App) {
    f.render_widget(Clear, content_chunk[1]);
    let current_artist = &app.artist_names_search_results[app.artist_index];

    let artist_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from(format!("{}", current_artist)))
        .border_style(if app.searched_artist_selected {
            Style::default().fg(app.main_border_color)
        } else {
            Style::default()
        })
        .style(Style::default().bg(app.main_background_color));

    let track_table = searched_track_table_for_artist_ui(
        app.selected_artist_tracks_names.clone(),
        app.selected_artist_track_album_names.clone(),
        app.selected_artist_tracks_duration.clone(),
        artist_block,
        app.main_highlight_color.clone(),
        app.main_background_color.clone(),
    );

    f.render_widget(Clear, content_chunk[1]);

    f.render_stateful_widget(
        track_table,
        content_chunk[1],
        &mut app.searched_artist_state,
    );
}
