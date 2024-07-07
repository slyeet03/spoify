use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Gauge, Paragraph, Wrap},
    Frame,
};

use crate::{app::App, structs::Themes};

use super::{blank_screen::render_blank_screen, util::format_duration};

/// Renders the player UI section, including playback info, progress bar, and current media details
pub fn render_player_in_fullscreen(
    f: &mut Frame,
    player_fullscreen_layout: &[Rect],
    player_fullscreen_vertical_chunk: &[Rect],
    app: &mut App,
    theme: &mut Themes,
) {
    f.render_widget(Clear, f.size());

    let player_info_block = Block::default()
        .borders(Borders::TOP | Borders::RIGHT | Borders::LEFT)
        .title(format!(
            "{} ({} | Shuffle: {} | Repeat: {} | Volume: {}%)",
            app.playback_status,
            app.current_device_name,
            app.shuffle_status,
            app.repeat_status,
            app.current_device_volume
        ))
        .style(Style::default().bg(theme.player_background_color));

    let mut player_info_vec = Vec::new();
    let _var = player_info_vec;

    // Collect player information lines based on the media type (episode or song)
    if app.currently_playing_media_type == "episode" {
        player_info_vec = vec![Line::from(vec![
            Span::styled(
                app.current_playing_name.clone(),
                Style::default().fg(theme.player_highlight_color),
            ),
            Span::raw(", "),
            Span::styled(app.current_playing_album.clone(), Style::default()),
        ])];
    } else {
        player_info_vec = vec![Line::from(vec![
            Span::styled(
                app.current_playing_name.clone(),
                Style::default().fg(theme.player_highlight_color),
            ),
            Span::raw(", "),
            Span::styled(app.currently_playing_artist.clone(), Style::default()),
            Span::raw(" ("),
            Span::styled(app.current_playing_album.clone(), Style::default()),
            Span::raw(")"),
        ])];
    }

    let current_timestamp = format_duration(app.currrent_timestamp.round() as i64);
    let ending_timestamp = format_duration(app.ending_timestamp.round() as i64);

    let label = &format!("{}/{}", current_timestamp, ending_timestamp);

    if app.ending_timestamp == 0.0 {
        app.ending_timestamp = 1.0;
    }

    app.progress_bar_ratio = app.currrent_timestamp / app.ending_timestamp;

    let player_info = Paragraph::new(player_info_vec).wrap(Wrap { trim: true });

    let progress_bar = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::BOTTOM | Borders::RIGHT | Borders::LEFT)
                .style(Style::default().bg(theme.player_background_color)),
        )
        .gauge_style(
            Style::default()
                .fg(theme.player_highlight_color)
                .bg(theme.player_background_color),
        )
        .label(label)
        .ratio(app.progress_bar_ratio);

    render_blank_screen(f, player_fullscreen_vertical_chunk, theme);
    f.render_widget(player_info_block.clone(), player_fullscreen_layout[0]);
    f.render_widget(
        player_info,
        player_info_block.clone().inner(player_fullscreen_layout[0]),
    );
    f.render_widget(progress_bar, player_fullscreen_layout[1]);
}
