#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Library {
    MadeFY,
    RecentlyPlayed,
    LikedSongs,
    Albums,
    Artists,
    Podcasts,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Menu {
    Library,
    Playlists,
    Search,
    Main,
    Default,
    Help,
    NewRelease,
    Lyrics,
    Error,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SearchMenu {
    Default,
    SearchedAlbum,
    SearchedArtist,
    SearchedPlaylist,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
    SearchResults,
}

impl From<Menu> for usize {
    fn from(input: Menu) -> usize {
        match input {
            Menu::Library => 0,
            Menu::Playlists => 1,
            Menu::Search => 2,
            Menu::Main => 3,
            Menu::Default => 4,
            Menu::Help => 5,
            Menu::NewRelease => 6,
            Menu::Lyrics => 7,
            Menu::Error => 8,
        }
    }
}
impl From<Library> for usize {
    fn from(input: Library) -> usize {
        match input {
            Library::MadeFY => 0,
            Library::RecentlyPlayed => 1,
            Library::LikedSongs => 2,
            Library::Albums => 3,
            Library::Artists => 4,
            Library::Podcasts => 5,
        }
    }
}
impl From<SearchMenu> for usize {
    fn from(input: SearchMenu) -> usize {
        match input {
            SearchMenu::Default => 0,
            SearchMenu::SearchedAlbum => 1,
            SearchMenu::SearchedArtist => 2,
            SearchMenu::SearchedPlaylist => 3,
        }
    }
}
