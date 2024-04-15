use ratatui::{
    layout::Constraint,
    style::{Color, Style, Stylize},
    widgets::{Block, Cell, Row, Table},
};

use ratatui::widgets::ListItem;

// Helper function to convert a list of strings to a vector of ListItems
pub fn convert_to_list<'a>(names: &'a [String]) -> Vec<ListItem<'a>> {
    let mut search_results = Vec::new();

    for name in names {
        search_results.push(ListItem::new(format!("{}", name)));
    }
    search_results
}

pub fn format_duration(duration: i64) -> String {
    let minutes = duration / 60000;
    let seconds = (duration % 60000) / 1000;
    format!("{}:{:02}", minutes, seconds)
}

pub fn track_table_ui(
    names: Vec<String>,
    artist_names: Vec<String>,
    album_names: Vec<String>,
    duration: Vec<i64>,
    block: Block,
    highlight_color: Color,
    background_color: Color,
) -> Table {
    let tracks: Vec<(usize, String, String, String, String)> = names
        .iter()
        .enumerate()
        .zip(artist_names.iter())
        .zip(album_names.iter())
        .zip(duration.iter().map(|d| format_duration(*d)))
        .map(|((((index, name), artist), album), duration)| {
            (
                index + 1,
                name.clone(),
                artist.clone(),
                album.clone(),
                duration,
            )
        })
        .collect();

    let table = Table::new(
        tracks
            .iter()
            .map(|(index, name, artist, albums, duration)| {
                Row::new(vec![
                    Cell::from(format!("{}", index)),
                    Cell::from(name.clone()),
                    Cell::from(artist.clone()),
                    Cell::from(albums.clone()),
                    Cell::from(duration.clone()),
                ])
            })
            .collect::<Vec<_>>(),
        [
            Constraint::Percentage(3),
            Constraint::Percentage(37),
            Constraint::Percentage(20),
            Constraint::Percentage(30),
            Constraint::Percentage(10),
        ],
    )
    .header(
        Row::new(vec![
            Cell::from("#"),
            Cell::from("Title"),
            Cell::from("Artist"),
            Cell::from("Album"),
            Cell::from("Duration"),
        ])
        .bold(),
    )
    .block(block.clone())
    .highlight_style(Style::default().fg(highlight_color))
    .style(Style::default().bg(background_color));

    table
}

pub fn album_table_ui(
    names: Vec<String>,
    artist_names: Vec<String>,
    tracks: Vec<usize>,
    block: Block,
    highlight_color: Color,
    background_color: Color,
) -> Table {
    let albums: Vec<(usize, String, String, usize)> = names
        .iter()
        .enumerate()
        .zip(artist_names.iter())
        .zip(tracks.iter())
        .map(|(((index, name), artist), track)| {
            (index + 1, name.clone(), artist.clone(), track.clone())
        })
        .collect();

    let table = Table::new(
        albums
            .iter()
            .map(|(index, name, artist, track)| {
                Row::new(vec![
                    Cell::from(format!("{}", index)),
                    Cell::from(name.clone()),
                    Cell::from(artist.clone()),
                    Cell::from(format!("{}", track)),
                ])
            })
            .collect::<Vec<_>>(),
        [
            Constraint::Percentage(10),
            Constraint::Percentage(45),
            Constraint::Percentage(35),
            Constraint::Percentage(10),
        ],
    )
    .header(
        Row::new(vec![
            Cell::from("#"),
            Cell::from("Title"),
            Cell::from("Artist"),
            Cell::from("Tracks"),
        ])
        .bold(),
    )
    .block(block.clone())
    .highlight_style(Style::default().fg(highlight_color))
    .style(Style::default().bg(background_color));

    table
}

pub fn podcast_table_ui(
    names: Vec<String>,
    publisher: Vec<String>,
    block: Block,
    highlight_color: Color,
    background_color: Color,
) -> Table {
    let podcasts: Vec<(usize, String, String)> = names
        .iter()
        .enumerate()
        .zip(publisher.iter())
        .map(|((index, name), publisher)| (index + 1, name.clone(), publisher.clone()))
        .collect();

    let table = Table::new(
        podcasts
            .iter()
            .map(|(index, name, publisher)| {
                Row::new(vec![
                    Cell::from(format!("{}", index)),
                    Cell::from(name.clone()),
                    Cell::from(publisher.clone()),
                ])
            })
            .collect::<Vec<_>>(),
        [
            Constraint::Percentage(10),
            Constraint::Percentage(50),
            Constraint::Percentage(40),
        ],
    )
    .header(
        Row::new(vec![
            Cell::from("#"),
            Cell::from("Title"),
            Cell::from("Publisher"),
        ])
        .bold(),
    )
    .block(block.clone())
    .highlight_style(Style::default().fg(highlight_color))
    .style(Style::default().bg(background_color));

    table
}

pub fn artist_table_ui(
    names: Vec<String>,
    block: Block,
    highlight_color: Color,
    background_color: Color,
) -> Table {
    let podcasts: Vec<(usize, String)> = names
        .iter()
        .enumerate()
        .map(|(index, name)| (index + 1, name.clone()))
        .collect();

    let table = Table::new(
        podcasts
            .iter()
            .map(|(index, name)| {
                Row::new(vec![
                    Cell::from(format!("{}", index)),
                    Cell::from(name.clone()),
                ])
            })
            .collect::<Vec<_>>(),
        [Constraint::Percentage(10), Constraint::Percentage(90)],
    )
    .header(Row::new(vec![Cell::from("#"), Cell::from("Title")]).bold())
    .block(block.clone())
    .highlight_style(Style::default().fg(highlight_color))
    .style(Style::default().bg(background_color));

    table
}

pub fn help_table_ui(
    task: Vec<String>,
    first_key: Vec<String>,
    block: Block,
    highlight_color: Color,
    background_color: Color,
) -> Table {
    let items: Vec<(usize, String, String)> = task
        .iter()
        .enumerate()
        .zip(first_key.iter())
        .map(|((index, task), first_key)| (index + 1, task.clone(), first_key.clone()))
        .collect();

    let table = Table::new(
        items
            .iter()
            .map(|(index, task, first_key)| {
                Row::new(vec![
                    Cell::from(format!("{}", index)),
                    Cell::from(task.clone()),
                    Cell::from(first_key.clone()),
                ])
            })
            .collect::<Vec<_>>(),
        [
            Constraint::Percentage(10),
            Constraint::Percentage(50),
            Constraint::Percentage(40),
        ],
    )
    .header(
        Row::new(vec![
            Cell::from("#"),
            Cell::from("Task"),
            Cell::from("First Key"),
        ])
        .bold(),
    )
    .block(block.clone())
    .highlight_style(Style::default().fg(highlight_color))
    .style(Style::default().bg(background_color));

    table
}
