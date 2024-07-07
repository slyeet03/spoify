use ratatui::{
    layout::Alignment,
    style::Style,
    widgets::{block::Title, Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::{app::App, structs::Key, structs::Themes};

/// Renders the error screen
pub fn render_error(f: &mut Frame, app: &mut App, key: &mut Key, theme: &mut Themes) {
    let error_label = format!("Error (press {} to exit the error screen)", key.error_key);
    f.render_widget(Clear, f.size());

    let error_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from(error_label))
        .border_style(Style::new().fg(theme.error_border_color))
        .style(Style::default().bg(theme.error_background_color));

    let error = Paragraph::new(app.error_text.clone())
        .block(error_block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(error, f.size());
}
