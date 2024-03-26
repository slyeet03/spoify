use crate::enums::{InputMode, Library, Menu};
use crate::handlers::key_event::handle_events;
use crate::handlers::key_event::search_input;
use crate::ui::homepage::render_frame;
use crate::ui::tui;

use ratatui::widgets::ListState;

use std::io;

pub struct App {
    pub exit: bool, //to control app's exit
    pub selected_library: Library,
    pub selected_menu: Menu,
    pub library_index: usize,
    pub library_state: ListState,
    pub search_query: String,
    pub input: String,
    pub cursor_position: usize,
    pub input_mode: InputMode,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            // drawing the ui
            terminal.draw(|frame| render_frame(frame, self.selected_menu, self))?;
            // Handling user inputs
            if self.selected_menu == Menu::Search {
                if self.input_mode == InputMode::Editing {
                    search_input(self)?;
                } else {
                    handle_events(self)?;
                }
            } else {
                handle_events(self)?;
            }
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
            selected_menu: Menu::Main,
            library_index: 0,
            library_state: ListState::default(),
            search_query: "".to_string(),
            input: String::new(),
            input_mode: InputMode::Normal,
            cursor_position: 0,
        }
    }
}
