use log::info;

use crate::spotify::query_find::album_storage;
use crate::spotify::query_find::artist_storage;
use crate::spotify::query_find::playlist_storage;
use crate::spotify::query_find::track_storage;
use crate::spotify::search::search;
use std::io;
use std::path::PathBuf;

pub fn query_storage(
    query: &str,
) -> Result<
    (
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
    ),
    io::Error,
> {
    let mut spotify_cache_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    spotify_cache_path.push("..");
    spotify_cache_path.push("spoify-tui");
    spotify_cache_path.push("spotify_cache");

    if search(query).is_ok() {
        let (album_names, album_links) = match album_storage(&spotify_cache_path) {
            Ok(result) => result,
            Err(err) => {
                info!("Error reading album data: {}", err);
                return Err(err);
            }
        };

        let (track_names, track_links) = match track_storage(&spotify_cache_path) {
            Ok(result) => result,
            Err(err) => {
                info!("Error reading track data: {}", err);
                return Err(err);
            }
        };

        let (artist_names, artist_links) = match artist_storage(&spotify_cache_path) {
            Ok(result) => result,
            Err(err) => {
                info!("Error reading artist data: {}", err);
                return Err(err);
            }
        };

        let (playlist_names, playlist_links) = match playlist_storage(&spotify_cache_path) {
            Ok(result) => result,
            Err(err) => {
                info!("Error reading playlist data: {}", err);
                return Err(err);
            }
        };

        return Ok((
            album_names,
            album_links,
            track_names,
            track_links,
            playlist_names,
            playlist_links,
            artist_names,
            artist_links,
        ));
    }

    Err(io::Error::new(io::ErrorKind::Other, "Search failed"))
}
