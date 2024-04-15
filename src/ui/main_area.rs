use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{block::Title, Block, Borders},
    Frame,
};

use crate::app::App;

pub fn render_main_area(f: &mut Frame, content_chunk: &[Rect], app: &mut App) {
    let content_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Welcome!"))
        .style(Style::default().bg(app.background_color));

    f.render_widget(content_block, content_chunk[1]);
}
