use serde::{Deserialize, Serialize};
use crate::utils;
use std::collections::HashMap;

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
    pub fn avg_dist(&self, data:&Vec<Record>) -> f32{
        let mut count = 0.0;
        for i in data{
            if i != self{
                count += utils::euclidean_d(self, &i);
            }
        }
        count / (data.len() as f32 -1.0) 
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
