use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{block::Title, Block, Borders, Clear, List},
    Frame,
};

use crate::{app::App, structs::Themes};

use super::util::{convert_to_list, new_release_table_ui};

/// Renders a simplified list of new release album names
pub fn render_default_new_releases(
    f: &mut Frame,
    content_sub_chunk: &[Rect],
    app: &mut App,
    theme: &mut Themes,
) {
    let new_release_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("New Releases"))
        .style(Style::default().bg(theme.new_release_background_color));

    let new_releases_name = convert_to_list(&app.new_release_name);
    let new_releases_list = List::new(new_releases_name)
        .block(new_release_block.clone())
        .highlight_style(Style::default().fg(theme.new_release_highlight_color));

    f.render_widget(new_releases_list, content_sub_chunk[1]);
}

/// Renders the full new releases view, including the tracks and other details for the selected album.
pub fn render_new_releases(
    f: &mut Frame,
    content_sub_chunk: &[Rect],
    content_chunk: &[Rect],
    app: &mut App,
    theme: &mut Themes,
) {
    let current_new_release_name = (&app.current_new_release_album).to_string();

    let new_release_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("New Releases"))
        .border_style(Style::default().fg(theme.new_release_border_color))
        .style(Style::default().bg(theme.new_release_background_color));

    let current_new_release_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from(current_new_release_name))
        .border_style(if app.new_release_album_selected {
            Style::default().fg(theme.new_release_border_color)
        } else {
            Style::default()
        })
        .style(Style::default().bg(theme.new_release_background_color));

    let new_releases_name = convert_to_list(&app.new_release_name);
    let new_releases_list = List::new(new_releases_name)
        .block(new_release_block.clone())
        .highlight_style(Style::default().fg(theme.new_release_highlight_color));

    f.render_widget(Clear, content_sub_chunk[1]);
    f.render_stateful_widget(
        new_releases_list,
        content_sub_chunk[1],
        &mut app.new_release_state,
    );

    // Conditionally render details for the selected new release album.
    if app.new_release_display {
        f.render_widget(Clear, content_chunk[1]);

        let new_release_tracks_table = new_release_table_ui(
            app.new_release_track_names.clone(),
            app.new_release_artist_names.clone(),
            app.new_release_durations_ms.clone(),
            current_new_release_block,
            theme.new_release_highlight_color.clone(),
            theme.new_release_background_color.clone(),
        );
        f.render_widget(Clear, content_chunk[1]);

        f.render_stateful_widget(
            new_release_tracks_table,
            content_chunk[1],
            &mut app.new_release_album_state,
        );
    }
}
