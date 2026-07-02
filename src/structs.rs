use crate::enums::InputMode;
use crate::enums::SearchMenu;
use ratatui::style::Color;
use ratatui::widgets::{ListState, TableState};

#[derive(Clone, Debug)]
pub struct Key {
    pub go_to_search_key: char,
    pub go_to_library_key: char,
    pub go_to_user_playlists_key: char,
    pub exit_application_key: char,
    pub pause_play_key: char,
    pub help_key: char,
    pub volume_up_key: char,
    pub volume_down_key: char,
    pub new_release_key: char,
    pub next_track_key: char,
    pub previous_track_key: char,
    pub error_key: char,
    pub player_fullscreen_key: char,
    pub change_keybind: char,
    pub refresh_key: char,
    pub open_config_fold_key: char,

    pub first_keys: Vec<String>,
    pub tasks: Vec<String>,
}

impl Default for Key {
    fn default() -> Self {
        Self {
            open_config_fold_key: ' ',
            go_to_search_key: ' ',
            go_to_library_key: ' ',
            go_to_user_playlists_key: ' ',
            exit_application_key: ' ',
            pause_play_key: ' ',
            help_key: ' ',
            volume_up_key: ' ',
            volume_down_key: ' ',
            new_release_key: ' ',
            next_track_key: ' ',
            previous_track_key: ' ',
            error_key: ' ',
            player_fullscreen_key: ' ',
            change_keybind: ' ',
            refresh_key: ' ',

            first_keys: Vec::new(),
            tasks: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Themes {
    pub player_border_color: Color,
    pub player_highlight_color: Color,
    pub player_background_color: Color,
    pub player_inactive_border_color: Color,

    pub library_border_color: Color,
    pub library_highlight_color: Color,
    pub library_background_color: Color,
    pub library_inactive_border_color: Color,

    pub playlist_border_color: Color,
    pub playlist_highlight_color: Color,
    pub playlist_background_color: Color,
    pub playlist_inactive_border_color: Color,

    pub new_release_border_color: Color,
    pub new_release_highlight_color: Color,
    pub new_release_background_color: Color,
    pub new_release_inactive_border_color: Color,

    pub main_border_color: Color,
    pub main_highlight_color: Color,
    pub main_background_color: Color,
    pub main_inactive_border_color: Color,

    pub search_border_color: Color,
    pub search_highlight_color: Color,
    pub search_background_color: Color,
    pub search_inactive_border_color: Color,

    pub help_border_color: Color,
    pub help_highlight_color: Color,
    pub help_background_color: Color,

    pub error_border_color: Color,
    pub error_background_color: Color,
}

impl Default for Themes {
    fn default() -> Self {
        Self {
            player_border_color: Color::Rgb(0, 0, 0),
            player_highlight_color: Color::Rgb(0, 0, 0),
            player_background_color: Color::Rgb(0, 0, 0),
            player_inactive_border_color: Color::Rgb(0, 0, 0),

            library_border_color: Color::Rgb(0, 0, 0),
            library_highlight_color: Color::Rgb(0, 0, 0),
            library_background_color: Color::Rgb(0, 0, 0),
            library_inactive_border_color: Color::Rgb(0, 0, 0),

            playlist_border_color: Color::Rgb(0, 0, 0),
            playlist_highlight_color: Color::Rgb(0, 0, 0),
            playlist_background_color: Color::Rgb(0, 0, 0),
            playlist_inactive_border_color: Color::Rgb(0, 0, 0),

            new_release_border_color: Color::Rgb(0, 0, 0),
            new_release_highlight_color: Color::Rgb(0, 0, 0),
            new_release_background_color: Color::Rgb(0, 0, 0),
            new_release_inactive_border_color: Color::Rgb(0, 0, 0),

            main_border_color: Color::Rgb(0, 0, 0),
            main_highlight_color: Color::Rgb(0, 0, 0),
            main_background_color: Color::Rgb(0, 0, 0),
            main_inactive_border_color: Color::Rgb(0, 0, 0),

            search_border_color: Color::Rgb(0, 0, 0),
            search_highlight_color: Color::Rgb(0, 0, 0),
            search_background_color: Color::Rgb(0, 0, 0),
            search_inactive_border_color: Color::Rgb(0, 0, 0),

            help_border_color: Color::Rgb(0, 0, 0),
            help_highlight_color: Color::Rgb(0, 0, 0),
            help_background_color: Color::Rgb(0, 0, 0),

            error_border_color: Color::Rgb(0, 0, 0),
            error_background_color: Color::Rgb(0, 0, 0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Settings {
    // Volume controls
    pub volume_increment_value: u8,
    pub volume_decreament_value: u8,
    pub volume_percent: u8,
    pub theme_name: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            volume_increment_value: 0,
            volume_decreament_value: 0,
            volume_percent: 0,
            theme_name: String::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Search {
    pub search_query: String,
    pub input: String,
    pub cursor_position: usize,
    pub input_mode: InputMode,
    pub results_rendered: bool,
    pub menu: SearchMenu,

    pub album_names: Vec<String>,
    pub track_names: Vec<String>,
    pub playlist_names: Vec<String>,
    pub artist_names: Vec<String>,

    pub album_links: Vec<String>,
    pub track_links: Vec<String>,
    pub playlist_links: Vec<String>,
    pub artist_links: Vec<String>,

    pub album_index: usize,
    pub track_index: usize,
    pub playlist_index: usize,
    pub artist_index: usize,

    pub selected_album: bool,
    pub selected_track: bool,
    pub selected_playlist: bool,
    pub selected_artist: bool,
    pub selected_search: bool,

    pub state: ListState,
    pub album_state: ListState,
    pub track_state: ListState,
    pub playlist_state: ListState,
    pub artist_state: ListState,

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
}

impl Default for Search {
    fn default() -> Self {
        Self {
            search_query: "".to_string(),
            input: String::new(),
            input_mode: InputMode::Normal,
            cursor_position: 0,
            menu: SearchMenu::Default,

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
            state: ListState::default(),
            selected_album: false,
            selected_track: false,
            selected_playlist: false,
            selected_artist: false,
            selected_search: false,
            results_rendered: false,

            album_index: 0,
            track_index: 0,
            playlist_index: 0,
            artist_index: 0,

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
        }
    }
}

#[derive(Clone, Debug)]
pub struct UserPlaylist {
    pub names: Vec<String>,
    pub artist_names: Vec<String>,
    pub track_names: Vec<String>,
    pub album_names: Vec<String>,
    pub links: Vec<String>,
    pub track_links: Vec<String>,
    pub track_duration: Vec<i64>,
    pub current: String,
    pub selected_playlist_uri: String,
    pub display: bool,
    pub tracks_selected: bool,
    pub state: ListState,
    pub tracks_state: TableState,
    pub index: usize,
    pub enter_for_playback_in_user_playlist: bool,
}

impl Default for UserPlaylist {
    fn default() -> Self {
        Self {
            state: ListState::default(),
            names: Vec::new(),
            links: Vec::new(),
            track_names: Vec::new(),
            track_duration: Vec::new(),
            artist_names: Vec::new(),
            track_links: Vec::new(),
            album_names: Vec::new(),
            selected_playlist_uri: String::new(),
            current: String::new(),
            display: false,
            tracks_selected: false,
            tracks_state: TableState::default(),
            index: 0,
            enter_for_playback_in_user_playlist: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct LikedSongs {
    pub names: Vec<String>,
    pub links: Vec<String>,
    pub duration: Vec<i64>,
    pub artist_names: Vec<String>,
    pub album_names: Vec<String>,
    pub selected: bool,
    pub display: bool,
    pub state: TableState,
    pub index: usize,
    pub enter_for_playback_in_liked_song: bool,
}

impl Default for LikedSongs {
    fn default() -> Self {
        Self {
            state: TableState::default(),
            names: Vec::new(),
            links: Vec::new(),
            duration: Vec::new(),
            artist_names: Vec::new(),
            selected: false,
            display: false,
            album_names: Vec::new(),
            index: 0,
            enter_for_playback_in_liked_song: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UserSavedAlbums {
    pub names: Vec<String>,
    pub links: Vec<String>,
    pub artist_names: Vec<String>,
    pub tracks: Vec<usize>,
    pub selected: bool,
    pub display: bool,
    pub state: TableState,
    pub index: usize,
    pub track_names: Vec<String>,
    pub track_artist: Vec<String>,
    pub track_duration: Vec<i64>,
    pub track_index: usize,
    pub track_state: TableState,
    pub track_display: bool,
    pub track_selected: bool, // for a track list that is selected
    pub current_album_selected: bool, // for a album that is selected
    pub track_links: Vec<String>,
    pub enter_for_playback_in_user_album: bool,
}

impl Default for UserSavedAlbums {
    fn default() -> Self {
        Self {
            display: false,
            selected: false,
            state: TableState::default(),
            names: Vec::new(),
            links: Vec::new(),
            artist_names: Vec::new(),
            tracks: Vec::new(),
            index: 0,
            track_names: Vec::new(),
            track_artist: Vec::new(),
            track_duration: Vec::new(),
            track_index: 0,
            track_state: TableState::default(),
            track_display: false,
            track_selected: false,
            track_links: Vec::new(),
            current_album_selected: false,
            enter_for_playback_in_user_album: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MadeFY {
    pub playlist_names: Vec<String>,
    pub playlist_links: Vec<String>,
    pub selected: bool,
    pub display: bool,
    pub state: TableState,
    pub index: usize,
    pub playlist_track_total: Vec<i64>,
    pub track_links: Vec<String>,
    pub track_names: Vec<String>,
    pub track_duration: Vec<i64>,
    pub artist_names: Vec<String>,
    pub album_names: Vec<String>,
    pub track_display: bool,
    pub track_state: TableState,
    pub track_index: usize,
    pub track_selected: bool, // for a track list that is selected
    pub current_playlist_selected: bool, // for a playlist that is selected
    pub enter_for_playback_in_made_fy: bool,
}

impl Default for MadeFY {
    fn default() -> Self {
        Self {
            playlist_names: Vec::new(),
            playlist_links: Vec::new(),
            selected: false,
            display: false,
            state: TableState::default(),
            index: 0,
            playlist_track_total: Vec::new(),
            track_links: Vec::new(),
            track_names: Vec::new(),
            track_duration: Vec::new(),
            artist_names: Vec::new(),
            album_names: Vec::new(),
            current_playlist_selected: false,
            track_display: false,
            track_state: TableState::default(),
            track_selected: false,
            track_index: 0,
            enter_for_playback_in_made_fy: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UserSavedPodcast{
    pub names: Vec<String>,
    pub links: Vec<String>,
    pub publisher: Vec<String>,
    pub selected: bool,
    pub display: bool,
    pub state: TableState,
    pub index: usize,
}

impl Default for UserSavedPodcast {
    fn default() -> Self {
        Self {
            names: Vec::new(),
            links: Vec::new(),
            publisher: Vec::new(),
            selected: false,
            display: false,
            state: TableState::default(),
            index: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UserRecentlyPlayed {
    pub recently_played_names: Vec<String>,
    pub recently_played_links: Vec<String>,
    pub recently_played_duration: Vec<i64>,
    pub recently_played_artist_names: Vec<String>,
    pub recently_played_album_names: Vec<String>,
    pub recently_played_selected: bool,
    pub recently_played_display: bool,
    pub recently_played_state: TableState,
    pub recently_played_index: usize,
    pub enter_for_playback_in_recently_played: bool,
}

impl Default for UserRecentlyPlayed {
    fn default() -> Self {
        Self {
            recently_played_names: Vec::new(),
            recently_played_links: Vec::new(),
            recently_played_duration: Vec::new(),
            recently_played_artist_names: Vec::new(),
            recently_played_album_names: Vec::new(),
            recently_played_selected: false,
            recently_played_display: false,
            recently_played_state: TableState::default(),
            recently_played_index: 0,
            enter_for_playback_in_recently_played: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UserSavedArtist {
    pub user_artist_names: Vec<String>,
    pub user_artist_links: Vec<String>,
    pub user_artist_selected: bool,
    pub user_artist_display: bool,
    pub user_artist_state: TableState,
    pub user_artist_index: usize,
    pub enter_for_playback_in_saved_artist: bool,
    pub user_artist_track_names: Vec<String>,
    pub user_artist_track_album: Vec<String>,
    pub user_artist_track_duration: Vec<i64>,
    pub user_artist_track_index: usize,
    pub user_artist_track_state: TableState,
    pub user_artist_track_display: bool,
    pub user_artist_track_selected: bool,
    pub user_artist_current_artist_selected: bool,
    pub user_artist_track_links: Vec<String>,
}

impl Default for UserSavedArtist {
    fn default() -> Self {
        Self {
            user_artist_index: 0,
            user_artist_names: Vec::new(),
            user_artist_links: Vec::new(),
            user_artist_selected: false,
            user_artist_display: false,
            user_artist_state: TableState::default(),
            user_artist_track_names: Vec::new(),
            user_artist_track_album: Vec::new(),
            user_artist_track_duration: Vec::new(),
            user_artist_track_index: 0,
            user_artist_track_state: TableState::default(),
            user_artist_track_display: false,
            user_artist_track_selected: false,
            user_artist_current_artist_selected: false,
            user_artist_track_links: Vec::new(),
            enter_for_playback_in_saved_artist: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct NewRelease {
    pub new_release_artist: Vec<String>,
    pub new_release_name: Vec<String>,
    pub new_release_state: ListState,
    pub current_new_release: String,
    pub new_release_display: bool,
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
}

impl Default for NewRelease {
    fn default() -> Self {
        Self {
            new_release_artist: Vec::new(),
            new_release_name: Vec::new(),
            new_release_state: ListState::default(),
            current_new_release: String::new(),
            new_release_display: false,
            new_release_album_selected: false,
            new_release_album_state: TableState::default(),
            new_release_album_links: Vec::new(),
            current_new_release_album: String::new(),
            current_new_release_album_link: String::new(),
            new_release_track_names: Vec::new(),
            new_release_artist_names: Vec::new(),
            new_release_durations_ms: Vec::new(),
            new_release_spotify_urls: Vec::new(),
            enter_for_playback_in_new_release: false,
            new_release_index: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UserCurrentlyPlaying {
    pub current_device_name: String,
    pub current_device_volume: String,
    pub playback_status: String,
    pub current_device_id: Option<String>,
    pub shuffle_status: String,
    pub repeat_status: String,
    pub is_shuffle: bool,
    pub current_timestamp: f64,
    pub ending_timestamp: f64,
    pub currently_playing_artist: String,
    pub current_playing_name: String,
    pub current_playing_id: String,
    pub current_playing_album: String,
    pub is_playing: bool,
    pub progress_bar_ratio: f64,
    pub currently_playing_media_type: String,
}

impl Default for UserCurrentlyPlaying {
    fn default() -> Self {
        Self {
            current_device_name: String::new(),
            current_device_volume: String::new(),
            playback_status: String::from("Playing"),
            shuffle_status: String::from("Off"),
            repeat_status: String::from("Off"),
            is_shuffle: false,
            current_device_id: Some(String::new()),
            current_timestamp: f64::from(0),
            ending_timestamp: f64::from(1),
            currently_playing_artist: String::new(),
            current_playing_name: String::new(),
            current_playing_id: String::new(),
            current_playing_album: String::new(),
            is_playing: false,
            progress_bar_ratio: 0.0,
            currently_playing_media_type: String::new(),
        }
    }
}