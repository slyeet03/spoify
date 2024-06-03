use crate::app::App;
use crate::spotify::auth::get_spotify_client;
use futures::future::join_all;
use futures::stream::StreamExt; // Add this import
use futures::FutureExt;
use futures_util::TryStreamExt;
use rayon::prelude::*;
use rspotify::model::TimeRange;
use rspotify::prelude::OAuthClient;
use rspotify::{model::FullTrack, ClientError};

/// Fetches a user's top tracks from Spotify for different time ranges and processes the data
#[tokio::main]
pub async fn top_tracks(app: &mut App) -> Result<(), ClientError> {
    // Get a Spotify client using an existing access token (if available).
    let spotify = get_spotify_client(app).await?;

    // Define the time ranges
    let time_ranges = [
        (TimeRange::LongTerm, &mut app.top_tracks_all_time_names),
        (TimeRange::MediumTerm, &mut app.top_tracks_6_months_names),
        (TimeRange::ShortTerm, &mut app.top_tracks_4_weeks_names),
    ];

    // Collect and process the user's top tracks for each time range concurrently
    let futures = time_ranges
        .iter()
        .map(|(time_range, _)| {
            let spotify = spotify.clone();
            async move {
                let mut top_tracks = Vec::new();
                let stream = spotify
                    .current_user_top_tracks(Some(*time_range))
                    .take(30) // Limit the number of top tracks to 20
                    .try_for_each(|item| {
                        top_tracks.push(item);
                        futures::future::ok(())
                    })
                    .boxed();
                stream.await?;
                Ok(top_tracks)
            }
        })
        .collect::<Vec<_>>();

    let results: Vec<Result<Vec<FullTrack>, ClientError>> = join_all(futures).await;

    // Process the results
    for (i, result) in results.into_iter().enumerate() {
        if let Ok(top_tracks) = result {
            let names = match i {
                0 => &mut app.top_tracks_all_time_names,
                1 => &mut app.top_tracks_6_months_names,
                2 => &mut app.top_tracks_4_weeks_names,
                _ => panic!("Invalid index"),
            };
            let track_names: Vec<String> = top_tracks
                .par_iter()
                .map(|track| track.name.to_string())
                .collect();
            names.extend(track_names);
        }
    }

    Ok(())
}
