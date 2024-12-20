use binary_heap_plus::BinaryHeap;
use std::{cmp::Ordering, collections::HashMap, hash::Hash};

use super::playback_record::PlaybackRecord;

/// Groups a vector of `PlaybackRecord` objects by a key-generating function.
///
/// # Arguments
///
/// * `data` - A reference to the vector of `PlaybackRecord` objects to group.
/// * `key_fn` - A closure that generates a key of type `K` for each record.
///              The key determines how the records are grouped.
///
/// # Returns
///
/// A `HashMap<K, Vec<PlaybackRecord>>` where:
///
/// * Each key is generated by applying `key_fn` to the records.
/// * Each value is a vector containing all records that correspond to that key.
///
/// # Examples (simplified PlaybackRecord struct for clarity)
///
/// ```
/// let playback_records = vec![
///     PlaybackRecord { id: 1, category: "Music".to_string() },
///     PlaybackRecord { id: 2, category: "Podcast".to_string() },
///     PlaybackRecord { id: 3, category: "Music".to_string() },
/// ];
/// let grouped = group_by(&playback_records, |record| record.category.clone());
///
/// // grouped will be:
/// {
///    "Music": vec![PlaybackRecord { id: 1 }, PlaybackRecord { id: 3 }],
///    "Podcast": vec![PlaybackRecord { id: 2 }],
/// }
/// ```

pub(crate) fn group_by<K, F>(
    data: &Vec<PlaybackRecord>,
    key_fn: F,
) -> HashMap<K, Vec<PlaybackRecord>>
where
    K: Eq + Hash,
    F: Fn(&PlaybackRecord) -> K,
{
    data.iter().fold(HashMap::new(), |mut map, record| {
        map.entry(key_fn(record))
            .or_insert_with(|| Vec::new())
            .push(record.clone());
        map
    })
}

/// Filters a vector of `PlaybackRecord` objects based on a predicate function.
///
/// # Arguments
///
/// * `data` - A reference to the vector of `PlaybackRecord` objects to filter.
/// * `filter_fn` - A closure that returns `true` for records to include and `false` for those to exclude.
///
/// # Returns
///
/// A new vector containing only the `PlaybackRecord` objects that satisfy the filter condition.
///
/// # Examples (simplified PlaybackRecord struct for clarity)
///
/// ```
/// let playback_records = vec![
///     PlaybackRecord { id: 1, duration: 30 },
///     PlaybackRecord { id: 2, duration: 120 },
/// ];
/// let filtered = filter_by(&playback_records, |record| record.duration > 60);
///
/// // filtered will be:
/// // vec![PlaybackRecord { id: 2 }]
/// ```
pub(crate) fn filter_by<F, K>(data: &Vec<K>, filter_fn: F) -> Vec<K>
where
    F: Fn(&K) -> bool,
    K: Clone,
{
    data.iter()
        .filter(|&record| filter_fn(record))
        .map(|record| record.clone())
        .collect()
}

/// Sorts a vector of `PlaybackRecord` objects using a custom sorting function.
///
/// # Arguments
///
/// * `data` - A reference to the vector of `PlaybackRecord` objects to sort.
/// * `sorting_fn` - A closure that defines the sorting order. It takes two
///                  `PlaybackRecord` references and returns an `Ordering`
///                  (`Less`, `Equal`, or `Greater`).
///
/// # Returns
///
/// A new vector containing the sorted `PlaybackRecord` objects.
///
/// # Examples (simplified PlaybackRecord struct for clarity)
///
/// ```
/// let playback_records = vec![
///     PlaybackRecord { id: 1, duration: 30 },
///     PlaybackRecord { id: 2, duration: 120 },
/// ];
/// let sorted = sort_by(&playback_records, |a, b| a.duration.cmp(&b.duration));
///
/// // sorted will be:
/// // vec![PlaybackRecord { id: 1 }, PlaybackRecord { id: 2 }]
/// ```
pub(crate) fn sort_by<F>(data: &Vec<PlaybackRecord>, sorting_fn: F) -> Vec<PlaybackRecord>
where
    F: Fn(&PlaybackRecord, &PlaybackRecord) -> Ordering,
{
    let mut data_copy = data.clone();
    data_copy.sort_by(sorting_fn);
    data_copy
}

/// Retrieves the top `n` `PlaybackRecord` objects from a vector based on a custom sorting function.
///
/// # Arguments
///
/// * `data` - A reference to the vector of `PlaybackRecord` objects.
/// * `n` - The number of top records to retrieve. If `n` is greater than the length
///         of the vector, all elements are returned.
/// * `sorting_fn` - A closure that defines the sorting order. This function is the same
///                  as the one used in [`Vec::sort_by`].
///
/// # Returns
///
/// A new vector containing the top `n` `PlaybackRecord` objects in sorted order.
///
/// # Notes
///
/// This function uses a binary heap for efficient extraction of the top `n` records.
///
/// # Examples (simplified PlaybackRecord struct for clarity)
///
///
/// ```
/// let playback_records = vec![
///     PlaybackRecord { id: 1, duration: 30 },
///     PlaybackRecord { id: 2, duration: 120 },
///     PlaybackRecord { id: 3, duration: 60 },
/// ];
/// let top_n = sort_by_top_n(&playback_records, 2, |a, b| b.duration.cmp(&a.duration));
///
/// // top_n will be:
/// // vec![PlaybackRecord { id: 2 }, PlaybackRecord { id: 3 }]
/// ```
pub(crate) fn sort_by_top_n<F, K>(data: &Vec<K>, n: usize, sorting_fn: F) -> Vec<K>
where
    F: Fn(&K, &K) -> Ordering,
    K: Clone,
{
    let mut bin_heap = BinaryHeap::from_vec_cmp(data.clone(), sorting_fn);
    (0..n.clamp(0, data.len()))
        .map(move |_| bin_heap.pop().unwrap())
        .collect()
}

#[cfg(test)]
mod test_group_by {
    use std::fmt::Debug;

    use super::*;
    use serde_json;
    const json: &str = include_str!("./test/test_data.json");
    fn get_test_data() -> Vec<PlaybackRecord> {
        serde_json::from_str(json).unwrap()
    }

    fn test_group_by<K, F>(key_fn: F)
    where
        K: Eq + Hash + Debug,
        F: Fn(&PlaybackRecord) -> K,
    {
        let data = get_test_data();
        let grouped_data = group_by(&data, &key_fn);
        for (key, records) in grouped_data.into_iter() {
            for record in records {
                assert_eq!(key_fn(&record), key)
            }
        }
    }

    #[test]
    fn test_group_by_ts() {
        test_group_by(|record| record.ts.clone());
    }

    #[test]
    fn test_group_by_platform() {
        test_group_by(|record| record.platform.clone());
    }

    #[test]
    fn test_group_by_ms_played() {
        test_group_by(|record| record.ms_played);
    }

    #[test]
    fn test_group_by_conn_country() {
        test_group_by(|record| record.conn_country.clone());
    }

    #[test]
    fn test_group_by_ip_addr() {
        test_group_by(|record| record.ip_addr.clone());
    }

    #[test]
    fn test_group_by_track_name() {
        test_group_by(|record| record.master_metadata_track_name.clone());
    }

    #[test]
    fn test_group_by_album_artist_name() {
        test_group_by(|record| record.master_metadata_album_artist_name.clone());
    }

    #[test]
    fn test_group_by_album_name() {
        test_group_by(|record| record.master_metadata_album_album_name.clone());
    }

    #[test]
    fn test_group_by_spotify_uri() {
        test_group_by(|record| record.spotify_track_uri.clone());
    }

    #[test]
    fn test_group_by_reason_start() {
        test_group_by(|record| record.reason_start.clone());
    }

    #[test]
    fn test_group_by_reason_end() {
        test_group_by(|record| record.reason_end.clone());
    }

    #[test]
    fn test_group_by_shuffle() {
        test_group_by(|record| record.shuffle);
    }

    #[test]
    fn test_group_by_skipped() {
        test_group_by(|record| record.skipped);
    }

    #[test]
    fn test_group_by_offline() {
        test_group_by(|record| record.offline);
    }

    #[test]
    fn test_group_by_offline_timestamp() {
        test_group_by(|record| record.offline_timestamp);
    }

    #[test]
    fn test_group_by_incognito_mode() {
        test_group_by(|record| record.incognito_mode);
    }
}
#[cfg(test)]
mod test_filter {
    use std::fmt::Debug;

    use super::*;
    use serde_json;
    const json: &str = include_str!("./test/test_data.json");
    fn get_test_data() -> Vec<PlaybackRecord> {
        serde_json::from_str(json).unwrap()
    }

    fn test_filter_by<F>(key_fn: F)
    where
        F: Fn(&PlaybackRecord) -> bool,
    {
        let data = get_test_data();
        let filtered_data = filter_by(&data, &key_fn);
        for record in data {
            if filtered_data.contains(&record) {
                assert!(&key_fn(&record))
            } else {
                assert!(!&key_fn(&record))
            }
        }
    }
    #[test]
    fn test_filter_by_author() {
        test_filter_by(|record| record.master_metadata_album_artist_name == "Heidi Barabisch");
    }

    #[test]
    fn test_filter_by_min_playtime() {
        test_filter_by(|record| record.ms_played > 10_000);
    }

    #[test]
    fn test_filter_by_country() {
        test_filter_by(|record| record.conn_country == "NL");
    }

    #[test]
    fn test_filter_by_offline() {
        test_filter_by(|record| record.offline);
    }

    #[test]
    fn test_filter_by_skipped() {
        test_filter_by(|record| record.skipped);
    }

    #[test]
    fn test_filter_by_incognito_mode() {
        test_filter_by(|record| record.incognito_mode);
    }

    #[test]
    fn test_filter_by_shuffle() {
        test_filter_by(|record| record.shuffle);
    }

    #[test]
    fn test_filter_by_reason_end_trackdone() {
        test_filter_by(|record| record.reason_end == "trackdone");
    }

    #[test]
    fn test_filter_by_platform_windows() {
        test_filter_by(|record| record.platform == "windows");
    }

    #[test]
    fn test_filter_by_platform_linux() {
        test_filter_by(|record| record.platform == "android");
    }
}
