use std::cell::UnsafeCell;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use gloo::{console::log, utils::format};
use rayon::prelude::*;
use serde::{ser, Deserialize, Serialize};
use serde_json::{from_slice, from_value, Deserializer, Value};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PlaybackRecord {
    pub ts: DateTime<Utc>,
    pub platform: String,
    pub ms_played: u32,
    pub conn_country: String,
    pub ip_addr: String,
    pub master_metadata_track_name: String,
    pub master_metadata_album_artist_name: String,
    pub master_metadata_album_album_name: String,
    pub spotify_track_uri: String,
    pub reason_start: String,
    pub reason_end: String,
    pub shuffle: bool,
    pub skipped: bool,
    pub offline: bool,
    pub offline_timestamp: Option<u64>,
    pub incognito_mode: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
struct UnsanitizedPlaybackRecord {
    pub ts: DateTime<Utc>,
    pub platform: String,
    pub ms_played: u32,
    pub conn_country: String,
    pub ip_addr: String,
    pub master_metadata_track_name: Option<String>,
    pub master_metadata_album_artist_name: Option<String>,
    pub master_metadata_album_album_name: Option<String>,
    pub spotify_track_uri: Option<String>,
    pub episode_name: Option<String>,
    pub episode_show_name: Option<String>,
    pub spotify_episode_uri: Option<String>,
    pub reason_start: String,
    pub reason_end: String,
    pub shuffle: bool,
    pub skipped: bool,
    pub offline: bool,
    pub offline_timestamp: Option<u64>,
    pub incognito_mode: bool,
}

impl UnsanitizedPlaybackRecord {
    pub fn from_json(json: &[u8]) -> Result<Vec<Self>, serde_json::Error> {
        let json_values: Vec<serde_json::Value> = from_slice(json)?;

        json_values
            .into_par_iter()
            .map(|value| from_value(value))
            .collect()
    }
}

impl From<UnsanitizedPlaybackRecord> for PlaybackRecord {
    fn from(raw: UnsanitizedPlaybackRecord) -> Self {
        PlaybackRecord {
            ts: raw.ts,
            platform: raw.platform,
            ms_played: raw.ms_played,
            conn_country: raw.conn_country,
            ip_addr: raw.ip_addr,
            master_metadata_track_name: raw
                .master_metadata_track_name
                .or_else(|| raw.episode_name.clone())
                .unwrap_or_else(|| "".to_string()),
            master_metadata_album_artist_name: raw
                .master_metadata_album_artist_name
                .or_else(|| raw.episode_show_name.clone())
                .unwrap_or_else(|| "".to_string()),
            master_metadata_album_album_name: raw
                .master_metadata_album_album_name
                .unwrap_or_else(|| "".to_string()),
            spotify_track_uri: raw
                .spotify_track_uri
                .or_else(|| raw.spotify_episode_uri)
                .unwrap_or_else(|| "".to_string()),
            reason_start: raw.reason_start,
            reason_end: raw.reason_end,
            shuffle: raw.shuffle,
            skipped: raw.skipped,
            offline: raw.offline,
            offline_timestamp: raw.offline_timestamp,
            incognito_mode: raw.incognito_mode,
        }
    }
}
impl PlaybackRecord {
    pub fn from_json(json: &[u8]) -> Result<Vec<PlaybackRecord>, serde_json::Error> {
        let unsanitized = UnsanitizedPlaybackRecord::from_json(json);
        if let Err(e) = unsanitized {
            return Err(e);
        }
        Ok(unsanitized
            .unwrap_or(vec![])
            .into_par_iter()
            .map(|ur| PlaybackRecord::from(ur))
            .collect())
    }
    pub fn from_jsons(jsons: &[Vec<u8>]) -> Result<Vec<PlaybackRecord>, serde_json::Error> {
        let data_files: Result<Vec<Vec<PlaybackRecord>>, serde_json::Error> = jsons
            .into_par_iter()
            .map(|json| PlaybackRecord::from_json(json))
            .collect();
        if let Err(e) = data_files {
            return Err(e);
        }
        Ok(data_files.unwrap_or(vec![]).into_iter().flatten().collect())
    }
}
