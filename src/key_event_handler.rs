use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::enums::Library; // Import Library from enums.rs
use crate::enums::Menu;
use crate::App; // Import Menu from enums.rs

use std::io;

pub fn handle_events(app: &mut App, selected_index: usize) -> io::Result<(usize)> {
    let mut selected_index = selected_index;

    match event::read()? {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            handle_key_event(app, key_event, &mut selected_index);
        }
        _ => {}
    };
    Ok(selected_index)
}

fn handle_key_event(app: &mut App, key_event: KeyEvent, selected_index: &mut usize) {
    match key_event.code {
        KeyCode::Char('q') => app.exit(),
        KeyCode::Char('l') => app.selected_menu = Menu::Library,
        KeyCode::Char('p') => app.selected_menu = Menu::Playlists,
        KeyCode::Char('s') => app.selected_menu = Menu::Search,
        KeyCode::Char('m') => app.selected_menu = Menu::Main,
        KeyCode::Char('d') => app.selected_menu = Menu::Default,
        KeyCode::Down => {
            match app.selected_menu {
                Menu::Library => {
                    if *selected_index < (Library::Podcasts as usize) {
                        *selected_index += 1;
                    }
                }
                _ => {} // Handle down arrow key for other menus
            }
        }
        KeyCode::Up => {
            match app.selected_menu {
                Menu::Library => {
                    if *selected_index > 0 {
                        *selected_index -= 1;
                    }
                }
                _ => {} // Handle up arrow key for other menus
            }
        }
        _ => {}
    }
}
