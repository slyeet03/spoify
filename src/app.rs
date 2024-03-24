use crate::enums::{Library, Menu};
use crate::handlers::key_event::handle_events;
use crate::ui::homepage::render_frame;
use crate::ui::tui;

use std::io;

pub struct App {
    pub exit: bool,
    pub selected_library: Library,
    pub selected_menu: Menu,
    pub selected_index: usize,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| render_frame(frame, self.selected_menu, self.selected_index))?;
            self.selected_index = handle_events(self, self.selected_index)?;
        }
        Ok(())
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            exit: false,
            selected_library: Library::MadeFY,
            selected_menu: Menu::Default,
            selected_index: 0,
        } // Set initial selection
    }
}
