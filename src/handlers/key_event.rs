use crate::app::App;
use crate::enums::Menu;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

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
        KeyCode::Down => {}
        KeyCode::Up => {}
        _ => {}
    }
}
