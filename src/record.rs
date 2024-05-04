use crate::utils;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Record {
    pub track_id: String,
    pub artists: String,
    pub album_name: String,
    pub track_name: String,
    pub popularity: u32,
    pub explicit: String,
    pub danceability: f32,
    pub energy: f32,
    pub key: u32,
    pub loudness: f32,
    pub mode: u32,
    pub valence: f32,
    pub tempo: f32,
    pub track_genre: String,
}

impl Record {
    pub fn clean(&mut self) {
        self.track_id = self.track_id.trim().to_string();
        self.artists = self.artists.trim().to_string();
        self.album_name = self.album_name.trim().to_string();
        self.track_name = self.track_name.trim().to_string();
        self.explicit = self.explicit.trim().to_lowercase();
    }
}
