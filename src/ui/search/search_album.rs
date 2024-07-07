use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{block::Title, Block, Borders, Clear},
    Frame,
};

use crate::{app::App, structs::Themes};

use super::util::searched_track_table_for_album_ui;

pub fn render_searched_album(
    f: &mut Frame,
    content_chunk: &[Rect],
    app: &mut App,
    theme: &mut Themes,
) {
    f.render_widget(Clear, content_chunk[1]);
    let current_album = &app.album_names_search_results[app.album_index];

    let album_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from(current_album.to_string()))
        .border_style(if app.searched_album_selected {
            Style::default().fg(theme.main_border_color)
        } else {
            Style::default()
        })
        .style(Style::default().bg(theme.main_background_color));

    let track_table = searched_track_table_for_album_ui(
        app.selected_album_tracks_names.clone(),
        app.selected_album_tracks_artists.clone(),
        app.selected_album_tracks_duration.clone(),
        album_block,
        theme.main_highlight_color,
        theme.main_background_color,
    );

    f.render_widget(Clear, content_chunk[1]);

    f.render_stateful_widget(track_table, content_chunk[1], &mut app.searched_album_state);
}
