use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

mod tui;

struct App {
    exit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self { exit: false }
    }
}

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let mut app = App::default(); // Create an App instance directly
    let app_result = app.run(&mut terminal);
    tui::restore()?;
    app_result
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, f: &mut Frame) {
        let search_block = Block::default()
            .borders(Borders::ALL)
            .title(Title::from("Search"));
        let help_block = Block::default()
            .borders(Borders::ALL)
            .title(Title::from("Help"));
        let library_block = Block::default()
            .borders(Borders::ALL)
            .title(Title::from("Library"));
        let playlist_block = Block::default()
            .borders(Borders::ALL)
            .title(Title::from("Playlist"));
        let player_block = Block::default()
            .borders(Borders::ALL)
            .title(Title::from("Player"));
        let content_block = Block::default()
            .borders(Borders::ALL)
            .title(Title::from("Content"));

        let size = f.size();
        // main display
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(70),
                Constraint::Percentage(20),
            ])
            .split(size);
        // search and help display
        let header_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(chunks[0]);
        // library, playlist and main content display
        let content_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
            .split(chunks[1]);
        // library and playlist
        let content_sub_chunk = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(content_chunk[0]);

        f.render_widget(search_block, header_chunk[0]);
        f.render_widget(help_block, header_chunk[1]);
        f.render_widget(library_block, content_sub_chunk[0]);
        f.render_widget(playlist_block, content_sub_chunk[1]);
        f.render_widget(player_block, chunks[2]);
        f.render_widget(content_block, content_chunk[1])
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }
}
