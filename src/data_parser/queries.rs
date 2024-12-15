use std::{hash::Hash, i64, str::FromStr, u64};

use chrono::prelude::*;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;

use super::{
    playback_record::PlaybackRecord,
    processing::{filter_by, group_by, sort_by_top_n},
};
pub fn get_total_listening_time_in_ms(data: &Vec<PlaybackRecord>) -> u64 {
    data.iter().map(|record| record.ms_played as u64).sum()
}
fn get_top_based_on_grouping<K, F>(data: &Vec<PlaybackRecord>, group_fn: F) -> Vec<(K, u64)>
where
    K: Eq + Hash + Clone + Default,
    F: Fn(&PlaybackRecord) -> K,
{
    let mut grouped_data: Vec<(K, u64)> = group_by(data, group_fn)
        .iter()
        .map(|(key, records)| (key.clone(), get_total_listening_time_in_ms(records)))
        .filter(|(k, _)| k != &Default::default())
        .collect();
    grouped_data.sort_by(|(_, d1), (_, d2)| d2.cmp(d1));
    grouped_data
}
pub fn get_top_artists(data: &Vec<PlaybackRecord>) -> Vec<(String, u64)> {
    get_top_based_on_grouping(data, |record| {
        record.master_metadata_album_artist_name.clone()
    })
}
pub fn get_top_percentages<T>(
    data: &Vec<PlaybackRecord>,
    cutoff: f32,
    minimum_elements: usize,
    grouping_method: T,
) -> Vec<(String, f32)>
where
    T: Fn(&Vec<PlaybackRecord>) -> Vec<(String, u64)>,
{
    let total_time = get_total_listening_time_in_ms(data);

    let top_artists = grouping_method(data);
    let total_t = total_time as f64;
    let artists: Vec<(String, f32)> = top_artists
        .into_iter()
        .map(|(name, time)| (name, (time as f64 / total_t) as f32))
        .map(|(n, r)| (n, (r * 10000.0).round() / 100.0))
        .collect();
    let mut ret = artists[..minimum_elements.clamp(0, artists.len())].to_vec();
    let more = artists[minimum_elements.clamp(0, artists.len() - 1)..]
        .to_vec()
        .into_iter()
        .filter(|(_, n)| n >= &cutoff)
        .collect::<Vec<(String, f32)>>();
    ret.extend(more);
    ret
}
pub fn get_top_artists_percentages(
    data: &Vec<PlaybackRecord>,
    cutoff: f32,
    minimum_elements: usize,
) -> Vec<(String, f32)> {
    get_top_percentages(data, cutoff, minimum_elements, get_top_artists)
}
pub fn get_top_songs_percentages(
    data: &Vec<PlaybackRecord>,
    cutoff: f32,
    minimum_elements: usize,
) -> Vec<(String, f32)> {
    get_top_percentages(data, cutoff, minimum_elements, get_top_songs)
}
pub fn get_top_albums_percentages(
    data: &Vec<PlaybackRecord>,
    cutoff: f32,
    minimum_elements: usize,
) -> Vec<(String, f32)> {
    get_top_percentages(data, cutoff, minimum_elements, get_top_albums)
}

pub fn get_top_songs(data: &Vec<PlaybackRecord>) -> Vec<(String, u64)> {
    get_top_based_on_grouping(data, |record| record.master_metadata_track_name.clone())
}

pub fn get_top_albums(data: &Vec<PlaybackRecord>) -> Vec<(String, u64)> {
    get_top_based_on_grouping(data, |record| {
        record.master_metadata_album_album_name.clone()
    })
}
pub fn get_top_countries(data: &Vec<PlaybackRecord>) -> Vec<(String, f32)> {
    get_top_based_on_grouping(data, |record| record.conn_country.clone())
        .into_par_iter()
        .map(|(s, u)| (s, (u as f32 / 60000.0).round()))
        .collect()
}

pub fn get_top_platforms(data: &Vec<PlaybackRecord>) -> Vec<(String, f32)> {
    get_top_based_on_grouping(data, |record| record.platform.clone())
        .into_par_iter()
        .map(|(s, u)| (s, (u as f32 / 60000.0).round()))
        .collect()
}
pub fn get_hours_of_the_day_distribution(data: &Vec<PlaybackRecord>) -> Vec<(String, f32)> {
    let total = get_total_listening_time_in_ms(data) as f32 / 60000.0;
    get_top_based_on_grouping(data, |record| format!("{}", record.ts.time().hour() + 1))
        .into_iter()
        .map(|(h, ms)| (h, ms as f32 / 60000.0))
        .map(|(h, m)| (h, m / total))
        .map(|(h, p)| (h, (p * 100.0).round()))
        .collect()
}
pub fn get_day_distribution(data: &Vec<PlaybackRecord>) -> Vec<(String, f32)> {
    let mut res: Vec<(chrono::Weekday, f32)> = group_by(data, |record| record.ts.weekday())
        .into_par_iter()
        .map(|(weekday, records)| {
            (
                weekday,
                (get_total_listening_time_in_ms(&records) as f64 / 60000.0),
            )
        })
        .map(|(weekday, r)| (weekday, r.round() as f32))
        .collect();
    res.sort_by_key(|(weekday, _)| weekday.number_from_monday());
    res.into_iter()
        .map(|(weekday, r)| (weekday.to_string(), r))
        .collect()
}
pub fn get_top_days(data: &Vec<PlaybackRecord>, n: usize) -> Vec<(String, f32)> {
    sort_by_top_n(
        &group_by(data, |record| record.ts.format("%d/%m/%y").to_string())
            .into_par_iter()
            .map(|(day, records)| (day, get_total_listening_time_in_ms(&records)))
            .collect(),
        n,
        |(_, r), (_, r2)| r.partial_cmp(r2).unwrap_or(std::cmp::Ordering::Equal),
    )
    .into_iter()
    .map(|(s, r)| (s, r as f64 / 60000.0))
    .map(|(s, r)| (s, r.round() as f32))
    .map(|(s, r)| (s, ((r / 1440.0) * 100.0).round()))
    .collect()
}
pub fn get_months_distribution(data: &Vec<PlaybackRecord>) -> Vec<(String, f32)> {
    let mut ret: Vec<(String, f32)> = group_by(data, |record| record.ts.format("%B").to_string())
        .into_iter()
        .map(|(month, records)| {
            (
                month,
                (get_total_listening_time_in_ms(&records) as f64 / 60000.0),
            )
        })
        .map(|(weekday, r)| (weekday, r.round() as f32))
        .collect();
    ret.sort_by_key(|(month, _)| Month::from_str(month).unwrap().number_from_month());
    ret
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
