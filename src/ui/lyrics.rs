use ratatui::{
    layout::Rect,
    style::Style,
    text::{Span, Text},
    widgets::{block::Title, Block, Borders, Clear, List, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render_lyrics(
    f: &mut Frame,
    content_sub_chunk: &[Rect],
    content_chunk: &[Rect],
    app: &mut App,
) {
}
