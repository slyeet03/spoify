extern crate indoc;

use indoc::indoc;
use itertools::izip;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style, Stylize},
    widgets::{block::Title, Block, Borders, Clear, List, Padding, Paragraph},
    Frame,
};

use crate::{app::App, structs::Themes};

use super::util::convert_to_list;

/// Renders the main content area of the application, currently displaying a welcome message and logo
pub fn render_main_area(
    f: &mut Frame,
    content_chunk: &[Rect],
    front_chunk: &[Rect],
    app: &mut App,
    theme: &mut Themes,
) {
    let content_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Welcome!"))
        .style(Style::default().bg(theme.main_background_color));

    let logo_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(theme.main_background_color));

    let logo = Paragraph::new(logo())
        .block(logo_block)
        .alignment(Alignment::Center)
        .style(Style::default().bg(theme.main_background_color));

    let stat_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(34),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(front_chunk[1]);

    let top_track_all_time_block = Block::default()
        .borders(Borders::LEFT)
        .padding(Padding::right(2))
        .title("Top Tracks(All Time)")
        .title_style(Style::default().add_modifier(Modifier::BOLD).underlined())
        .style(Style::default().bg(theme.main_background_color));

    let top_track_6_months_block = Block::default()
        .borders(Borders::NONE)
        .padding(Padding::right(2))
        .title("Top Tracks(6 Months)")
        .title_style(Style::default().underlined().add_modifier(Modifier::BOLD))
        .style(Style::default().bg(theme.main_background_color));

    let top_track_4_weeks_block = Block::default()
        .borders(Borders::NONE)
        .padding(Padding::right(2))
        .title("Top Tracks(4 Weeks)")
        .title_style(Style::default().underlined().add_modifier(Modifier::BOLD))
        .style(Style::default().bg(theme.main_background_color));

    let top_tracks_all_time_names_clone = app.top_tracks_all_time_names.clone();
    let top_track_all_time_names = convert_to_list(&top_tracks_all_time_names_clone);
    let top_track_all_time_list =
        List::new(top_track_all_time_names).block(top_track_all_time_block.clone());

    let top_tracks_6_months_names_clone = app.top_tracks_6_months_names.clone();
    let top_track_6_months_names = convert_to_list(&top_tracks_6_months_names_clone);
    let top_track_6_months_list =
        List::new(top_track_6_months_names).block(top_track_6_months_block.clone());

    let top_tracks_4_weeks_names_clone = app.top_tracks_4_weeks_names.clone();
    let top_track_4_weeks_names = convert_to_list(&top_tracks_4_weeks_names_clone);
    let top_track_4_weeks_list =
        List::new(top_track_4_weeks_names).block(top_track_4_weeks_block.clone());

    f.render_widget(Clear, content_chunk[1]);
    f.render_widget(top_track_all_time_list, stat_chunk[0]);
    f.render_widget(top_track_6_months_list, stat_chunk[1]);
    f.render_widget(top_track_4_weeks_list, stat_chunk[2]);
    f.render_widget(logo, front_chunk[0]);
    f.render_widget(content_block, content_chunk[1]);
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
