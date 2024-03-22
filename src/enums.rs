#[derive(Copy, Clone, Debug)]
pub enum Library {
    MadeFY,
    RecentlyPlayed,
    LikedSongs,
    Albums,
    Artists,
    Podcasts,
}
#[derive(Copy, Clone, Debug)]
pub enum Menu {
    Library,
    Playlists,
    Search,
    Main,
    Default,
}

impl From<Menu> for usize {
    fn from(input: Menu) -> usize {
        match input {
            Menu::Library => 0,
            Menu::Playlists => 1,
            Menu::Search => 2,
            Menu::Main => 3,
            Menu::Default => 4,
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
