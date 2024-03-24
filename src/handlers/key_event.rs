use crate::app::App;
use crate::enums::Menu;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use std::io;

pub fn handle_events(app: &mut App) -> io::Result<()> {
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
        KeyCode::Char('l') => {
            app.selected_menu = Menu::Library;
            app.library_state.select(Some(0)); // Reset the library_state when switching to Library menu
        }
        KeyCode::Char('p') => app.selected_menu = Menu::Playlists,
        KeyCode::Char('s') => app.selected_menu = Menu::Search,
        KeyCode::Char('m') => app.selected_menu = Menu::Main,
        KeyCode::Down if app.selected_menu == Menu::Library => {
            let next_index = app.library_state.selected().unwrap_or(0) + 1;
            app.library_state.select(Some(next_index % 6)); // Wrap around when reaching the end
        }
        KeyCode::Up if app.selected_menu == Menu::Library => {
            let prev_index = if app.library_state.selected().unwrap_or(0) == 0 {
                5 // Wrap around to the last item
            } else {
                app.library_state.selected().unwrap_or(0) - 1
            };
            app.library_state.select(Some(prev_index));
        }
        _ => {}
    }
}
