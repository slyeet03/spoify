use ratatui::{
    layout::Constraint,
    style::{Color, Style, Stylize},
    widgets::{Block, Cell, Row, Table},
};

use crate::ui::util::format_duration;

pub fn searched_track_table_for_album_ui(
    names: Vec<String>,
    artist_names: Vec<String>,
    duration: Vec<i64>,
    block: Block,
    highlight_color: Color,
    background_color: Color,
    inactive_border_color: Color,
) -> Table {
    let tracks: Vec<(usize, String, String, String)> = names
        .iter()
        .enumerate()
        .zip(artist_names.iter())
        .zip(duration.iter().map(|d| format_duration(*d)))
        .map(|(((index, name), artist), duration)| {
            (index + 1, name.clone(), artist.clone(), duration)
        })
        .collect();

    let table = Table::new(
        tracks
            .iter()
            .map(|(index, name, artist, duration)| {
                Row::new(vec![
                    Cell::from(format!("{}", index)),
                    Cell::from(name.clone()),
                    Cell::from(artist.clone()),
                    Cell::from(duration.clone()),
                ])
            })
            .collect::<Vec<_>>(),
        [
            Constraint::Percentage(5),
            Constraint::Percentage(57),
            Constraint::Percentage(23),
            Constraint::Percentage(15),
        ],
    )
    .header(
        Row::new(vec![
            Cell::from("#"),
            Cell::from("Title"),
            Cell::from("Artist"),
            Cell::from("Duration"),
        ])
        .bold(),
    )
    .block(block.clone())
    .highlight_style(Style::default().fg(highlight_color))
    .style(
        Style::default()
            .bg(background_color)
            .fg(inactive_border_color),
    );

    table
}

pub fn searched_track_table_for_artist_ui(
    names: Vec<String>,
    album_names: Vec<String>,
    duration: Vec<i64>,
    block: Block,
    highlight_color: Color,
    background_color: Color,
    inactive_border_color: Color,
) -> Table {
    let tracks: Vec<(usize, String, String, String)> = names
        .iter()
        .enumerate()
        .zip(album_names.iter())
        .zip(duration.iter().map(|d| format_duration(*d)))
        .map(|(((index, name), album), duration)| {
            (index + 1, name.clone(), album.clone(), duration)
        })
        .collect();

    let table = Table::new(
        tracks
            .iter()
            .map(|(index, name, album, duration)| {
                Row::new(vec![
                    Cell::from(format!("{}", index)),
                    Cell::from(name.clone()),
                    Cell::from(album.clone()),
                    Cell::from(duration.clone()),
                ])
            })
            .collect::<Vec<_>>(),
        [
            Constraint::Percentage(5),
            Constraint::Percentage(57),
            Constraint::Percentage(23),
            Constraint::Percentage(15),
        ],
    )
    .header(
        Row::new(vec![
            Cell::from("#"),
            Cell::from("Title"),
            Cell::from("Album"),
            Cell::from("Duration"),
        ])
        .bold(),
    )
    .block(block.clone())
    .highlight_style(Style::default().fg(highlight_color))
    .style(
        Style::default()
            .bg(background_color)
            .fg(inactive_border_color),
    );

    table
}
