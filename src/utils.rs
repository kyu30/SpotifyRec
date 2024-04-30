use crate::record::Record;
use std::fs::File;
use csv::Reader;
use serde::Deserialize;

pub fn read(path: &str) -> Vec<Record> {
    let file = File::open(path).expect("File not found");
    let mut reader = Reader::from_reader(file);
    reader.deserialize().map(|result| result.unwrap()).collect()
}

pub fn euclidean_d(track1: &Record, track2: &Record) -> f32 {
    let dance_diff = (track1.danceability - track2.danceability).abs();
    let energy_diff = (track1.energy - track2.energy).abs();
    let popularity_diff = (track1.popularity - track2.popularity).abs();
    let valence_diff = (track1.valence - track2.valence).abs();
    let tempo_diff = (track1.tempo - track2.tempo).abs();
    (dance_diff.powi(2) + energy_diff.powi(2) + popularity_diff.powi(2) + valence_diff.powi(2) + tempo_diff.powi(2)).sqrt()
}

pub fn find_similar<'a>(data: &'a Vec<Record>, target: &'a Record, top_n: usize) -> Vec<&'a Record>{
    let mut distances: Vec<(&Record, f32)> = data.iter().filter(|track| track.track_genre.to_lowercase() == target.track_genre.to_lowercase() && track.track_id != target.track_id).map(|track| (track, euclidean_d(track, target))).collect();
    distances.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());
    distances.iter().take(top_n).map(|d| d.0).collect()
} 

pub fn recommend(data: &Vec<Record>, target: &str, artist: &str, top_n: usize){
    let target_track = search(data, target, Some(artist));
    if let Some(track) = target_track{
        let similar = find_similar(data, &track, top_n);
        println!("Recommendations for '{} by {}:", target, artist);
        for similar_track in similar{
            println!("- '{}' by {}", similar_track.track_name, similar_track.artists);
        }
    }
    else{
        println!("Track '{}' by '{}' not found", target, artist);
    }
}

pub fn search<'a>(data:&'a Vec<Record>, track_name: &'a str, artist: Option<&'a str>) -> Option<&'a Record>{
    let name = track_name.to_lowercase();
    data.iter().find(|track| {
        let name_match = track.track_name.to_lowercase() == name;
        let artist_match = if let Some(artist) = artist{
            track.artists.to_lowercase().contains(&artist.to_lowercase())
        } else {
            true
        };
        name_match && artist_match
    })
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
