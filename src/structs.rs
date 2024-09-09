use ratatui::style::Color;

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
