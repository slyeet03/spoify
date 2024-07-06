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
    pub lyrics_key: char,
    pub next_track_key: char,
    pub previous_track_key: char,
    pub error_key: char,
    pub player_fullscreen_key: char,
    pub change_keybind: char,

    pub first_keys: Vec<String>,
    pub tasks: Vec<String>,
}

impl Default for Key {
    fn default() -> Self {
        Self {
            go_to_search_key: ' ',
            go_to_library_key: ' ',
            go_to_user_playlists_key: ' ',
            exit_application_key: ' ',
            pause_play_key: ' ',
            help_key: ' ',
            volume_up_key: ' ',
            volume_down_key: ' ',
            new_release_key: ' ',
            lyrics_key: ' ',
            next_track_key: ' ',
            previous_track_key: ' ',
            error_key: ' ',
            player_fullscreen_key: ' ',
            change_keybind: ' ',

            first_keys: Vec::new(),
            tasks: Vec::new(),
        }
    }
}
