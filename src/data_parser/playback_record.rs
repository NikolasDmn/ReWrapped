use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use gloo::{console::log, utils::format};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

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
    pub fn from_json(json: &str) -> Result<Vec<Self>, serde_json::Error> {
        serde_json::from_str(json)
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
    pub fn from_json(json: &str) -> Result<Vec<PlaybackRecord>, serde_json::Error> {
        let unsanitized = UnsanitizedPlaybackRecord::from_json(json);
        if let Err(e) = unsanitized {
            return Err(e);
        }
        Ok(unsanitized
            .unwrap()
            .into_iter()
            .map(|ur| PlaybackRecord::from(ur))
            .collect())
    }
    pub fn from_jsons(jsons: &Vec<String>) -> Result<Vec<PlaybackRecord>, serde_json::Error> {
        let data_files: Result<Vec<Vec<PlaybackRecord>>, serde_json::Error> = jsons
            .iter()
            .map(|json| {
                let x = PlaybackRecord::from_json(json);
                log!(format!("{:?}", x));
                x
            })
            .collect();
        if let Err(e) = data_files {
            log!(format!("Error: {:?}", e).as_str());
            return Err(e);
        }
        Ok(data_files.unwrap().into_iter().flatten().collect())
    }
}
