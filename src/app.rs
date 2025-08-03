use crate::NewRelease;
use crate::structs::UserSavedArtist;
use crate::structs::UserRecentlyPlayed;
use crate::structs::UserSavedPodcast;
use crate::structs::MadeFY;
use crate::structs::UserSavedAlbums;
use crate::enums::{InputMode, Library, Menu};
use crate::handlers::key_event::handle_key_event;
use crate::handlers::key_event::search_input;
use crate::spotify::player::player::process_currently_playing;
use crate::structs::LikedSongs;
use crate::structs::{Key, Search, Settings, Themes};
use crate::ui::tui;
use crate::ui::ui::render_frame;
use crate::UserPlaylist;
use crossterm::event::{self, Event};
use ratatui::widgets::{ListState};
use std::io;
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct App {
    pub file_name: String,
    // Controls whether the application should exit
    pub exit: bool,

    // Stores the device id to resume playback
    pub device_id_after_pause: Option<String>,

    // Controls the navigation inside Menu
    pub selected_menu: Menu,
    pub can_navigate_menu: bool,

    // Controls the navigation inside Library
    pub selected_library: Library,
    pub library_state: ListState,

    // Handles Search function
    // Handles User's playlists

    // Handles User's Liked Songs

    // Handles User's Saved Albums

    // Handles User's Saved Podcasts
    

    // Handles User's Recently Played Songs
    

    // Handles User's Saved Artists
    

    // Handles Made For You
    

    // Handles User's currently playing device
    

    // Handle New Release section
    

    // Creds
    pub client_id: String,
    pub client_secret: String,

    // Error
    pub error_text: String,

    // Playback
    pub selected_link_for_playback: String,
    pub is_only_id: bool,
    pub is_in_track: bool,
    // Top Tracks
    pub top_tracks_all_time_names: Vec<String>,
    pub top_tracks_6_months_names: Vec<String>,
    pub top_tracks_4_weeks_names: Vec<String>,

    // Add track to playlist
    pub add_track_to_playlist_state: ListState,
    pub track_added_to_playlist_name: String,
    pub playlist_index_for_track_addition: usize,
    pub track_added_to_playlist_link: String,
    pub playlist_link_for_track_addition: String,

    // Follow/Unfollow Playlist
    pub playlist_link_to_follow: String,
    pub have_playlist: bool,
}

impl App {
    /// Runs the application's main loop until the user quits
    pub fn run(
        &mut self,
        terminal: &mut tui::Tui,
        rx1: Receiver<()>,
        keys: &mut Key,
        theme: &mut Themes,
        settings: &mut Settings,
        search: &mut Search,
        userplaylist: &mut UserPlaylist,
        likedsongs: &mut LikedSongs,
        useralbum: &mut UserSavedAlbums, 
        madefy: &mut MadeFY,
        podcast: &mut UserSavedPodcast, 
        recentlyplayed: &mut UserRecentlyPlayed, 
        userartist: &mut UserSavedArtist, 
        newrelease: &mut NewRelease
    ) -> io::Result<()> {
        let mut last_tick: Instant = Instant::now();
        // Set the duration for refreshing UI
        let timeout: Duration = Duration::from_millis(200);

        while !self.exit {
            // Handling user inputs
            if event::poll(timeout)? {
                if let Event::Key(key_event) = event::read()? {
                    handle_key_event(
                        self,
                        key_event,
                        keys,
                        theme,
                        settings,
                        search,
                        userplaylist,
                        likedsongs,
                        useralbum,
                        madefy,
                        podcast,
                        recentlyplayed,
                        userartist,
                        newrelease
                    );

                    // In editing mode, handle search input
                    if search.input_mode == InputMode::Editing {
                        let _ = search_input(self, search, key_event);
                    }
                }
            }

            // Update UI
            let now: Instant = std::time::Instant::now();
            if now.duration_since(last_tick) >= timeout {
                last_tick = now;

                // Check if a message has been received from the player info update thread
                if rx1.try_recv().is_ok() {
                    process_currently_playing(self, settings);
                }

                // Draw the UI
                terminal.draw(|frame| {
                    render_frame(
                        frame,
                        self.selected_menu,
                        self,
                        keys,
                        theme,
                        search,
                        userplaylist,
                        likedsongs,
                        useralbum,
                        madefy,
                        podcast,
                        recentlyplayed,
                        userartist,
                        newrelease
                    )
                })?;
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

            selected_menu: Menu::Default,

            selected_library: Library::MadeFY,
            library_state: ListState::default(),

            can_navigate_menu: true,

           
           


        
            device_id_after_pause: Some(String::new()),

        
           

            client_id: String::new(),
            client_secret: String::new(),

            
          
            
           

            error_text: String::new(),

            

           

            selected_link_for_playback: String::new(),

            
            
            
            

            is_only_id: false,
            is_in_track: false,

            top_tracks_all_time_names: Vec::new(),
            top_tracks_6_months_names: Vec::new(),
            top_tracks_4_weeks_names: Vec::new(),
            file_name: String::new(),

            track_added_to_playlist_name: String::new(),
            playlist_index_for_track_addition: 0,
            track_added_to_playlist_link: String::new(),
            playlist_link_for_track_addition: String::new(),
            add_track_to_playlist_state: ListState::default(),

            playlist_link_to_follow: String::new(),
            have_playlist: true,
        }
    }
}
