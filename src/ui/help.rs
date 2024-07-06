use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{block::Title, Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use crate::structs::Key;

use super::util::help_table_ui;

/// Renders the default help block
pub fn render_default_help(f: &mut Frame, header_chunk: &[Rect], app: &mut App) {
    let help_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Help"))
        .border_style(Style::default())
        .style(Style::default().bg(app.help_background_color));

    let mut help_panel_vec = Vec::new();
    let _var = help_panel_vec;

    help_panel_vec = vec![Line::from(vec![Span::raw("Type ?")])];

    let help_panel = Paragraph::new(help_panel_vec)
        .wrap(Wrap { trim: true })
        .block(help_block);

    f.render_widget(help_panel, header_chunk[1]);
}

/// Renders the full help UI with a table of available commands and keybindings
pub fn render_help(f: &mut Frame, app: &mut App, key: &mut Key) {
    let help_label = format!("Help (press {} to go back)", key.help_key);
    f.render_widget(Clear, f.size());

    let help_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from(help_label))
        .border_style(Style::new().fg(app.help_border_color))
        .style(Style::default().bg(app.help_background_color));

    let help_table = help_table_ui(
        key.tasks.clone(),
        key.first_keys.clone(),
        help_block,
        app.help_highlight_color.clone(),
        app.help_background_color.clone(),
    );

    f.render_widget(help_table, f.size());
}
