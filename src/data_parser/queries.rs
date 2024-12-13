use std::{hash::Hash, i64, u64};

use super::{
    playback_record::PlaybackRecord,
    processing::{filter_by, group_by},
};
pub fn get_total_listening_time_in_ms(data: &Vec<PlaybackRecord>) -> u64 {
    data.iter().map(|record| record.ms_played as u64).sum()
}
fn get_top_based_on_grouping<K, F>(data: &Vec<PlaybackRecord>, group_fn: F) -> Vec<(K, u64)>
where
    K: Eq + Hash + Clone,
    F: Fn(&PlaybackRecord) -> K,
{
    let mut grouped_data: Vec<(K, u64)> = group_by(data, group_fn)
        .iter()
        .map(|(key, records)| (key.clone(), get_total_listening_time_in_ms(records)))
        .collect();
    grouped_data.sort_by(|(_, d1), (_, d2)| d2.cmp(d1));
    grouped_data
}
pub fn get_top_artists(data: &Vec<PlaybackRecord>) -> Vec<(String, u64)> {
    get_top_based_on_grouping(data, |record| {
        record.master_metadata_album_artist_name.clone()
    })
}
pub fn get_top_artists_percentages(data: &Vec<PlaybackRecord>, n: usize) -> Vec<(String, f32)> {
    let total_time = get_total_listening_time_in_ms(data);

    let top_artists = get_top_artists(data);
    let total_t = total_time as f64;
    top_artists
        .into_iter()
        .map(|(name, time)| (name, (time as f64 / total_t) as f32))
        .map(|(n, r)| (n, (r * 10000.0).round() / 100.0))
        .filter(|(_, n)| n >= &4.0)
        .collect()
}

pub fn get_top_songs(data: &Vec<PlaybackRecord>) -> Vec<(String, u64)> {
    get_top_based_on_grouping(data, |record| record.master_metadata_track_name.clone())
}

pub fn get_top_albums(data: &Vec<PlaybackRecord>) -> Vec<(String, u64)> {
    get_top_based_on_grouping(data, |record| {
        record.master_metadata_album_album_name.clone()
    })
}
pub fn get_top_countries(data: &Vec<PlaybackRecord>) -> Vec<(String, u64)> {
    get_top_based_on_grouping(data, |record| record.conn_country.clone())
}

pub fn get_top_platforms(data: &Vec<PlaybackRecord>) -> Vec<(String, u64)> {
    get_top_based_on_grouping(data, |record| record.platform.clone())
}
/// Returns playback duration of the `true` and `false` values of any boolean field in the `[PlaybackRecord]`
/// struct.
///
/// # Arguments
///
/// * `data` - A reference to the vector of `PlaybackRecord` objects to group.
/// * `group_fn` - A closure that generates a key of type `bool` for each record.
///              The key determines which boolean field to differentiate.
///
/// # Returns
///
/// A `(a: u64, b: u64)` where a is the duration when field is `true` and b is the duration when
/// field is `false` (in ms)
pub fn playback_duration_difference_between_boolean_fields_in_ms<F>(
    data: &Vec<PlaybackRecord>,
    group_fn: F,
) -> (u64, u64)
where
    F: Fn(&PlaybackRecord) -> bool,
{
    let mut grouped_data = group_by(data, group_fn)
        .into_iter()
        .map(|(key, records)| get_total_listening_time_in_ms(&records));
    (
        grouped_data.next().unwrap_or(0),
        grouped_data.next().unwrap_or(0),
    )
}

pub fn playback_duration_difference_between_shuffle_or_not(
    data: &Vec<PlaybackRecord>,
) -> (u64, u64) {
    playback_duration_difference_between_boolean_fields_in_ms(data, |record| record.shuffle)
}

pub fn playback_duration_difference_between_icognito_or_not(
    data: &Vec<PlaybackRecord>,
) -> (u64, u64) {
    playback_duration_difference_between_boolean_fields_in_ms(data, |record| record.incognito_mode)
}
pub fn playback_duration_difference_between_offline_or_not(
    data: &Vec<PlaybackRecord>,
) -> (u64, u64) {
    playback_duration_difference_between_boolean_fields_in_ms(data, |record| record.offline)
}

/// Returns the most skipped songs that are at least half the average playback of a track
pub fn get_most_skipped_songs(data: &Vec<PlaybackRecord>) -> Vec<(String, i64)> {
    let dropoff = get_total_listening_time_in_ms(data) / data.len() as u64 / 2;
    let mut skipped_rates: Vec<(String, i64)> =
        group_by(data, |record| record.master_metadata_track_name.clone())
            .into_iter()
            .filter(|(_, records)| get_total_listening_time_in_ms(records) > dropoff)
            .map(|(song, records)| {
                (
                    song,
                    playback_duration_difference_between_boolean_fields_in_ms(&records, |r| {
                        r.skipped
                    }),
                )
            })
            .map(|(song, (skipped, not_skipped))| {
                (song, skipped as i64 / (skipped + not_skipped) as i64)
            })
            .collect();
    skipped_rates.sort_by(|(_, r1), (_, r2)| r2.cmp(r1));
    skipped_rates
}
/// Returns the most skipped artists that are at least half the average playback of a track
pub fn get_most_skipped_artists(data: &Vec<PlaybackRecord>) -> Vec<(String, i64)> {
    let dropoff = get_total_listening_time_in_ms(data) / data.len() as u64 / 2;
    let mut skipped_rates: Vec<(String, i64)> = group_by(data, |record| {
        record.master_metadata_album_artist_name.clone()
    })
    .into_iter()
    .filter(|(_, records)| get_total_listening_time_in_ms(records) > dropoff)
    .map(|(song, records)| {
        (
            song,
            playback_duration_difference_between_boolean_fields_in_ms(&records, |r| r.skipped),
        )
    })
    .map(|(song, (skipped, not_skipped))| (song, skipped as i64 / (skipped + not_skipped) as i64))
    .collect();
    skipped_rates.sort_by(|(_, r1), (_, r2)| r2.cmp(r1));
    skipped_rates
}
/// Returns the most skipped albums that are at least half the average playback of a track
pub fn get_most_skipped_albums(data: &Vec<PlaybackRecord>) -> Vec<(String, i64)> {
    let dropoff = get_total_listening_time_in_ms(data) / data.len() as u64 / 2;
    let mut skipped_rates: Vec<(String, i64)> = group_by(data, |record| {
        record.master_metadata_album_album_name.clone()
    })
    .into_iter()
    .filter(|(_, records)| get_total_listening_time_in_ms(records) > dropoff)
    .map(|(song, records)| {
        (
            song,
            playback_duration_difference_between_boolean_fields_in_ms(&records, |r| r.skipped),
        )
    })
    .map(|(song, (skipped, not_skipped))| (song, skipped as i64 / (skipped + not_skipped) as i64))
    .collect();
    skipped_rates.sort_by(|(_, r1), (_, r2)| r2.cmp(r1));
    skipped_rates
}