use crate::structs::UserSavedPodcast;
use crate::structs::MadeFY;
use crate::UserSavedAlbums;
use crate::LikedSongs;
use crate::UserPlaylist;
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

pub fn go_to_library_event(app: &mut App,search:&mut Search, userplaylist: &mut UserPlaylist, likedsongs: &mut LikedSongs, useralbum: &mut UserSavedAlbums, madefy: &mut MadeFY,podcast: &mut UserSavedPodcast) {
    app.selected_menu = Menu::Library;
    app.library_state.select(Some(0)); //reseting the library state
    default(app,search,userplaylist,likedsongs,useralbum,madefy,podcast);
}

pub fn library_down_event(app: &mut App, likedsongs: &mut LikedSongs, useralbum: &mut UserSavedAlbums, madefy: &mut MadeFY,podcast: &mut UserSavedPodcast) {
    if app.selected_menu == Menu::Library {
        if app.library_state.selected() == Some(0) {
            if madefy.made_fy_selected {
                (madefy.made_fy_state, madefy.made_fy_index) = down_key_for_table(
                    madefy.made_fy_playlist_names.clone(),
                    madefy.made_fy_state.clone(),
                );
            }
            if madefy.made_fy_track_selected {
                (madefy.made_fy_track_state, madefy.made_fy_track_index) = down_key_for_table(
                    madefy.made_fy_track_names.clone(),
                    madefy.made_fy_track_state.clone(),
                );
            }
        }
        if app.library_state.selected() == Some(2) {
            if likedsongs.liked_songs_selected {
                (likedsongs.liked_songs_state, likedsongs.liked_songs_index) =
                    down_key_for_table(likedsongs.liked_song_names.clone(), likedsongs.liked_songs_state.clone());
            }
        } else if app.library_state.selected() == Some(3) {
            if useralbum.user_album_selected {
                (useralbum.user_album_state, useralbum.user_album_index) =
                    down_key_for_table(useralbum.user_album_names.clone(), useralbum.user_album_state.clone());
            }
            if useralbum.user_album_track_selected {
                (useralbum.user_album_track_state, useralbum.user_album_track_index) = down_key_for_table(
                    useralbum.user_album_track_names.clone(),
                    useralbum.user_album_track_state.clone(),
                );
            }
        } else if app.library_state.selected() == Some(1) {
            if app.recently_played_selected {
                (app.recently_played_state, app.recently_played_index) = down_key_for_table(
                    app.recently_played_names.clone(),
                    app.recently_played_state.clone(),
                );
            }
        } else if app.library_state.selected() == Some(5) {
            if podcast.podcast_selected {
                (podcast.podcast_state, podcast.podcast_index) =
                    down_key_for_table(podcast.podcast_names.clone(), podcast.podcast_state.clone());
            }
        } else if app.library_state.selected() == Some(4) {
            if app.user_artist_selected {
                (app.user_artist_state, app.user_artist_index) = down_key_for_table(
                    app.user_artist_names.clone(),
                    app.user_artist_state.clone(),
                );
            }
            if app.user_artist_track_selected {
                (app.user_artist_track_state, app.user_artist_track_index) = down_key_for_table(
                    app.user_artist_track_names.clone(),
                    app.user_artist_track_state.clone(),
                );
            }
        }
    }
}

pub fn library_up_event(app: &mut App, likedsongs: &mut LikedSongs, useralbum: &mut UserSavedAlbums, madefy: &mut MadeFY,podcast: &mut UserSavedPodcast) {
    if app.selected_menu == Menu::Library {
        if app.library_state.selected() == Some(0) {
            if madefy.made_fy_selected {
                (madefy.made_fy_state, madefy.made_fy_index) = up_key_for_table(
                    madefy.made_fy_playlist_names.clone(),
                    madefy.made_fy_state.clone(),
                );
            }
            if madefy.made_fy_track_selected {
                (madefy.made_fy_track_state, madefy.made_fy_track_index) = up_key_for_table(
                    madefy.made_fy_track_names.clone(),
                    madefy.made_fy_track_state.clone(),
                );
            }
        }
        if app.library_state.selected() == Some(2) {
            if likedsongs.liked_songs_selected {
                (likedsongs.liked_songs_state, likedsongs.liked_songs_index) =
                    up_key_for_table(likedsongs.liked_song_names.clone(), likedsongs.liked_songs_state.clone());
            }
        } else if app.library_state.selected() == Some(3) {
            if useralbum.user_album_selected {
                (useralbum.user_album_state, useralbum.user_album_index) =
                    up_key_for_table(useralbum.user_album_names.clone(), useralbum.user_album_state.clone());
            }
            if useralbum.user_album_track_selected {
                (useralbum.user_album_track_state, useralbum.user_album_track_index) = up_key_for_table(
                    useralbum.user_album_track_names.clone(),
                    useralbum.user_album_track_state.clone(),
                );
            }
        } else if app.library_state.selected() == Some(1) {
            if app.recently_played_selected {
                (app.recently_played_state, app.recently_played_index) = up_key_for_table(
                    app.recently_played_names.clone(),
                    app.recently_played_state.clone(),
                )
            }
        } else if app.library_state.selected() == Some(5) {
            if podcast.podcast_selected {
                (podcast.podcast_state, podcast.podcast_index) =
                    up_key_for_table(podcast.podcast_names.clone(), podcast.podcast_state.clone());
            }
        } else if app.library_state.selected() == Some(4) {
            if app.user_artist_selected {
                (app.user_artist_state, app.user_artist_index) =
                    up_key_for_table(app.user_artist_names.clone(), app.user_artist_state.clone());
            }
            if app.user_artist_track_selected {
                (app.user_artist_track_state, app.user_artist_track_index) = up_key_for_table(
                    app.user_artist_track_names.clone(),
                    app.user_artist_track_state.clone(),
                );
            }
        }
    }
}

pub fn library_enter_event(app: &mut App,search: &mut Search, likedsongs: &mut LikedSongs, useralbum: &mut UserSavedAlbums, madefy: &mut MadeFY,podcast: &mut UserSavedPodcast) {
    if app.selected_menu == Menu::Library {
        search.searched_album_selected = false;
        search.searched_artist_selected = false;
        search.searched_playlist_selected = false;
        if app.library_state.selected() == Some(0) {
            app.selected_library = Library::MadeFY;
            if madefy.made_fy_current_playlist_selected {
                if let Err(e) = fetch_made_fy_tracks(app,madefy) {
                    println!("{}", e);
                }
                process_made_fy_tracks(app,madefy);
                madefy.made_fy_track_display = true;
                madefy.made_fy_display = false;
                madefy.made_fy_track_selected = true;
                madefy.made_fy_current_playlist_selected = false;
                madefy.made_fy_selected = false;
                madefy.enter_for_playback_in_made_fy = true;
                madefy.made_fy_track_state.select(Some(0));
            } else if madefy.enter_for_playback_in_made_fy {
                app.selected_link_for_playback =
                    madefy.made_fy_track_links[madefy.made_fy_track_index].clone();
                if let Err(e) = start_playback(app) {
                    println!("{}", e);
                }
            } else {
                if let Err(e) = made_fy(app) {
                    println!("{}", e);
                }
                process_made_fy(app,madefy);
                madefy.made_fy_display = true;
                madefy.made_fy_current_playlist_selected = true;
            }
        } else if app.library_state.selected() == Some(2) {
            app.selected_library = Library::LikedSongs;
            if likedsongs.enter_for_playback_in_liked_song {
                app.selected_link_for_playback =
                    likedsongs.liked_song_links[likedsongs.liked_songs_index].clone();
                if let Err(e) = start_playback(app) {
                    println!("{}", e);
                }
            } else {
                if let Err(e) = liked_tracks(app) {
                    println!("{}", e);
                }
                process_liked_tracks(app,likedsongs);
                likedsongs.liked_song_display = true;
                likedsongs.enter_for_playback_in_liked_song = true;
            }
        } else if app.library_state.selected() == Some(3) {
            app.selected_library = Library::Albums;
            if useralbum.user_album_current_album_selected {
                if let Err(e) = user_album_tracks(app,useralbum) {
                    println!("{}", e);
                }
                process_user_album_tracks(app,useralbum);
                useralbum.user_album_track_display = true;
                useralbum.user_album_display = false;
                useralbum.user_album_track_selected = true;
                useralbum.user_album_current_album_selected = false;
                useralbum.user_album_selected = false;
                useralbum.user_album_track_state.select(Some(0));
            } else if useralbum.enter_for_playback_in_user_album {
                app.selected_link_for_playback =
                    useralbum.user_album_track_links[useralbum.user_album_track_index].clone();
                if let Err(e) = start_playback(app) {
                    println!("{}", e);
                }
            } else {
                if let Err(e) = user_albums(app) {
                    println!("{}", e);
                }
                process_user_albums(app,useralbum);
                useralbum.user_album_display = true;
                useralbum.user_album_current_album_selected = true;
                useralbum.enter_for_playback_in_user_album = true;
            }
        } else if app.library_state.selected() == Some(1) {
            app.selected_library = Library::RecentlyPlayed;
            if app.enter_for_playback_in_recently_played {
                app.selected_link_for_playback =
                    app.recently_played_links[app.recently_played_index].clone();
                if let Err(e) = start_playback(app) {
                    println!("{}", e);
                }
            } else {
                if let Err(e) = recently_played(app) {
                    println!("{}", e);
                }
                process_recently_played(app);
                app.recently_played_display = true;
                app.enter_for_playback_in_recently_played = true;
            }
        } else if app.library_state.selected() == Some(5) {
            app.selected_library = Library::Podcasts;
            if let Err(e) = user_podcast(app) {
                println!("{}", e);
            }
            process_podcasts(app,podcast);
            podcast.podcast_display = true;
        } else if app.library_state.selected() == Some(4) {
            app.selected_library = Library::Artists;
            if app.user_artist_current_artist_selected {
                if let Err(e) = user_artist_tracks(app) {
                    println!("{}", e);
                }
                process_user_artist_tracks(app,search);
                app.user_artist_track_display = true;
                app.user_artist_display = false;
                app.user_artist_track_selected = true;
                app.user_artist_current_artist_selected = false;
                app.user_artist_selected = false;
                app.user_artist_track_state.select(Some(0));
            } else if app.enter_for_playback_in_saved_artist {
                app.selected_link_for_playback =
                    app.user_artist_track_links[app.user_artist_track_index].clone();
                if let Err(e) = start_playback(app) {
                    println!("{}", e);
                }
            } else {
                if let Err(e) = user_artists(app) {
                    println!("{}", e);
                }
                process_user_artists(app);
                app.user_artist_display = true;
                app.user_artist_current_artist_selected = true;
                app.enter_for_playback_in_saved_artist = true;
            }
        }
    }
}

pub fn library_tab_event(app: &mut App, likedsongs: &mut LikedSongs, useralbum: &mut UserSavedAlbums, madefy: &mut MadeFY,podcast: &mut UserSavedPodcast) {
    if app.selected_menu == Menu::Library {
        app.can_navigate_menu = !app.can_navigate_menu;
        if app.library_state.selected() == Some(0) && madefy.made_fy_display {
            madefy.made_fy_state.select(Some(0));
            madefy.made_fy_selected = !madefy.made_fy_selected;
        } else if app.library_state.selected() == Some(2) && likedsongs.liked_song_display {
            likedsongs.liked_songs_state.select(Some(0));
            likedsongs.liked_songs_selected = !likedsongs.liked_songs_selected;
        } else if app.library_state.selected() == Some(3) && useralbum.user_album_display {
            useralbum.user_album_state.select(Some(0));
            useralbum.user_album_selected = !useralbum.user_album_selected;
        } else if app.library_state.selected() == Some(1) && app.recently_played_display {
            app.recently_played_state.select(Some(0));
            app.recently_played_selected = !app.recently_played_selected;
        } else if app.library_state.selected() == Some(5) && podcast.podcast_display {
            podcast.podcast_state.select(Some(0));
            podcast.podcast_selected = !podcast.podcast_selected;
        } else if app.library_state.selected() == Some(4) && app.user_artist_display {
            app.user_artist_state.select(Some(0));
            app.user_artist_selected = !app.user_artist_selected;
        }
    }
}
