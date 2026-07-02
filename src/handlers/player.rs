use crate::{
    app::App,
    enums::Menu,
    spotify::player::{
        next_track::next_track, pause_playback::pause, play_playback::play,
        previous_track::previous_track, repeat::cycle_repeat, shuffle::toogle_shuffle,
        volume_decrease::volume_decreament, volume_increase::volume_increment,
    },
    structs::{Settings,UserCurrentlyPlaying},
};

pub fn fullscreen_player_event(app: &mut App) {
    if app.selected_menu == Menu::Player {
        app.selected_menu = Menu::Default;
    } else {
        app.selected_menu = Menu::Player;
    }
}

pub fn repeat_event(app: &mut App, currentlyplaying: &mut UserCurrentlyPlaying) {
    if let Err(e) = cycle_repeat(app, currentlyplaying) {
        println!("{}", e);
    }
}

pub fn shuffle_event(app: &mut App, currentlyplaying: &mut UserCurrentlyPlaying) {
    currentlyplaying.is_shuffle = !currentlyplaying.is_shuffle;
    if let Err(e) = toogle_shuffle(app, currentlyplaying) {
        println!("{}", e);
    }
}

pub fn volume_decreament_event(app: &mut App, settings: &mut Settings) {
    if let Err(e) = volume_decreament(app, settings) {
        println!("{}", e);
    }
}

pub fn volume_increment_event(app: &mut App, settings: &mut Settings) {
    if let Err(e) = volume_increment(app, settings) {
        println!("{}", e);
    }
}

pub fn next_track_event(app: &mut App, currentlyplaying: &mut UserCurrentlyPlaying) {
    if let Err(e) = next_track(app, currentlyplaying) {
        println!("{}", e);
    }
}

pub fn previous_track_event(app: &mut App, currentlyplaying: &mut UserCurrentlyPlaying) {
    if let Err(e) = previous_track(app, currentlyplaying) {
        println!("{}", e);
    }
}

pub fn play_pause_event(app: &mut App, currentlyplaying: &mut UserCurrentlyPlaying) {
    if currentlyplaying.playback_status == "Paused" {
        if let Err(e) = play(app, currentlyplaying) {
            println!("{}", e);
        }
    } else if currentlyplaying.playback_status == "Playing" {
        if let Err(e) = pause(app, currentlyplaying) {
            println!("{}", e);
        }
    }
}
