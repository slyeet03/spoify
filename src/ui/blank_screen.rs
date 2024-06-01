use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, Clear},
    Frame,
};

use crate::app::App;

/// Renders the error screen
pub fn render_blank_screen(f: &mut Frame, player_fullscreen_chunk: &[Rect], app: &mut App) {
    f.render_widget(Clear, f.size());
    let upper_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(app.main_background_color));

    let lower_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(app.main_background_color));

    f.render_widget(upper_block, player_fullscreen_chunk[0]);
    f.render_widget(lower_block, player_fullscreen_chunk[2]);
}
