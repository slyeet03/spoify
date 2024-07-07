use ratatui::{
    style::Style,
    widgets::{block::Title, Block, Borders, Clear, List},
    Frame,
};

use crate::{
    app::App,
    structs::{Key, Themes},
    ui::util::convert_to_list,
};

/// Renders the error screen
pub fn render_add_track_to_playlist_screen(
    f: &mut Frame,
    app: &mut App,
    key: &mut Key,
    theme: &mut Themes,
) {
    let add_playlist_label = format!(
        "Select a playlist to add {} to (press {} to cancel)",
        app.track_added_to_playlist_name, key.exit_application_key
    );
    f.render_widget(Clear, f.size());

    let add_playlist_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from(add_playlist_label))
        .border_style(Style::new().fg(theme.playlist_border_color))
        .style(Style::default().bg(theme.playlist_background_color));

    let add_to_playlist_names = convert_to_list(&app.user_playlist_names);
    let add_to_playlist_list = List::new(add_to_playlist_names)
        .block(add_playlist_block.clone())
        .highlight_style(Style::default().fg(theme.playlist_highlight_color));

    f.render_widget(Clear, f.size());

    f.render_stateful_widget(
        add_to_playlist_list,
        f.size(),
        &mut app.add_track_to_playlist_state,
    );
}
