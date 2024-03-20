#[warn(unused_imports)]
use std::io;

use crate::enums::Library; // Import Library from enums.rs
use crate::enums::Menu;
use crate::ui::render_frame; // Import Menu from enums.rs

mod enums;
mod key_event_handler;
mod tui;
mod ui;

pub struct App {
    pub exit: bool,
    pub selected_library: Library,
    pub selected_menu: Menu,
}

impl Default for App {
    fn default() -> Self {
        Self {
            exit: false,
            selected_library: Library::MadeFY,
            selected_menu: Menu::Library,
        } // Set initial selection
    }
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| render_frame(frame, self.selected_library))?;
            key_event_handler::handle_events(self)?;
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let mut app = App::default();
    let app_result = app.run(&mut terminal);
    tui::restore()?;
    app_result
}
