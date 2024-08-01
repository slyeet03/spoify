use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, Clear},
    Frame,
};

use crate::structs::Themes;

/// Renders a blank screen
pub fn render_blank_screen(
    f: &mut Frame,
    player_fullscreen_vertical_chunk: &[Rect],
    theme: &mut Themes,
) {
    f.render_widget(Clear, f.size());
    let upper_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(theme.main_background_color));

    let lower_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(theme.main_background_color));

    f.render_widget(upper_block.clone(), player_fullscreen_vertical_chunk[0]);
    f.render_widget(lower_block.clone(), player_fullscreen_vertical_chunk[2]);
}
