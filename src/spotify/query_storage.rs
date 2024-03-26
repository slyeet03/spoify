use crate::spotify::album_storage::album_storage;
use crate::spotify::artist_storage::artist_storage;
use crate::spotify::playlist_storage::playlist_storage;
use crate::spotify::track_storage::track_storage;

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
        ));
    }
    Err(io::Error::new(io::ErrorKind::Other, "Search failed"))
}
