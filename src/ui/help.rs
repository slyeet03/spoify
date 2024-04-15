use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{block::Title, Block, Borders},
    Frame,
};

use crate::app::App;

pub fn render_default_help(f: &mut Frame, header_chunk: &[Rect], app: &mut App) {
    let help_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Help"))
        .border_style(Style::default())
        .style(Style::default().bg(app.background_color));

    f.render_widget(help_block, header_chunk[1]);
}
