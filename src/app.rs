use crate::enums::{InputMode, Library, Menu};
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
    pub album_names: Vec<String>,
    pub track_names: Vec<String>,
    pub playlist_names: Vec<String>,
    pub artist_names: Vec<String>,
    pub album_links: Vec<String>,
    pub track_links: Vec<String>,
    pub playlist_links: Vec<String>,
    pub artist_links: Vec<String>,
    pub selected_album: bool,
    pub selected_track: bool,
    pub selected_playlist: bool,
    pub selected_artist: bool,
    pub selected_search: bool,
    pub search_state: ListState,
    pub album_state: ListState,
    pub track_state: ListState,
    pub playlist_state: ListState,
    pub artist_state: ListState,

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

    // Handles User's Saved Podcasts
    pub podcast_names: Vec<String>,
    pub podcast_links: Vec<String>,
    pub podcast_publisher: Vec<String>,
    pub podcast_selected: bool,
    pub podcast_display: bool,
    pub podcast_state: TableState,
    pub podcast_selected_uri: String,
    pub current_podcast: String,

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

    // Handles User's Saved Artists
    pub user_artist_names: Vec<String>,
    pub user_artist_links: Vec<String>,
    pub user_artist_selected: bool,
    pub user_artist_display: bool,
    pub user_artist_state: TableState,
    pub user_artist_selected_uri: String,
    pub current_user_artist: String,

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
    pub current_playing_album: String,
    pub is_playing: bool,
    pub progress_bar_ratio: f64,
    pub currently_playing_media_type: String,

    // Handle Help section
    pub is_help_section: bool,
    pub task: Vec<String>,
    pub key: Vec<String>,

    // Keybindings
    pub go_to_search_key: char,
    pub go_to_library_key: char,
    pub go_to_user_playlists_key: char,
    pub exit_application_key: char,
    pub shuffle_key: char,
    pub repeat_key: char,
    pub pause_play_key: char,
    pub help_key: char,
    pub volume_up_key: char,
    pub volume_down_key: char,

    pub first_keys: Vec<String>,
    pub tasks: Vec<String>,

    // Color's for UI
    pub border_color: Color,
    pub highlight_color: Color,
    pub background_color: Color,
}

impl App {
    /// Runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui, rx: Receiver<()>) -> io::Result<()> {
        let mut last_tick = Instant::now();
        // Set the duration for refreshing UI
        let timeout = Duration::from_millis(200);

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
            let now = std::time::Instant::now();
            if now.duration_since(last_tick) >= timeout {
                last_tick = now;

                // Check if a message has been received from the player info update thread
                if let Ok(_) = rx.try_recv() {
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
            // Configure the UI color's
            highlight_color: Color::Rgb(0, 255, 146),
            border_color: Color::Rgb(29, 185, 84),
            background_color: Color::Rgb(33, 33, 33),

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

            album_names: Vec::new(),
            album_links: Vec::new(),
            track_names: Vec::new(),
            track_links: Vec::new(),
            playlist_names: Vec::new(),
            playlist_links: Vec::new(),
            artist_names: Vec::new(),
            artist_links: Vec::new(),
            album_state: ListState::default(),
            track_state: ListState::default(),
            playlist_state: ListState::default(),
            artist_state: ListState::default(),
            search_state: ListState::default(),
            selected_album: false,
            selected_track: false,
            selected_playlist: false,
            selected_artist: false,
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
            shuffle_key: char::from(' '),
            repeat_key: char::from(' '),
            pause_play_key: char::from(' '),
            help_key: char::from(' '),
            volume_up_key: char::from(' '),
            volume_down_key: char::from(' '),

            first_keys: Vec::new(),
            tasks: Vec::new(),
        }
    }
}
