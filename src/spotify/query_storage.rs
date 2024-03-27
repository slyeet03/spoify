use crate::spotify::query_find::album_storage;
use crate::spotify::query_find::artist_storage;
use crate::spotify::query_find::playlist_storage;
use crate::spotify::query_find::track_storage;

use crate::spotify::search::search;

use std::io;

pub fn query_storage(
    query: &str,
    data_dir: &std::path::Path,
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
    let search_result = search(query, data_dir);
    if search_result.is_ok() {
        let (album_names, album_links) = match album_storage() {
            Ok(result) => result,
            Err(err) => {
                println!("Error reading album data: {}", err);
                return Err(err);
            }
        };
        let (track_names, track_links) = match track_storage() {
            Ok(result) => result,
            Err(err) => {
                println!("Error reading track data: {}", err);
                return Err(err);
            }
        };
        let (artist_names, artist_links) = match artist_storage() {
            Ok(result) => result,
            Err(err) => {
                println!("Error reading artist data: {}", err);
                return Err(err);
            }
        };
        let (playlist_names, playlist_links) = match playlist_storage() {
            Ok(result) => result,
            Err(err) => {
                println!("Error reading playlist data: {}", err);
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
