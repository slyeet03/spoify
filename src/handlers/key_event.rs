use crate::app::App;
use crate::enums::Menu;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use std::io;

pub fn handle_events(app: &mut App) -> io::Result<()> {
    match event::read()? {
        //handling key press events
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            handle_key_event(app, key_event);
        }
        _ => {}
    };

    Ok(())
}

fn handle_key_event(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        //hadling key events
        KeyCode::Char('q') => app.exit(),
        KeyCode::Char('l') => {
            app.selected_menu = Menu::Library;
            app.library_state.select(Some(0)); //reseting the library state
        }
        KeyCode::Char('p') => app.selected_menu = Menu::Playlists,
        KeyCode::Char('s') => app.selected_menu = Menu::Search,
        KeyCode::Char('m') => app.selected_menu = Menu::Main,
        KeyCode::Down if app.selected_menu == Menu::Library => {
            //move down in the library list
            let next_index = app.library_state.selected().unwrap_or(0) + 1;
            app.library_state.select(Some(next_index % 6)); //wrapping around the last option
        }
        KeyCode::Up if app.selected_menu == Menu::Library => {
            //move up in the library list
            let prev_index = if app.library_state.selected().unwrap_or(0) == 0 {
                5 //wrapping to the last option when user presses up at the first option
            } else {
                app.library_state.selected().unwrap_or(0) - 1
            };
            app.library_state.select(Some(prev_index));
        }
        _ => {}
    }
}
