use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{block::Title, Block, Borders, Clear, List},
    Frame,
};

use crate::app::App;

use super::util::convert_to_list;

pub fn render_lyrics(f: &mut Frame, content_chunk: &[Rect], app: &mut App) {
    let song_name = app.current_playing_name.clone();

    let lyric_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from(song_name))
        .border_style(if app.lyrics_selected {
            Style::default().fg(app.border_color)
        } else {
            Style::default()
        })
        .style(Style::default().bg(app.background_color));

    let lyric_item = convert_to_list(&app.lyrics);
    let lyric_item_list = List::new(lyric_item)
        .block(lyric_block.clone())
        .highlight_style(Style::default().fg(app.highlight_color));

    f.render_widget(Clear, content_chunk[1]);
    f.render_stateful_widget(lyric_item_list, content_chunk[1], &mut app.lyric_state);
}
