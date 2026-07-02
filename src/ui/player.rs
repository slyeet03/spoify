use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
    Frame,
};

use crate::{app::App, structs::{Themes, UserCurrentlyPlaying}};

use super::util::format_duration;

/// Renders the player UI section, including playback info, progress bar, and current media details
pub fn render_player(f: &mut Frame, player_layout: &[Rect], app: &mut App, theme: &mut Themes, currentlyplaying: &mut UserCurrentlyPlaying) {
    let player_info_block = Block::default()
        .borders(Borders::TOP | Borders::RIGHT | Borders::LEFT)
        .title(format!(
            "{} ({} | Shuffle: {} | Repeat: {} | Volume: {}%)",
            currentlyplaying.playback_status,
            currentlyplaying.current_device_name,
            currentlyplaying.shuffle_status,
            currentlyplaying.repeat_status,
            currentlyplaying.current_device_volume
        ))
        .style(
            Style::default()
                .bg(theme.player_background_color)
                .fg(theme.player_inactive_border_color),
        );

    let mut player_info_vec = Vec::new();
    let _var = player_info_vec;

    // Collect player information lines based on the media type (episode or song)
    if currentlyplaying.media_type == "episode" {
        player_info_vec = vec![Line::from(vec![
            Span::styled(
                currentlyplaying.name.clone(),
                Style::default().fg(theme.player_highlight_color),
            ),
            Span::raw(", "),
            Span::styled(currentlyplaying.album.clone(), Style::default()),
        ])];
    } else {
        player_info_vec = vec![Line::from(vec![
            Span::styled(
                currentlyplaying.name.clone(),
                Style::default().fg(theme.player_highlight_color),
            ),
            Span::raw(", "),
            Span::styled(currentlyplaying.artist.clone(), Style::default()),
            Span::raw(" ("),
            Span::styled(currentlyplaying.album.clone(), Style::default()),
            Span::raw(")"),
        ])];
    }

    let current_timestamp = format_duration(currentlyplaying.current_timestamp.round() as i64);
    let ending_timestamp = format_duration(currentlyplaying.ending_timestamp.round() as i64);

    let label = &format!("{}/{}", current_timestamp, ending_timestamp);

    if currentlyplaying.ending_timestamp == 0.0 {
        currentlyplaying.ending_timestamp = 1.0;
    }

    currentlyplaying.progress_bar_ratio = currentlyplaying.current_timestamp / currentlyplaying.ending_timestamp;

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
        .ratio(currentlyplaying.progress_bar_ratio);

    f.render_widget(player_info_block.clone(), player_layout[0]);
    f.render_widget(
        player_info,
        player_info_block.clone().inner(player_layout[0]),
    );
    f.render_widget(progress_bar, player_layout[1]);
}
