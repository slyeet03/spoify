use ratatui::{
    layout::Alignment,
    style::Style,
    widgets::{block::Title, Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

/// Renders the error screen
pub fn render_error(f: &mut Frame, app: &mut App) {
    f.render_widget(Clear, f.size());

    let error_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Error (press e to exit the error screen)"))
        .border_style(Style::new().fg(app.error_border_color))
        .style(Style::default().bg(app.error_background_color));

    let error = Paragraph::new(app.error_text.clone())
        .block(error_block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(error, f.size());
}
