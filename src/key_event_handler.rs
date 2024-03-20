use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::App;

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
        _ => {}
    }
}
