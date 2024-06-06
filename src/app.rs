use crate::enums::{InputMode, Library, Menu, SearchMenu};
use crate::handlers::key_event::handle_key_event;
use crate::handlers::key_event::search_input;
use crate::spotify::player::player::process_currently_playing;
use crate::ui::tui;
use crate::ui::ui::render_frame;
use crossterm::event::{self, Event};
use ratatui::style::Color;
use ratatui::widgets::{ListState, TableState};
use std::io;
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct App {
    pub file_name: String,
    // Controls whether the application should exit
    pub exit: bool,

    // Controls the navigation inside Menu
    pub selected_menu: Menu,
    pub can_navigate_menu: bool,

    // Controls the navigation inside Library
    pub selected_library: Library,
    pub library_index: usize,
    pub library_state: ListState,

    // Handles Search function
    pub search_query: String,
    pub input: String,
    pub cursor_position: usize,
    pub input_mode: InputMode,
    pub search_results_rendered: bool,
    pub search_menu: SearchMenu,

    pub album_names_search_results: Vec<String>,
    pub track_names_search_results: Vec<String>,
    pub playlist_names_search_results: Vec<String>,
    pub artist_names_search_results: Vec<String>,

    pub album_links_search_results: Vec<String>,
    pub track_links_search_results: Vec<String>,
    pub playlist_links_search_results: Vec<String>,
    pub artist_links_search_results: Vec<String>,

    pub album_index: usize,
    pub track_index: usize,
    pub playlist_index: usize,
    pub artist_index: usize,

    pub selected_album_in_search_result: bool,
    pub selected_track_in_search_result: bool,
    pub selected_playlist_in_search_result: bool,
    pub selected_artist_in_search_result: bool,
    pub selected_search: bool,

    pub search_state: ListState,
    pub album_state_in_search_result: ListState,
    pub track_state_in_search_result: ListState,
    pub playlist_state_in_search_result: ListState,
    pub artist_state_in_search_result: ListState,

    pub selected_album_tracks_names: Vec<String>,
    pub selected_album_tracks_artists: Vec<String>,
    pub selected_album_tracks_duration: Vec<i64>,
    pub selected_album_tracks_links: Vec<String>,
    pub searched_album_selected: bool,
    pub searched_album_state: TableState,
    pub searched_album_index: usize,

    pub selected_playlist_tracks_names: Vec<String>,
    pub selected_playlist_tracks_artists: Vec<String>,
    pub selected_playlist_tracks_duration: Vec<i64>,
    pub selected_playlist_tracks_albums: Vec<String>,
    pub selected_playlist_tracks_links: Vec<String>,
    pub searched_playlist_selected: bool,
    pub searched_playlist_state: TableState,
    pub searched_playlist_index: usize,

    pub selected_artist_tracks_names: Vec<String>,
    pub selected_artist_tracks_duration: Vec<i64>,
    pub selected_artist_tracks_links: Vec<String>,
    pub selected_artist_track_album_names: Vec<String>,
    pub searched_artist_selected: bool,
    pub searched_artist_state: TableState,
    pub searched_artist_index: usize,

    // Handles User's playlists
    pub user_playlist_names: Vec<String>,
    pub user_playlist_artist_names: Vec<String>,
    pub user_playlist_track_names: Vec<String>,
    pub user_playlist_album_names: Vec<String>,

    pub user_playlist_links: Vec<String>,
    pub user_playlist_track_links: Vec<String>,
    pub user_playlist_artist_links: Vec<String>,

    pub user_playlist_track_duration: Vec<i64>,

    pub current_user_playlist: String,
    pub selected_playlist_uri: String,

    pub user_playlist_display: bool,
    pub user_playlist_tracks_selected: bool,

    pub user_playlist_state: ListState,
    pub user_playlist_tracks_state: TableState,
    pub user_playlist_index: usize,

    pub enter_for_playback_in_user_playlist: bool,

    // Handles User's Liked Songs
    pub liked_song_names: Vec<String>,
    pub liked_song_links: Vec<String>,
    pub liked_song_duration: Vec<i64>,
    pub liked_song_artist_names: Vec<String>,
    pub liked_song_artist_links: Vec<String>,
    pub liked_song_album_names: Vec<String>,
    pub liked_songs_selected: bool,
    pub liked_song_display: bool,
    pub selected_liked_song_uri: String,
    pub liked_songs_state: TableState,
    pub liked_songs_index: usize,
    pub enter_for_playback_in_liked_song: bool,

    // Handles User's Saved Albums
    pub user_album_names: Vec<String>,
    pub user_album_links: Vec<String>,
    pub user_album_artist_names: Vec<String>,
    pub user_album_artist_links: Vec<String>,
    pub user_album_tracks: Vec<usize>,
    pub user_album_selected: bool,
    pub user_album_display: bool,
    pub user_album_state: TableState,
    pub user_album_selected_uri: String,
    pub current_user_album: String,
    pub user_album_index: usize,

    pub user_album_track_names: Vec<String>,
    pub user_album_track_artist: Vec<String>,
    pub user_album_track_duration: Vec<i64>,
    pub user_album_track_index: usize,
    pub user_album_track_state: TableState,
    pub user_album_track_display: bool,
    pub user_album_track_selected: bool, // for a track list that is selected
    pub user_album_current_album_selected: bool, // for a album that is selected
    pub user_album_track_links: Vec<String>,
    pub enter_for_playback_in_user_album: bool,

    // Handles User's Saved Podcasts
    pub podcast_names: Vec<String>,
    pub podcast_links: Vec<String>,
    pub podcast_publisher: Vec<String>,
    pub podcast_selected: bool,
    pub podcast_display: bool,
    pub podcast_state: TableState,
    pub podcast_selected_uri: String,
    pub current_podcast: String,
    pub podcast_index: usize,

    // Handles User's Recently Played Songs
    pub recently_played_names: Vec<String>,
    pub recently_played_links: Vec<String>,
    pub recently_played_duration: Vec<i64>,
    pub recently_played_artist_names: Vec<String>,
    pub recently_played_artist_links: Vec<String>,
    pub recently_played_album_names: Vec<String>,
    pub recently_played_selected: bool,
    pub recently_played_display: bool,
    pub selected_recently_played_uri: String,
    pub recently_played_state: TableState,
    pub recently_played_index: usize,
    pub enter_for_playback_in_recently_played: bool,

    // Handles User's Saved Artists
    pub user_artist_names: Vec<String>,
    pub user_artist_links: Vec<String>,
    pub user_artist_selected: bool,
    pub user_artist_display: bool,
    pub user_artist_state: TableState,
    pub user_artist_selected_uri: String,
    pub current_user_artist: String,
    pub user_artist_index: usize,
    pub enter_for_playback_in_saved_artist: bool,

    pub user_artist_track_names: Vec<String>,
    pub user_artist_track_album: Vec<String>,
    pub user_artist_track_duration: Vec<i64>,
    pub user_artist_track_index: usize,
    pub user_artist_track_state: TableState,
    pub user_artist_track_display: bool,
    pub user_artist_track_selected: bool, // for a track list that is selected
    pub user_artist_current_artist_selected: bool, // for an artist that is selected
    pub user_artist_track_links: Vec<String>,

    // Handles Made For You
    pub made_fy_playlist_names: Vec<String>,
    pub made_fy_playlist_links: Vec<String>,
    pub made_fy_selected: bool,
    pub made_fy_display: bool,
    pub made_fy_state: TableState,
    pub made_fy_index: usize,
    pub made_fy_playlist_track_total: Vec<i64>,
    pub made_fy_track_links: Vec<String>,
    pub made_fy_track_names: Vec<String>,
    pub made_fy_track_duration: Vec<i64>,
    pub made_fy_artist_names: Vec<String>,
    pub made_fy_album_names: Vec<String>,
    pub made_fy_track_display: bool,
    pub made_fy_track_state: TableState,
    pub made_fy_track_index: usize,
    pub made_fy_track_selected: bool, // for a track list that is selected
    pub made_fy_current_playlist_selected: bool, // for a playlist that is selected
    pub enter_for_playback_in_made_fy: bool,

    // Handles User's currently playing device
    pub current_device_name: String,
    pub current_device_volume: String,
    pub is_device_active: Vec<bool>,
    pub playback_status: String,
    pub device_ids: Vec<String>,
    pub current_device_id: Option<String>,
    pub shuffle_status: String,
    pub repeat_status: String,
    pub is_shuffle: bool,
    pub currrent_timestamp: f64,
    pub ending_timestamp: f64,
    pub currently_playing_artist: String,
    pub current_playing_name: String,
    pub current_playing_id: String,
    pub current_playing_album: String,
    pub is_playing: bool,
    pub progress_bar_ratio: f64,
    pub currently_playing_media_type: String,

    // Handle Help section
    pub is_help_section: bool,
    pub task: Vec<String>,
    pub key: Vec<String>,

    // Handle New Release section
    pub new_release_artist: Vec<String>,
    pub new_release_name: Vec<String>,
    pub new_release_state: ListState,
    pub current_new_release: String,
    pub new_release_display: bool,
    pub new_release_selected: String,
    pub is_new_release_selected: bool,
    pub new_release_album_selected: bool,
    pub new_release_album_state: TableState,
    pub new_release_album_links: Vec<String>,
    pub current_new_release_album: String,
    pub current_new_release_album_link: String,
    pub new_release_index: usize,

    pub new_release_track_names: Vec<String>,
    pub new_release_artist_names: Vec<String>,
    pub new_release_durations_ms: Vec<i64>,
    pub new_release_spotify_urls: Vec<String>,
    pub enter_for_playback_in_new_release: bool,

    // Keybindings
    pub go_to_search_key: char,
    pub go_to_library_key: char,
    pub go_to_user_playlists_key: char,
    pub exit_application_key: char,
    pub pause_play_key: char,
    pub help_key: char,
    pub volume_up_key: char,
    pub volume_down_key: char,
    pub new_release_key: char,
    pub lyrics_key: char,
    pub next_track_key: char,
    pub previous_track_key: char,
    pub error_key: char,
    pub player_fullscreen_key: char,

    pub first_keys: Vec<String>,
    pub tasks: Vec<String>,

    // Color's for UI
    pub player_border_color: Color,
    pub player_highlight_color: Color,
    pub player_background_color: Color,

    pub library_border_color: Color,
    pub library_highlight_color: Color,
    pub library_background_color: Color,

    pub playlist_border_color: Color,
    pub playlist_highlight_color: Color,
    pub playlist_background_color: Color,

    pub new_release_border_color: Color,
    pub new_release_highlight_color: Color,
    pub new_release_background_color: Color,

    pub main_border_color: Color,
    pub main_highlight_color: Color,
    pub main_background_color: Color,

    pub search_border_color: Color,
    pub search_highlight_color: Color,
    pub search_background_color: Color,

    pub help_border_color: Color,
    pub help_highlight_color: Color,
    pub help_background_color: Color,

    pub error_border_color: Color,
    pub error_background_color: Color,

    // Volume controls
    pub volume_increment_value: u8,
    pub volume_decreament_value: u8,
    pub volume_percent: u8,

    // Creds
    pub client_id: String,
    pub client_secret: String,

    // Error
    pub error_text: String,

    // Playback
    pub selected_link_for_playback: String,
    pub is_only_id: bool,
    pub is_in_track: bool,

    // Top Tracks
    pub top_tracks_all_time_names: Vec<String>,
    pub top_tracks_6_months_names: Vec<String>,
    pub top_tracks_4_weeks_names: Vec<String>,

    // Add track to playlist
    pub add_track_to_playlist_state: ListState,
    pub track_added_to_playlist_name: String,
    pub playlist_index_for_track_addition: usize,
    pub track_added_to_playlist_link: String,
    pub playlist_link_for_track_addition: String,
}

impl App {
    /// Runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui, rx1: Receiver<()>) -> io::Result<()> {
        let mut last_tick: Instant = Instant::now();
        // Set the duration for refreshing UI
        let timeout: Duration = Duration::from_millis(200);

        while !self.exit {
            // Handling user inputs
            if event::poll(timeout)? {
                if let Event::Key(key_event) = event::read()? {
                    handle_key_event(self, key_event);

                    // In editing mode, handle search input
                    if self.input_mode == InputMode::Editing {
                        let _ = search_input(self, key_event);
                    }
                }
            }

            // Update UI
            let now: Instant = std::time::Instant::now();
            if now.duration_since(last_tick) >= timeout {
                last_tick = now;

                // Check if a message has been received from the player info update thread
                if let Ok(_) = rx1.try_recv() {
                    process_currently_playing(self);
                }

                // Draw the UI
                terminal.draw(|frame| render_frame(frame, self.selected_menu, self))?;
            }
        }

        Ok(())
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            exit: false,

            selected_menu: Menu::Default,

            selected_library: Library::MadeFY,
            library_index: 0,
            library_state: ListState::default(),

            user_playlist_state: ListState::default(),

            search_query: "".to_string(),
            input: String::new(),
            input_mode: InputMode::Normal,
            cursor_position: 0,
            search_menu: SearchMenu::Default,

            album_names_search_results: Vec::new(),
            album_links_search_results: Vec::new(),
            track_names_search_results: Vec::new(),
            track_links_search_results: Vec::new(),
            playlist_names_search_results: Vec::new(),
            playlist_links_search_results: Vec::new(),
            artist_names_search_results: Vec::new(),
            artist_links_search_results: Vec::new(),
            album_state_in_search_result: ListState::default(),
            track_state_in_search_result: ListState::default(),
            playlist_state_in_search_result: ListState::default(),
            artist_state_in_search_result: ListState::default(),
            search_state: ListState::default(),
            selected_album_in_search_result: false,
            selected_track_in_search_result: false,
            selected_playlist_in_search_result: false,
            selected_artist_in_search_result: false,
            selected_search: false,
            search_results_rendered: false,

            user_playlist_names: Vec::new(),
            user_playlist_links: Vec::new(),
            user_playlist_track_names: Vec::new(),
            user_playlist_track_duration: Vec::new(),
            user_playlist_artist_names: Vec::new(),
            user_playlist_track_links: Vec::new(),
            user_playlist_artist_links: Vec::new(),
            user_playlist_album_names: Vec::new(),
            selected_playlist_uri: String::new(),
            current_user_playlist: String::new(),
            user_playlist_display: false,
            user_playlist_tracks_selected: false,
            user_playlist_tracks_state: TableState::default(),

            liked_songs_state: TableState::default(),
            liked_song_names: Vec::new(),
            liked_song_links: Vec::new(),
            liked_song_duration: Vec::new(),
            liked_song_artist_names: Vec::new(),
            liked_song_artist_links: Vec::new(),
            liked_songs_selected: false,
            selected_liked_song_uri: String::new(),
            liked_song_display: false,
            liked_song_album_names: Vec::new(),

            user_album_display: false,
            user_album_selected: false,
            user_album_state: TableState::default(),
            user_album_names: Vec::new(),
            user_album_links: Vec::new(),
            user_album_selected_uri: String::new(),
            current_user_album: String::new(),
            user_album_artist_names: Vec::new(),
            user_album_artist_links: Vec::new(),
            user_album_tracks: Vec::new(),
            can_navigate_menu: true,

            recently_played_names: Vec::new(),
            recently_played_links: Vec::new(),
            recently_played_duration: Vec::new(),
            recently_played_artist_names: Vec::new(),
            recently_played_artist_links: Vec::new(),
            recently_played_album_names: Vec::new(),
            recently_played_selected: false,
            recently_played_display: false,
            selected_recently_played_uri: String::new(),
            recently_played_state: TableState::default(),

            podcast_names: Vec::new(),
            podcast_links: Vec::new(),
            podcast_publisher: Vec::new(),
            podcast_selected: false,
            podcast_display: false,
            podcast_state: TableState::default(),
            podcast_selected_uri: String::new(),
            current_podcast: String::new(),

            user_artist_names: Vec::new(),
            user_artist_links: Vec::new(),
            user_artist_selected: false,
            user_artist_display: false,
            user_artist_state: TableState::default(),
            user_artist_selected_uri: String::new(),
            current_user_artist: String::new(),

            current_device_name: String::new(),
            current_device_volume: String::new(),
            is_device_active: Vec::new(),
            playback_status: String::from("Playing"),
            shuffle_status: String::from("Off"),
            repeat_status: String::from("Off"),
            is_shuffle: false,
            device_ids: Vec::new(),
            current_device_id: Some(String::new()),

            currrent_timestamp: f64::from(0),
            ending_timestamp: f64::from(1),
            currently_playing_artist: String::new(),
            current_playing_name: String::new(),
            current_playing_id: String::new(),
            current_playing_album: String::new(),
            is_playing: false,
            progress_bar_ratio: 0.0,
            currently_playing_media_type: String::new(),

            is_help_section: false,
            task: Vec::new(),
            key: Vec::new(),

            go_to_search_key: char::from(' '),
            go_to_library_key: char::from(' '),
            go_to_user_playlists_key: char::from(' '),
            exit_application_key: char::from(' '),
            pause_play_key: char::from(' '),
            help_key: char::from(' '),
            volume_up_key: char::from(' '),
            volume_down_key: char::from(' '),
            new_release_key: char::from(' '),
            lyrics_key: char::from(' '),
            next_track_key: char::from(' '),
            previous_track_key: char::from(' '),
            error_key: char::from(' '),
            player_fullscreen_key: char::from(' '),
            add_track_to_playlist_state: ListState::default(),

            first_keys: Vec::new(),
            tasks: Vec::new(),

            new_release_artist: Vec::new(),
            new_release_name: Vec::new(),
            new_release_state: ListState::default(),
            current_new_release: String::new(),
            new_release_display: false,
            new_release_selected: String::new(),
            is_new_release_selected: false,
            new_release_album_selected: false,
            new_release_album_state: TableState::default(),
            new_release_album_links: Vec::new(),
            current_new_release_album: String::new(),
            current_new_release_album_link: String::new(),
            new_release_track_names: Vec::new(),
            new_release_artist_names: Vec::new(),
            new_release_durations_ms: Vec::new(),
            new_release_spotify_urls: Vec::new(),

            volume_increment_value: 0,
            volume_decreament_value: 0,
            volume_percent: 0,

            client_id: String::new(),
            client_secret: String::new(),

            player_border_color: Color::Rgb(0, 0, 0),
            player_highlight_color: Color::Rgb(0, 0, 0),
            player_background_color: Color::Rgb(0, 0, 0),

            library_border_color: Color::Rgb(0, 0, 0),
            library_highlight_color: Color::Rgb(0, 0, 0),
            library_background_color: Color::Rgb(0, 0, 0),

            playlist_border_color: Color::Rgb(0, 0, 0),
            playlist_highlight_color: Color::Rgb(0, 0, 0),
            playlist_background_color: Color::Rgb(0, 0, 0),

            new_release_border_color: Color::Rgb(0, 0, 0),
            new_release_highlight_color: Color::Rgb(0, 0, 0),
            new_release_background_color: Color::Rgb(0, 0, 0),

            main_border_color: Color::Rgb(0, 0, 0),
            main_highlight_color: Color::Rgb(0, 0, 0),
            main_background_color: Color::Rgb(0, 0, 0),

            search_border_color: Color::Rgb(0, 0, 0),
            search_highlight_color: Color::Rgb(0, 0, 0),
            search_background_color: Color::Rgb(0, 0, 0),

            help_border_color: Color::Rgb(0, 0, 0),
            help_highlight_color: Color::Rgb(0, 0, 0),
            help_background_color: Color::Rgb(0, 0, 0),

            error_border_color: Color::Rgb(0, 0, 0),
            error_background_color: Color::Rgb(0, 0, 0),

            album_index: 0,
            track_index: 0,
            playlist_index: 0,
            artist_index: 0,
            user_playlist_index: 0,
            liked_songs_index: 0,
            user_album_index: 0,
            podcast_index: 0,
            recently_played_index: 0,
            user_artist_index: 0,
            new_release_index: 0,

            error_text: String::new(),

            selected_album_tracks_names: Vec::new(),
            selected_album_tracks_artists: Vec::new(),
            selected_album_tracks_duration: Vec::new(),
            selected_album_tracks_links: Vec::new(),
            searched_album_selected: false,
            searched_album_state: TableState::default(),
            searched_album_index: 0,

            selected_artist_track_album_names: Vec::new(),
            searched_artist_selected: false,
            searched_artist_state: TableState::default(),
            searched_artist_index: 0,
            selected_artist_tracks_names: Vec::new(),
            selected_artist_tracks_duration: Vec::new(),
            selected_artist_tracks_links: Vec::new(),

            searched_playlist_selected: false,
            searched_playlist_state: TableState::default(),
            searched_playlist_index: 0,
            selected_playlist_tracks_names: Vec::new(),
            selected_playlist_tracks_artists: Vec::new(),
            selected_playlist_tracks_duration: Vec::new(),
            selected_playlist_tracks_albums: Vec::new(),
            selected_playlist_tracks_links: Vec::new(),

            made_fy_playlist_names: Vec::new(),
            made_fy_playlist_links: Vec::new(),
            made_fy_selected: false,
            made_fy_display: false,
            made_fy_state: TableState::default(),
            made_fy_index: 0,
            made_fy_playlist_track_total: Vec::new(),
            made_fy_track_links: Vec::new(),
            made_fy_track_names: Vec::new(),
            made_fy_track_duration: Vec::new(),
            made_fy_artist_names: Vec::new(),
            made_fy_album_names: Vec::new(),
            made_fy_current_playlist_selected: false,
            made_fy_track_display: false,
            made_fy_track_state: TableState::default(),
            made_fy_track_selected: false,
            made_fy_track_index: 0,

            user_album_track_names: Vec::new(),
            user_album_track_artist: Vec::new(),
            user_album_track_duration: Vec::new(),
            user_album_track_index: 0,
            user_album_track_state: TableState::default(),
            user_album_track_display: false,
            user_album_track_selected: false,
            user_album_track_links: Vec::new(),
            user_album_current_album_selected: false,

            user_artist_track_names: Vec::new(),
            user_artist_track_album: Vec::new(),
            user_artist_track_duration: Vec::new(),
            user_artist_track_index: 0,
            user_artist_track_state: TableState::default(),
            user_artist_track_display: false,
            user_artist_track_selected: false,
            user_artist_current_artist_selected: false,
            user_artist_track_links: Vec::new(),

            selected_link_for_playback: String::new(),

            enter_for_playback_in_user_playlist: false,
            enter_for_playback_in_liked_song: false,
            enter_for_playback_in_user_album: false,
            enter_for_playback_in_recently_played: false,
            enter_for_playback_in_saved_artist: false,
            enter_for_playback_in_made_fy: false,
            enter_for_playback_in_new_release: false,

            is_only_id: false,
            is_in_track: false,

            top_tracks_all_time_names: Vec::new(),
            top_tracks_6_months_names: Vec::new(),
            top_tracks_4_weeks_names: Vec::new(),
            file_name: String::new(),

            track_added_to_playlist_name: String::new(),
            playlist_index_for_track_addition: 0,
            track_added_to_playlist_link: String::new(),
            playlist_link_for_track_addition: String::new(),
        }
    }
}
