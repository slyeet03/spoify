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
}

impl Default for Search {
    fn default() -> Self {
        Self {
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
    pub user_playlist_names: Vec<String>,
    pub user_playlist_artist_names: Vec<String>,
    pub user_playlist_track_names: Vec<String>,
    pub user_playlist_album_names: Vec<String>,
    pub user_playlist_links: Vec<String>,
    pub user_playlist_track_links: Vec<String>,
    pub user_playlist_track_duration: Vec<i64>,
    pub current_user_playlist: String,
    pub selected_playlist_uri: String,
    pub user_playlist_display: bool,
    pub user_playlist_tracks_selected: bool,
    pub user_playlist_state: ListState,
    pub user_playlist_tracks_state: TableState,
    pub user_playlist_index: usize,
    pub enter_for_playback_in_user_playlist: bool,
}

impl Default for UserPlaylist {
    fn default() -> Self {
        Self {
            user_playlist_state: ListState::default(),
            user_playlist_names: Vec::new(),
            user_playlist_links: Vec::new(),
            user_playlist_track_names: Vec::new(),
            user_playlist_track_duration: Vec::new(),
            user_playlist_artist_names: Vec::new(),
            user_playlist_track_links: Vec::new(),
            user_playlist_album_names: Vec::new(),
            selected_playlist_uri: String::new(),
            current_user_playlist: String::new(),
            user_playlist_display: false,
            user_playlist_tracks_selected: false,
            user_playlist_tracks_state: TableState::default(),
            user_playlist_index: 0,
            enter_for_playback_in_user_playlist: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct LikedSongs {
    pub liked_song_names: Vec<String>,
    pub liked_song_links: Vec<String>,
    pub liked_song_duration: Vec<i64>,
    pub liked_song_artist_names: Vec<String>,
    pub liked_song_album_names: Vec<String>,
    pub liked_songs_selected: bool,
    pub liked_song_display: bool,
    pub liked_songs_state: TableState,
    pub liked_songs_index: usize,
    pub enter_for_playback_in_liked_song: bool,
}

impl Default for LikedSongs {
    fn default() -> Self {
        Self {
            liked_songs_state: TableState::default(),
            liked_song_names: Vec::new(),
            liked_song_links: Vec::new(),
            liked_song_duration: Vec::new(),
            liked_song_artist_names: Vec::new(),
            liked_songs_selected: false,
            liked_song_display: false,
            liked_song_album_names: Vec::new(),
            liked_songs_index: 0,
            enter_for_playback_in_liked_song: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UserSavedAlbums {
    pub user_album_names: Vec<String>,
    pub user_album_links: Vec<String>,
    pub user_album_artist_names: Vec<String>,
    pub user_album_tracks: Vec<usize>,
    pub user_album_selected: bool,
    pub user_album_display: bool,
    pub user_album_state: TableState,
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
}

impl Default for UserSavedAlbums {
    fn default() -> Self {
        Self {
            user_album_display: false,
            user_album_selected: false,
            user_album_state: TableState::default(),
            user_album_names: Vec::new(),
            user_album_links: Vec::new(),
            user_album_artist_names: Vec::new(),
            user_album_tracks: Vec::new(),
            user_album_index: 0,
            user_album_track_names: Vec::new(),
            user_album_track_artist: Vec::new(),
            user_album_track_duration: Vec::new(),
            user_album_track_index: 0,
            user_album_track_state: TableState::default(),
            user_album_track_display: false,
            user_album_track_selected: false,
            user_album_track_links: Vec::new(),
            user_album_current_album_selected: false,
            enter_for_playback_in_user_album: false,
        }
    }
}
