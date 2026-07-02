use crate::structs::UserSavedArtist;
use crate::structs::UserRecentlyPlayed;
use crate::structs::UserSavedPodcast;
use crate::structs::MadeFY;
use crate::structs::LikedSongs;
use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{block::Title, Block, Borders, Clear, List},
    Frame,
};

use crate::ui::util::{album_table_ui, artist_table_ui, podcast_table_ui, track_table_ui};
use crate::{app::App, structs::Themes, structs::UserSavedAlbums};

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
    theme: &mut Themes,
    likedsongs: &mut LikedSongs,
    useralbum: &mut UserSavedAlbums, 
    madefy: &mut MadeFY,
    podcast: &mut UserSavedPodcast, 
    recentlyplayed: &mut UserRecentlyPlayed,
    userartist: &mut UserSavedArtist
) {
    let library_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Library"))
        .border_style(Style::new().fg(theme.library_border_color))
        .style(Style::default().bg(theme.library_background_color));

    let liked_song_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Liked Songs"))
        .border_style(if likedsongs.selected {
            Style::default().fg(theme.main_border_color)
        } else {
            Style::default().fg(theme.main_inactive_border_color)
        })
        .style(Style::default().bg(theme.main_background_color));

    let recently_played_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Recently Played"))
        .border_style(if recentlyplayed.selected {
            Style::default().fg(theme.main_border_color)
        } else {
            Style::default().fg(theme.main_inactive_border_color)
        })
        .style(Style::default().bg(theme.main_background_color));

    let user_album_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Albums"))
        .border_style(if useralbum.selected {
            Style::default().fg(theme.main_border_color)
        } else {
            Style::default().fg(theme.main_inactive_border_color)
        })
        .style(Style::default().bg(theme.main_background_color));

    let podcast_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Podcasts"))
        .border_style(if podcast.selected {
            Style::default().fg(theme.main_border_color)
        } else {
            Style::default().fg(theme.main_inactive_border_color)
        })
        .style(Style::default().bg(theme.main_background_color));

    let user_artist_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Artists"))
        .border_style(if userartist.user_artist_selected {
            Style::default().fg(theme.main_border_color)
        } else {
            Style::default().fg(theme.main_inactive_border_color)
        })
        .style(Style::default().bg(theme.main_background_color));

    let made_fy_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Made For You"))
        .border_style(if madefy.selected {
            Style::default().fg(theme.main_border_color)
        } else {
            Style::default().fg(theme.main_inactive_border_color)
        })
        .style(Style::default().bg(theme.main_background_color));

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
        .highlight_style(Style::default().fg(theme.library_highlight_color));

    f.render_stateful_widget(library_list, content_sub_chunk[0], &mut app.library_state);

    // Render content for the selected library section based on app state.
    if madefy.display {
        f.render_widget(Clear, content_chunk[1]);

        let made_fy_playlist_table = made_fy_table_ui(
            madefy.playlist_names.clone(),
            madefy.playlist_track_total.clone(),
            made_fy_block,
            theme.main_highlight_color,
            theme.main_background_color,
            theme.main_inactive_border_color,
        );

        f.render_widget(Clear, content_chunk[1]);

        f.render_stateful_widget(
            made_fy_playlist_table,
            content_chunk[1],
            &mut madefy.state,
        );
    }

    if madefy.track_display {
        let made_fy_track_label = madefy.playlist_names[madefy.index].to_string();

        let made_fy_track_block = Block::default()
            .borders(Borders::ALL)
            .title(Title::from(made_fy_track_label))
            .border_style(if madefy.track_selected {
                Style::default().fg(theme.main_border_color)
            } else {
                Style::default().fg(theme.main_inactive_border_color)
            })
            .style(Style::default().bg(theme.main_background_color));

        f.render_widget(Clear, content_chunk[1]);

        let made_fy_track_table = track_table_ui(
            madefy.track_names.clone(),
            madefy.artist_names.clone(),
            madefy.album_names.clone(),
            madefy.track_duration.clone(),
            made_fy_track_block,
            theme.main_highlight_color,
            theme.main_background_color,
            theme.main_inactive_border_color,
        );

        f.render_widget(Clear, content_chunk[1]);

        f.render_stateful_widget(
            made_fy_track_table,
            content_chunk[1],
            &mut madefy.track_state,
        );
    }

    if likedsongs.display {
        f.render_widget(Clear, content_chunk[1]);

        let liked_songs_table = track_table_ui(
            likedsongs.names.clone(),
            likedsongs.artist_names.clone(),
            likedsongs.album_names.clone(),
            likedsongs.duration.clone(),
            liked_song_block,
            theme.main_highlight_color,
            theme.main_background_color,
            theme.main_inactive_border_color,
        );

        f.render_widget(Clear, content_chunk[1]);

        f.render_stateful_widget(
            liked_songs_table,
            content_chunk[1],
            &mut likedsongs.state,
        );
    }

    if recentlyplayed.display {
        f.render_widget(Clear, content_chunk[1]);

        let recently_played_table = track_table_ui(
            recentlyplayed.names.clone(),
            recentlyplayed.artist_names.clone(),
            recentlyplayed.album_names.clone(),
            recentlyplayed.duration.clone(),
            recently_played_block,
            theme.main_highlight_color,
            theme.main_background_color,
            theme.main_inactive_border_color,
        );

        f.render_widget(Clear, content_chunk[1]);

        f.render_stateful_widget(
            recently_played_table,
            content_chunk[1],
            &mut recentlyplayed.state,
        );
    }

    if podcast.display {
        f.render_widget(Clear, content_chunk[1]);

        let podcast_table = podcast_table_ui(
            podcast.names.clone(),
            podcast.publisher.clone(),
            podcast_block,
            theme.main_highlight_color,
            theme.main_background_color,
            theme.main_inactive_border_color,
        );

        f.render_widget(Clear, content_chunk[1]);

        f.render_stateful_widget(podcast_table, content_chunk[1], &mut podcast.state);
    }

    if userartist.user_artist_display {
        f.render_widget(Clear, content_chunk[1]);

        let artist_table = artist_table_ui(
            userartist.user_artist_names.clone(),
            user_artist_block,
            theme.main_highlight_color,
            theme.main_background_color,
            theme.main_inactive_border_color,
        );

        f.render_widget(Clear, content_chunk[1]);

        f.render_stateful_widget(artist_table, content_chunk[1], &mut userartist.user_artist_state);
    }

    if useralbum.display {
        f.render_widget(Clear, content_chunk[1]);

        let user_album_table = album_table_ui(
            useralbum.names.clone(),
            useralbum.artist_names.clone(),
            useralbum.tracks.clone(),
            user_album_block,
            theme.main_highlight_color,
            theme.main_background_color,
            theme.main_inactive_border_color,
        );

        f.render_widget(Clear, content_chunk[1]);

        f.render_stateful_widget(
            user_album_table,
            content_chunk[1],
            &mut useralbum.state,
        );
    }
    if useralbum.track_display {
        let user_album_track_label = useralbum.names[useralbum.index].to_string();

        let user_album_track_block = Block::default()
            .borders(Borders::ALL)
            .title(Title::from(user_album_track_label))
            .border_style(if useralbum.track_selected {
                Style::default().fg(theme.main_border_color)
            } else {
                Style::default().fg(theme.main_inactive_border_color)
            })
            .style(Style::default().bg(theme.main_background_color));

        f.render_widget(Clear, content_chunk[1]);

        let user_album_track_table = searched_track_table_for_album_ui(
            useralbum.track_names.clone(),
            useralbum.track_artist.clone(),
            useralbum.track_duration.clone(),
            user_album_track_block,
            theme.main_highlight_color,
            theme.main_background_color,
            theme.main_inactive_border_color,
        );

        f.render_widget(Clear, content_chunk[1]);

        f.render_stateful_widget(
            user_album_track_table,
            content_chunk[1],
            &mut useralbum.track_state,
        );
    }
    if userartist.user_artist_track_display {
        let user_artist_track_label = userartist.user_artist_names[userartist.user_artist_index].to_string();

        let user_artist_track_block = Block::default()
            .borders(Borders::ALL)
            .title(Title::from(user_artist_track_label))
            .border_style(if userartist.user_artist_track_selected {
                Style::default().fg(theme.main_border_color)
            } else {
                Style::default().fg(theme.main_inactive_border_color)
            })
            .style(Style::default().bg(theme.main_background_color));

        f.render_widget(Clear, content_chunk[1]);

        let user_artist_track_table = searched_track_table_for_artist_ui(
            userartist.user_artist_track_names.clone(),
            userartist.user_artist_track_album.clone(),
            userartist.user_artist_track_duration.clone(),
            user_artist_track_block,
            theme.main_highlight_color,
            theme.main_background_color,
            theme.main_inactive_border_color,
        );

        f.render_widget(Clear, content_chunk[1]);

        f.render_stateful_widget(
            user_artist_track_table,
            content_chunk[1],
            &mut userartist.user_artist_track_state,
        );
    }
}

/// Renders a simplified library view with only the list of library sections
pub fn render_default_library(f: &mut Frame, content_sub_chunk: &[Rect], theme: &mut Themes) {
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
        .style(
            Style::default()
                .bg(theme.library_background_color)
                .fg(theme.main_inactive_border_color),
        );

    let library_list = List::new(library_items.clone()).block(library_block);
    f.render_widget(library_list, content_sub_chunk[0]);
}
