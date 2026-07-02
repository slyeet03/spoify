use crate::structs::NewRelease;
use crate::structs::UserSavedArtist;
use crate::structs::UserRecentlyPlayed;
use crate::structs::UserSavedPodcast;
use crate::structs::MadeFY;
use crate::UserSavedAlbums;
use crate::LikedSongs;
use crate::UserPlaylist;
use crate::structs::UserCurrentlyPlaying;
use super::util::{default, down_key_for_table, up_key_for_table};
use crate::{
    app::App,
    enums::{Library, Menu},
    structs::Search,
    spotify::{
        library_section::{
            liked_songs::{liked_tracks, process_liked_tracks},
            made_fy::{made_fy, process_made_fy},
            made_fy_tracks::{fetch_made_fy_tracks, process_made_fy_tracks},
            podcast::{process_podcasts, user_podcast},
            recently_played::{process_recently_played, recently_played},
            user_album_tracks::{process_user_album_tracks, user_album_tracks},
            user_albums::{process_user_albums, user_albums},
            user_artist_tracks::{process_user_artist_tracks, user_artist_tracks},
            user_artists::{process_user_artists, user_artists},
        },
        player::start_playback::start_playback,
    },
};

pub fn go_to_library_event(app: &mut App,search:&mut Search, userplaylist: &mut UserPlaylist, likedsongs: &mut LikedSongs, useralbum: &mut UserSavedAlbums, madefy: &mut MadeFY,podcast: &mut UserSavedPodcast, recentlyplayed: &mut UserRecentlyPlayed, userartist: &mut UserSavedArtist, newrelease: &mut NewRelease) {
    app.selected_menu = Menu::Library;
    app.library_state.select(Some(0)); //reseting the library state
    default(app,search,userplaylist,likedsongs,useralbum,madefy,podcast,recentlyplayed,userartist, newrelease);
}

pub fn library_down_event(app: &mut App, likedsongs: &mut LikedSongs, useralbum: &mut UserSavedAlbums, madefy: &mut MadeFY,podcast: &mut UserSavedPodcast, recentlyplayed: &mut UserRecentlyPlayed, userartist: &mut UserSavedArtist) {
    if app.selected_menu == Menu::Library {
        if app.library_state.selected() == Some(0) {
            if madefy.selected {
                (madefy.state, madefy.index) = down_key_for_table(
                    madefy.playlist_names.clone(),
                    madefy.state.clone(),
                );
            }
            if madefy.track_selected {
                (madefy.track_state, madefy.track_index) = down_key_for_table(
                    madefy.track_names.clone(),
                    madefy.track_state.clone(),
                );
            }
        }
        if app.library_state.selected() == Some(2) {
            if likedsongs.selected {
                (likedsongs.state, likedsongs.index) =
                    down_key_for_table(likedsongs.names.clone(), likedsongs.state.clone());
            }
        } else if app.library_state.selected() == Some(3) {
            if useralbum.selected {
                (useralbum.state, useralbum.index) =
                    down_key_for_table(useralbum.names.clone(), useralbum.state.clone());
            }
            if useralbum.track_selected {
                (useralbum.track_state, useralbum.track_index) = down_key_for_table(
                    useralbum.track_names.clone(),
                    useralbum.track_state.clone(),
                );
            }
        } else if app.library_state.selected() == Some(1) {
            if recentlyplayed.selected {
                (recentlyplayed.state, recentlyplayed.index) = down_key_for_table(
                    recentlyplayed.names.clone(),
                    recentlyplayed.state.clone(),
                );
            }
        } else if app.library_state.selected() == Some(5) {
            if podcast.selected {
                (podcast.state, podcast.index) =
                    down_key_for_table(podcast.names.clone(), podcast.state.clone());
            }
        } else if app.library_state.selected() == Some(4) {
            if userartist.user_artist_selected {
                (userartist.user_artist_state, userartist.user_artist_index) = down_key_for_table(
                    userartist.user_artist_names.clone(),
                    userartist.user_artist_state.clone(),
                );
            }
            if userartist.user_artist_track_selected {
                (userartist.user_artist_track_state, userartist.user_artist_track_index) = down_key_for_table(
                    userartist.user_artist_track_names.clone(),
                    userartist.user_artist_track_state.clone(),
                );
            }
        }
    }
}

pub fn library_up_event(app: &mut App, likedsongs: &mut LikedSongs, useralbum: &mut UserSavedAlbums, madefy: &mut MadeFY,podcast: &mut UserSavedPodcast, recentlyplayed: &mut UserRecentlyPlayed, userartist: &mut UserSavedArtist) {
    if app.selected_menu == Menu::Library {
        if app.library_state.selected() == Some(0) {
            if madefy.selected {
                (madefy.state, madefy.index) = up_key_for_table(
                    madefy.playlist_names.clone(),
                    madefy.state.clone(),
                );
            }
            if madefy.track_selected {
                (madefy.track_state, madefy.track_index) = up_key_for_table(
                    madefy.track_names.clone(),
                    madefy.track_state.clone(),
                );
            }
        }
        if app.library_state.selected() == Some(2) {
            if likedsongs.selected {
                (likedsongs.state, likedsongs.index) =
                    up_key_for_table(likedsongs.names.clone(), likedsongs.state.clone());
            }
        } else if app.library_state.selected() == Some(3) {
            if useralbum.selected {
                (useralbum.state, useralbum.index) =
                    up_key_for_table(useralbum.names.clone(), useralbum.state.clone());
            }
            if useralbum.track_selected {
                (useralbum.track_state, useralbum.track_index) = up_key_for_table(
                    useralbum.track_names.clone(),
                    useralbum.track_state.clone(),
                );
            }
        } else if app.library_state.selected() == Some(1) {
            if recentlyplayed.selected {
                (recentlyplayed.state, recentlyplayed.index) = up_key_for_table(
                    recentlyplayed.names.clone(),
                    recentlyplayed.state.clone(),
                )
            }
        } else if app.library_state.selected() == Some(5) {
            if podcast.selected {
                (podcast.state, podcast.index) =
                    up_key_for_table(podcast.names.clone(), podcast.state.clone());
            }
        } else if app.library_state.selected() == Some(4) {
            if userartist.user_artist_selected {
                (userartist.user_artist_state, userartist.user_artist_index) =
                    up_key_for_table(userartist.user_artist_names.clone(), userartist.user_artist_state.clone());
            }
            if userartist.user_artist_track_selected {
                (userartist.user_artist_track_state, userartist.user_artist_track_index) = up_key_for_table(
                    userartist.user_artist_track_names.clone(),
                    userartist.user_artist_track_state.clone(),
                );
            }
        }
    }
}

pub fn library_enter_event(app: &mut App,search: &mut Search, likedsongs: &mut LikedSongs, useralbum: &mut UserSavedAlbums, madefy: &mut MadeFY,podcast: &mut UserSavedPodcast, recentlyplayed: &mut UserRecentlyPlayed, userartist: &mut UserSavedArtist, currentlyplaying: &mut UserCurrentlyPlaying) {
    if app.selected_menu == Menu::Library {
        search.searched_album_selected = false;
        search.searched_artist_selected = false;
        search.searched_playlist_selected = false;
        if app.library_state.selected() == Some(0) {
            app.selected_library = Library::MadeFY;
            if madefy.current_playlist_selected {
                if let Err(e) = fetch_made_fy_tracks(app,madefy) {
                    println!("{}", e);
                }
                process_made_fy_tracks(app,madefy);
                madefy.track_display = true;
                madefy.display = false;
                madefy.track_selected = true;
                madefy.current_playlist_selected = false;
                madefy.selected = false;
                madefy.enter_for_playback_in_made_fy = true;
                madefy.track_state.select(Some(0));
            } else if madefy.enter_for_playback_in_made_fy {
                app.selected_link_for_playback =
                    madefy.track_links[madefy.track_index].clone();
                if let Err(e) = start_playback(app, currentlyplaying) {
                    println!("{}", e);
                }
            } else {
                if let Err(e) = made_fy(app) {
                    println!("{}", e);
                }
                process_made_fy(app,madefy);
                madefy.display = true;
                madefy.current_playlist_selected = true;
            }
        } else if app.library_state.selected() == Some(2) {
            app.selected_library = Library::LikedSongs;
            if likedsongs.enter_for_playback_in_liked_song {
                app.selected_link_for_playback =
                    likedsongs.links[likedsongs.index].clone();
                if let Err(e) = start_playback(app, currentlyplaying) {
                    println!("{}", e);
                }
            } else {
                if let Err(e) = liked_tracks(app) {
                    println!("{}", e);
                }
                process_liked_tracks(app,likedsongs);
                likedsongs.display = true;
                likedsongs.enter_for_playback_in_liked_song = true;
            }
        } else if app.library_state.selected() == Some(3) {
            app.selected_library = Library::Albums;
            if useralbum.current_album_selected {
                if let Err(e) = user_album_tracks(app,useralbum) {
                    println!("{}", e);
                }
                process_user_album_tracks(app,useralbum);
                useralbum.track_display = true;
                useralbum.display = false;
                useralbum.track_selected = true;
                useralbum.current_album_selected = false;
                useralbum.selected = false;
                useralbum.track_state.select(Some(0));
            } else if useralbum.enter_for_playback_in_user_album {
                app.selected_link_for_playback =
                    useralbum.track_links[useralbum.track_index].clone();
                if let Err(e) = start_playback(app, currentlyplaying) {
                    println!("{}", e);
                }
            } else {
                if let Err(e) = user_albums(app) {
                    println!("{}", e);
                }
                process_user_albums(app,useralbum);
                useralbum.display = true;
                useralbum.current_album_selected = true;
                useralbum.enter_for_playback_in_user_album = true;
            }
        } else if app.library_state.selected() == Some(1) {
            app.selected_library = Library::RecentlyPlayed;
            if recentlyplayed.enter_for_playback_in_recently_played {
                app.selected_link_for_playback =
                    recentlyplayed.links[recentlyplayed.index].clone();
                if let Err(e) = start_playback(app, currentlyplaying) {
                    println!("{}", e);
                }
            } else {
                if let Err(e) = recently_played(app) {
                    println!("{}", e);
                }
                process_recently_played(app,recentlyplayed);
                recentlyplayed.display = true;
                recentlyplayed.enter_for_playback_in_recently_played = true;
            }
        } else if app.library_state.selected() == Some(5) {
            app.selected_library = Library::Podcasts;
            if let Err(e) = user_podcast(app) {
                println!("{}", e);
            }
            process_podcasts(app,podcast);
            podcast.display = true;
        } else if app.library_state.selected() == Some(4) {
            app.selected_library = Library::Artists;
            if userartist.user_artist_current_artist_selected {
                if let Err(e) = user_artist_tracks(app,userartist) {
                    println!("{}", e);
                }
                process_user_artist_tracks(app,search,userartist);
                userartist.user_artist_track_display = true;
                userartist.user_artist_display = false;
                userartist.user_artist_track_selected = true;
                userartist.user_artist_current_artist_selected = false;
                userartist.user_artist_selected = false;
                userartist.user_artist_track_state.select(Some(0));
            } else if userartist.enter_for_playback_in_saved_artist {
                app.selected_link_for_playback =
                    userartist.user_artist_track_links[userartist.user_artist_track_index].clone();
                if let Err(e) = start_playback(app, currentlyplaying) {
                    println!("{}", e);
                }
            } else {
                if let Err(e) = user_artists(app) {
                    println!("{}", e);
                }
                process_user_artists(app,userartist);
                userartist.user_artist_display = true;
                userartist.user_artist_current_artist_selected = true;
                userartist.enter_for_playback_in_saved_artist = true;
            }
        }
    }
}

pub fn library_tab_event(app: &mut App, likedsongs: &mut LikedSongs, useralbum: &mut UserSavedAlbums, madefy: &mut MadeFY,podcast: &mut UserSavedPodcast, recentlyplayed: &mut UserRecentlyPlayed, userartist: &mut UserSavedArtist) {
    if app.selected_menu == Menu::Library {
        app.can_navigate_menu = !app.can_navigate_menu;
        if app.library_state.selected() == Some(0) && madefy.display {
            madefy.state.select(Some(0));
            madefy.selected = !madefy.selected;
        } else if app.library_state.selected() == Some(2) && likedsongs.display {
            likedsongs.state.select(Some(0));
            likedsongs.selected = !likedsongs.selected;
        } else if app.library_state.selected() == Some(3) && useralbum.display {
            useralbum.state.select(Some(0));
            useralbum.selected = !useralbum.selected;
        } else if app.library_state.selected() == Some(1) && recentlyplayed.display {
            recentlyplayed.state.select(Some(0));
            recentlyplayed.selected = !recentlyplayed.selected;
        } else if app.library_state.selected() == Some(5) && podcast.display {
            podcast.state.select(Some(0));
            podcast.selected = !podcast.selected;
        } else if app.library_state.selected() == Some(4) && userartist.user_artist_display {
            userartist.user_artist_state.select(Some(0));
            userartist.user_artist_selected = !userartist.user_artist_selected;
        }
    }
}
