extern crate indoc;

use indoc::indoc;
use itertools::izip;
use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{block::Title, Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the main content area of the application, currently displaying a welcome message and logo
pub fn render_main_area(f: &mut Frame, content_chunk: &[Rect], app: &mut App) {
    let content_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Welcome!"))
        .style(Style::default().bg(app.main_background_color));

    let logo = Paragraph::new(logo())
        .block(content_block)
        .style(Style::default().bg(app.main_background_color));

    f.render_widget(logo, content_chunk[1]);
}

fn logo() -> String {
    let spoify = indoc! {r"
                     _ ____     
   _________  ____  (_) __/_  __
  / ___/ __ \/ __ \/ / /_/ / / /
 (__  ) /_/ / /_/ / / __/ /_/ / 
/____/ .___/\____/_/_/  \__, /  
    /_/                /____/   
    "};

    izip!(spoify.lines())
        .map(|spoify| format!("{spoify:5}"))
        .collect::<Vec<_>>()
        .join("\n")
}
