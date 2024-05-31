use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{block::Title, Block, Borders, Clear, List},
    Frame,
};

use crate::app::App;
use crate::ui::util::{album_table_ui, artist_table_ui, podcast_table_ui, track_table_ui};

use super::{
    search::util::{searched_track_table_for_album_ui, searched_track_table_for_artist_ui},
    util::made_fy_table_ui,
};

/// Renders the library view of the application, including the list of library sections and content for the selected section
pub fn render_library(
    f: &mut Frame,
    content_sub_chunk: &[Rect],
    content_chunk: &[Rect],
    app: &mut App,
) {
    let library_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Library"))
        .border_style(Style::new().fg(app.library_border_color))
        .style(Style::default().bg(app.library_background_color));

    let liked_song_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Liked Songs"))
        .border_style(if app.liked_songs_selected {
            Style::default().fg(app.main_border_color)
        } else {
            Style::default()
        })
        .style(Style::default().bg(app.main_background_color));

    let recently_played_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Recently Played"))
        .border_style(if app.recently_played_selected {
            Style::default().fg(app.main_border_color)
        } else {
            Style::default()
        })
        .style(Style::default().bg(app.main_background_color));

    let user_album_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Albums"))
        .border_style(if app.user_album_selected {
            Style::default().fg(app.main_border_color)
        } else {
            Style::default()
        })
        .style(Style::default().bg(app.main_background_color));

    let podcast_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Podcasts"))
        .border_style(if app.podcast_selected {
            Style::default().fg(app.main_border_color)
        } else {
            Style::default()
        })
        .style(Style::default().bg(app.main_background_color));

    let user_artist_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Artists"))
        .border_style(if app.user_artist_selected {
            Style::default().fg(app.main_border_color)
        } else {
            Style::default()
        })
        .style(Style::default().bg(app.main_background_color));

    let made_fy_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Made For You"))
        .border_style(if app.made_fy_selected {
            Style::default().fg(app.main_border_color)
        } else {
            Style::default()
        })
        .style(Style::default().bg(app.main_background_color));

    let library_items = vec![
        String::from("Made For You"),
        String::from("Recently Played"),
        String::from("Liked Songs"),
        String::from("Albums"),
        String::from("Artists"),
        String::from("Podcasts"),
    ];
    // Rendering currently selected menu
    let library_list = List::new(library_items)
        .block(library_block)
        .highlight_style(Style::default().fg(app.library_highlight_color));

    f.render_stateful_widget(library_list, content_sub_chunk[0], &mut app.library_state);

    // Render content for the selected library section based on app state.
    if app.made_fy_display {
        f.render_widget(Clear, content_chunk[1]);

        let made_fy_playlist_table = made_fy_table_ui(
            app.made_fy_playlist_names.clone(),
            app.made_fy_playlist_track_total.clone(),
            made_fy_block,
            app.main_highlight_color,
            app.main_background_color,
        );

        f.render_widget(Clear, content_chunk[1]);

        f.render_stateful_widget(
            made_fy_playlist_table,
            content_chunk[1],
            &mut app.made_fy_state,
        );
    }

    if app.made_fy_track_display {
        let made_fy_track_label = format!("{}", app.made_fy_playlist_names[app.made_fy_index]);

        let made_fy_track_block = Block::default()
            .borders(Borders::ALL)
            .title(Title::from(made_fy_track_label))
            .border_style(if app.made_fy_track_selected {
                Style::default().fg(app.main_border_color)
            } else {
                Style::default()
            })
            .style(Style::default().bg(app.main_background_color));

        f.render_widget(Clear, content_chunk[1]);

        let made_fy_track_table = track_table_ui(
            app.made_fy_track_names.clone(),
            app.made_fy_artist_names.clone(),
            app.made_fy_album_names.clone(),
            app.made_fy_track_duration.clone(),
            made_fy_track_block,
            app.main_highlight_color.clone(),
            app.main_background_color.clone(),
        );

        f.render_widget(Clear, content_chunk[1]);

        f.render_stateful_widget(
            made_fy_track_table,
            content_chunk[1],
            &mut app.made_fy_track_state,
        );
    }

    if app.liked_song_display {
        f.render_widget(Clear, content_chunk[1]);

        let liked_songs_table = track_table_ui(
            app.liked_song_names.clone(),
            app.liked_song_artist_names.clone(),
            app.liked_song_album_names.clone(),
            app.liked_song_duration.clone(),
            liked_song_block,
            app.main_highlight_color.clone(),
            app.main_background_color.clone(),
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
            app.main_highlight_color.clone(),
            app.main_background_color.clone(),
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
            app.main_highlight_color.clone(),
            app.main_background_color.clone(),
        );

        f.render_widget(Clear, content_chunk[1]);

        f.render_stateful_widget(podcast_table, content_chunk[1], &mut app.podcast_state);
    }

    if app.user_artist_display {
        f.render_widget(Clear, content_chunk[1]);

        let artist_table = artist_table_ui(
            app.user_artist_names.clone(),
            user_artist_block,
            app.main_highlight_color.clone(),
            app.main_background_color.clone(),
        );

        f.render_widget(Clear, content_chunk[1]);

        f.render_stateful_widget(artist_table, content_chunk[1], &mut app.user_artist_state);
    }

    if app.user_album_display {
        f.render_widget(Clear, content_chunk[1]);

        let user_album_table = album_table_ui(
            app.user_album_names.clone(),
            app.user_album_artist_names.clone(),
            app.user_album_tracks.clone(),
            user_album_block,
            app.main_highlight_color.clone(),
            app.main_background_color.clone(),
        );

        f.render_widget(Clear, content_chunk[1]);

        f.render_stateful_widget(
            user_album_table,
            content_chunk[1],
            &mut app.user_album_state,
        );
    }
    if app.user_album_track_display {
        let user_album_track_label = format!("{}", app.user_album_names[app.user_album_index]);

        let user_album_track_block = Block::default()
            .borders(Borders::ALL)
            .title(Title::from(user_album_track_label))
            .border_style(if app.user_album_track_selected {
                Style::default().fg(app.main_border_color)
            } else {
                Style::default()
            })
            .style(Style::default().bg(app.main_background_color));

        f.render_widget(Clear, content_chunk[1]);

        let user_album_track_table = searched_track_table_for_album_ui(
            app.user_album_track_names.clone(),
            app.user_album_track_artist.clone(),
            app.user_album_track_duration.clone(),
            user_album_track_block,
            app.main_highlight_color.clone(),
            app.main_background_color.clone(),
        );

        f.render_widget(Clear, content_chunk[1]);

        f.render_stateful_widget(
            user_album_track_table,
            content_chunk[1],
            &mut app.user_album_track_state,
        );
    }
    if app.user_artist_track_display {
        let user_artist_track_label = format!("{}", app.user_artist_names[app.user_artist_index]);

        let user_artist_track_block = Block::default()
            .borders(Borders::ALL)
            .title(Title::from(user_artist_track_label))
            .border_style(if app.user_artist_track_selected {
                Style::default().fg(app.main_border_color)
            } else {
                Style::default()
            })
            .style(Style::default().bg(app.main_background_color));

        f.render_widget(Clear, content_chunk[1]);

        let user_artist_track_table = searched_track_table_for_artist_ui(
            app.user_artist_track_names.clone(),
            app.user_artist_track_album.clone(),
            app.user_artist_track_duration.clone(),
            user_artist_track_block,
            app.main_highlight_color.clone(),
            app.main_background_color.clone(),
        );

        f.render_widget(Clear, content_chunk[1]);

        f.render_stateful_widget(
            user_artist_track_table,
            content_chunk[1],
            &mut app.user_artist_track_state,
        );
    }
}

/// Renders a simplified library view with only the list of library sections
pub fn render_default_library(f: &mut Frame, content_sub_chunk: &[Rect], app: &mut App) {
    // Define the library items
    let library_items = vec![
        String::from("Made For You"),
        String::from("Recently Played"),
        String::from("Liked Songs"),
        String::from("Albums"),
        String::from("Artists"),
        String::from("Podcasts"),
    ];

    let library_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Library"))
        .style(Style::default().bg(app.library_background_color));

    let library_list = List::new(library_items.clone()).block(library_block);
    f.render_widget(library_list, content_sub_chunk[0]);
}
