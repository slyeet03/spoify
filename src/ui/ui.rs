use crate::app::App;
use crate::enums::{InputMode, Menu};
use crate::spotify::search::convert_to_list;
use ratatui::prelude::*;
use ratatui::style::{Color, Style};
use ratatui::widgets::{block::*, Cell, Clear, Row, Table};
use ratatui::widgets::{Block, Borders, List, Paragraph};

pub fn render_frame(f: &mut Frame, selected_menu: Menu, app: &mut App) {
    //define library items
    let library_items = vec![
        String::from("Made For You"),
        String::from("Recently Played"),
        String::from("Liked Songs"),
        String::from("Albums"),
        String::from("Artists"),
        String::from("Podcasts"),
    ];
    let current_playlist_name = (&app.current_user_playlist).to_string();

    //creating all the ui blocks
    let search_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Search"))
        .style(Style::default().bg(app.background_color));
    let library_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Library"))
        .style(Style::default().bg(app.background_color));
    let playlist_block_user = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Playlist"))
        .style(Style::default().bg(app.background_color));
    let player_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from(
            "".to_owned()
                + &app.playback_status
                + " | "
                + &app.current_device_name
                + " | Volume: "
                + &app.current_device_volume
                + "%",
        ))
        .style(Style::default().bg(app.background_color));
    let content_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Welcome!"))
        .style(Style::default().bg(app.background_color));

    let album_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Albums"))
        .border_style(if app.selected_album {
            Style::default().fg(app.border_color)
        } else {
            Style::default()
        })
        .style(Style::default().bg(app.background_color));
    let artist_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Artists"))
        .border_style(if app.selected_artist {
            Style::default().fg(app.border_color)
        } else {
            Style::default()
        })
        .style(Style::default().bg(app.background_color));
    let song_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Songs"))
        .border_style(if app.selected_track {
            Style::default().fg(app.border_color)
        } else {
            Style::default()
        })
        .style(Style::default().bg(app.background_color));
    let playlist_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Playlists"))
        .border_style(if app.selected_playlist {
            Style::default().fg(app.border_color)
        } else {
            Style::default()
        })
        .style(Style::default().bg(app.background_color));

    let user_playlist_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from(current_playlist_name))
        .border_style(if app.user_playlist_tracks_selected {
            Style::default().fg(app.border_color)
        } else {
            Style::default()
        })
        .style(Style::default().bg(app.background_color));
    let liked_song_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Liked Songs"))
        .border_style(if app.liked_songs_selected {
            Style::default().fg(app.border_color)
        } else {
            Style::default()
        })
        .style(Style::default().bg(app.background_color));

    let recently_played_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Recently Played"))
        .border_style(if app.recently_played_selected {
            Style::default().fg(app.border_color)
        } else {
            Style::default()
        })
        .style(Style::default().bg(app.background_color));

    let user_album_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Albums"))
        .border_style(if app.user_album_selected {
            Style::default().fg(app.border_color)
        } else {
            Style::default()
        })
        .style(Style::default().bg(app.background_color));

    let podcast_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Podcasts"))
        .border_style(if app.podcast_selected {
            Style::default().fg(app.border_color)
        } else {
            Style::default()
        })
        .style(Style::default().bg(app.background_color));

    let user_artist_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Podcasts"))
        .border_style(if app.user_artist_selected {
            Style::default().fg(app.border_color)
        } else {
            Style::default()
        })
        .style(Style::default().bg(app.background_color));

    let search_input = Paragraph::new(app.input.as_str())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(app.border_color),
            InputMode::SearchResults => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL).title("Search"))
        .style(Style::default().bg(app.background_color));

    //list widget for library items
    let library_list = List::new(library_items.clone()).block(library_block);
    let user_playlist_names = convert_to_list(&app.user_playlist_names);
    let user_playlist_list = List::new(user_playlist_names).block(playlist_block_user.clone());
    let size = f.size();
    // main display layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(8),
            Constraint::Percentage(72),
            Constraint::Percentage(20),
        ])
        .split(size);
    // search layout
    let header_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .split(chunks[0]);
    // library, playlist and main content display layout
    let content_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(chunks[1]);
    // library and playlist layout
    let content_sub_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(content_chunk[0]);

    let main_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(content_chunk[1]);

    let main_chunk_upper = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunk[0]);
    let main_chunk_lower = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunk[1]);

    //rendering the default ui
    f.render_widget(search_block, header_chunk[0]);
    f.render_widget(&library_list, content_sub_chunk[0]);
    f.render_widget(playlist_block_user, content_sub_chunk[1]);
    f.render_widget(player_block, chunks[2]);
    f.render_widget(content_block, content_chunk[1]);
    f.render_widget(user_playlist_list, content_sub_chunk[1]);
    //rendering different sections based on the selected menu
    match selected_menu {
        Menu::Default => {}
        Menu::Main => {
            // add tabbing fn thru artist,album,songs,playlists
            //add menu nav inside those blocks
        }
        Menu::Library => {
            let library_block = Block::default()
                .borders(Borders::ALL)
                .title(Title::from("Library"))
                .border_style(Style::new().fg(app.border_color))
                .style(Style::default().bg(app.background_color));

            let library_items = vec![
                String::from("Made For You"),
                String::from("Recently Played"),
                String::from("Liked Songs"),
                String::from("Albums"),
                String::from("Artists"),
                String::from("Podcasts"),
            ];
            //rendering currently selected menu
            let library_list = List::new(library_items)
                .block(library_block)
                .highlight_style(Style::default().fg(app.highlight_color));

            f.render_stateful_widget(library_list, content_sub_chunk[0], &mut app.library_state);

            if app.liked_song_display {
                f.render_widget(Clear, content_chunk[1]);

                let liked_songs_table = track_table_ui(
                    app.liked_song_names.clone(),
                    app.liked_song_artist_names.clone(),
                    app.liked_song_album_names.clone(),
                    app.liked_song_duration.clone(),
                    liked_song_block,
                    app.highlight_color.clone(),
                    app.background_color.clone(),
                );

                f.render_widget(Clear, content_chunk[1]);

                f.render_stateful_widget(
                    liked_songs_table,
                    content_chunk[1],
                    &mut app.liked_songs_state,
                );
            }

            if app.recently_played_display {
                f.render_widget(Clear, content_chunk[1]);

                let recently_played_table = track_table_ui(
                    app.recently_played_names.clone(),
                    app.recently_played_artist_names.clone(),
                    app.recently_played_album_names.clone(),
                    app.recently_played_duration.clone(),
                    recently_played_block,
                    app.highlight_color.clone(),
                    app.background_color.clone(),
                );

                f.render_widget(Clear, content_chunk[1]);

                f.render_stateful_widget(
                    recently_played_table,
                    content_chunk[1],
                    &mut app.recently_played_state,
                );
            }

            if app.podcast_display {
                f.render_widget(Clear, content_chunk[1]);

                let podcast_table = podcast_table_ui(
                    app.podcast_names.clone(),
                    app.podcast_publisher.clone(),
                    podcast_block,
                    app.highlight_color.clone(),
                    app.background_color.clone(),
                );

                f.render_widget(Clear, content_chunk[1]);

                f.render_stateful_widget(podcast_table, content_chunk[1], &mut app.podcast_state);
            }

            if app.user_artist_display {
                f.render_widget(Clear, content_chunk[1]);

                let artist_table = artist_table_ui(
                    app.user_artist_names.clone(),
                    user_artist_block,
                    app.highlight_color.clone(),
                    app.background_color.clone(),
                );

                f.render_widget(Clear, content_chunk[1]);

                f.render_stateful_widget(
                    artist_table,
                    content_chunk[1],
                    &mut app.user_artist_state,
                );
            }

            if app.user_album_display {
                f.render_widget(Clear, content_chunk[1]);

                let user_album_table = album_table_ui(
                    app.user_album_names.clone(),
                    app.user_album_artist_names.clone(),
                    app.user_album_tracks.clone(),
                    user_album_block,
                    app.highlight_color.clone(),
                    app.background_color.clone(),
                );

                f.render_widget(Clear, content_chunk[1]);

                f.render_stateful_widget(
                    user_album_table,
                    content_chunk[1],
                    &mut app.user_album_state,
                );
            }
        }
        Menu::Playlists => {
            let playlist_block_user = Block::default()
                .borders(Borders::ALL)
                .title(Title::from("Playlist"))
                .border_style(Style::new().fg(app.border_color))
                .style(Style::default().bg(app.background_color));

            let user_playlist_names = convert_to_list(&app.user_playlist_names);
            let user_playlist_list = List::new(user_playlist_names)
                .block(playlist_block_user.clone())
                .highlight_style(Style::default().fg(app.highlight_color));
            f.render_widget(Clear, content_sub_chunk[1]);
            f.render_stateful_widget(
                user_playlist_list,
                content_sub_chunk[1],
                &mut app.user_playlist_state,
            );
            if app.user_playlist_display {
                f.render_widget(Clear, content_chunk[1]);
                let user_playlist_tracks_table = track_table_ui(
                    app.user_playlist_track_names.clone(),
                    app.user_playlist_artist_names.clone(),
                    app.user_playlist_album_names.clone(),
                    app.user_playlist_track_duration.clone(),
                    user_playlist_block,
                    app.highlight_color.clone(),
                    app.background_color.clone(),
                );
                f.render_widget(Clear, content_chunk[1]);

                f.render_stateful_widget(
                    user_playlist_tracks_table,
                    content_chunk[1],
                    &mut app.user_playlist_tracks_state,
                );
            }
        }
        Menu::Search => {
            let search_block = Block::default()
                .borders(Borders::ALL)
                .title(Title::from("Search"))
                .border_style(Style::new().fg(app.border_color))
                .style(Style::default().bg(app.background_color));

            f.render_widget(search_block, header_chunk[0]);

            match app.input_mode {
                InputMode::Normal => {}
                InputMode::Editing => {
                    f.render_widget(search_input, header_chunk[0]);
                    f.set_cursor(
                        header_chunk[0].x + app.cursor_position as u16 + 1,
                        header_chunk[0].y + 1,
                    );
                }
                InputMode::SearchResults if app.search_results_rendered => {
                    let album_names_list = convert_to_list(&app.album_names);
                    let track_names_list = convert_to_list(&app.track_names);
                    let artist_names_list = convert_to_list(&app.artist_names);
                    let playlist_names_list = convert_to_list(&app.playlist_names);

                    let album_list = List::new(album_names_list)
                        .block(album_block.clone())
                        .highlight_style(Style::default().fg(app.highlight_color));

                    let song_list = List::new(track_names_list)
                        .block(song_block.clone())
                        .highlight_style(Style::default().fg(app.highlight_color));

                    let playlist_list = List::new(playlist_names_list)
                        .block(playlist_block.clone())
                        .highlight_style(Style::default().fg(app.highlight_color));

                    let artist_list = List::new(artist_names_list)
                        .block(artist_block.clone())
                        .highlight_style(Style::default().fg(app.highlight_color));

                    f.render_stateful_widget(song_list, main_chunk_upper[0], &mut app.track_state);
                    f.render_stateful_widget(
                        artist_list,
                        main_chunk_upper[1],
                        &mut app.artist_state,
                    );
                    f.render_stateful_widget(album_list, main_chunk_lower[0], &mut app.album_state);
                    f.render_stateful_widget(
                        playlist_list,
                        main_chunk_lower[1],
                        &mut app.playlist_state,
                    );
                }
                _ => {}
            }
        }
    }
}

fn format_duration(duration: i64) -> String {
    let minutes = duration / 60000;
    let seconds = (duration % 60000) / 1000;
    format!("{}:{:02}", minutes, seconds)
}

fn track_table_ui(
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

fn album_table_ui(
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

fn podcast_table_ui(
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

fn artist_table_ui(
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
