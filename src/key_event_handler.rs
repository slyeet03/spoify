use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::enums::Library; // Import Library from enums.rs
use crate::enums::Menu;
use crate::App; // Import Menu from enums.rs

pub fn handle_events(app: &mut App) -> std::io::Result<()> {
    match event::read()? {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            handle_key_event(app, key_event);
        }
        _ => {}
    };
    Ok(())
}

fn handle_key_event(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => app.exit(),
        KeyCode::Char('l') => app.selected_menu = Menu::Library,
        KeyCode::Char('p') => app.selected_menu = Menu::Playlists,
        KeyCode::Char('s') => app.selected_menu = Menu::Search,
        KeyCode::Char('m') => app.selected_menu = Menu::Main,
        KeyCode::Char('d') => app.selected_menu = Menu::Default,
        KeyCode::Down => {
            match app.selected_menu {
                Menu::Library => match app.selected_library {
                    Library::MadeFY => app.selected_library = Library::RecentlyPlayed,
                    Library::RecentlyPlayed => app.selected_library = Library::LikedSongs,
                    Library::LikedSongs => app.selected_library = Library::Albums,
                    Library::Albums => app.selected_library = Library::Artists,
                    Library::Artists => app.selected_library = Library::Podcasts,
                    Library::Podcasts => app.selected_library = Library::MadeFY,
                },
                // Handle up arrow key for playlists and other menus
                _ => {}
            }
        }
        KeyCode::Down => {}
        _ => {}
    }
}
