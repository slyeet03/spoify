use crate::structs::Search;
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
    search: &mut Search,
) {
    f.render_widget(Clear, content_chunk[1]);
    let current_playlist = &search.playlist_names_search_results[search.playlist_index];

    let playlist_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from(current_playlist.to_string()))
        .border_style(if search.searched_playlist_selected {
            Style::default().fg(theme.main_border_color)
        } else {
            Style::default().fg(theme.main_inactive_border_color)
        })
        .style(Style::default().bg(theme.main_background_color));

    let track_table = track_table_ui(
        search.selected_playlist_tracks_names.clone(),
        search.selected_playlist_tracks_artists.clone(),
        search.selected_playlist_tracks_albums.clone(),
        search.selected_playlist_tracks_duration.clone(),
        playlist_block,
        theme.main_highlight_color,
        theme.main_background_color,
        theme.main_inactive_border_color,
    );

    f.render_widget(Clear, content_chunk[1]);

    f.render_stateful_widget(
        track_table,
        content_chunk[1],
        &mut search.searched_playlist_state,
    );
}
