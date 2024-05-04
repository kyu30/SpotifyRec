use crate::record::Record;
use csv::ReaderBuilder;
use csv::WriterBuilder;
use csv::Reader;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::prelude::*;
use rand::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn read(path: &str) -> Vec<Record> {
    let file = File::open(path).expect("File not found");
    let mut reader = Reader::from_reader(file);
    reader.deserialize().map(|result| result.unwrap()).collect()
}

pub fn random_sample(
    input_path: &str,
    output_path: &str,
    sample_size: usize,
) -> Result<(), Box<dyn Error>> {
    let file = File::open(input_path)?;
    let mut reader = ReaderBuilder::new().from_reader(file);
    let mut records: Vec<Record> = reader.deserialize().filter_map(Result::ok).collect();
    let mut rng = thread_rng();
    records.shuffle(&mut rng);
    records.truncate(sample_size);
    let file_out = File::create(output_path)?;
    let mut writer = WriterBuilder::new().from_writer(file_out);

    for record in records {
        writer.serialize(record)?;
    }

    Ok(())
}

pub fn knn_graph(records: &[Record], k: usize) -> Graph<Record, f32> {
    let mut graph = Graph::new();
    let mut node_indices = Vec::new();
    for record in records {
        let node = graph.add_node(record.clone());
        node_indices.push(node);
        println!("Node {} created", record.track_name);
    }
    for (i, &idx) in node_indices.iter().enumerate() {
        let mut distances = Vec::new();
        for (j, &other_idx) in node_indices.iter().enumerate() {
            if i != j {
                println!(
                    "Checking edge {} - {} by {}",
                    &records[i].track_name, &records[j].track_name, &records[j].artists
                );
                let dist = sim_calc(&records[i], &records[j]);
                distances.push((dist, other_idx));
            }
        }
        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        for &(d, near_idx) in distances.iter().take(k) {
            let weight = d;
            graph.add_edge(idx, near_idx, weight);
        }
    }
    graph
}

pub fn calc_centrality(graph: &Graph<Record, f32>) -> HashMap<NodeIndex, usize> {
    graph
        .node_indices()
        .map(|n| (n, graph.edges(n).count()))
        .collect()
}

pub fn index(records: &[Record]) -> HashMap<String, Record> {
    let mut map = HashMap::new();
    for record in records {
        map.insert(record.track_id.clone(), record.clone());
    }
    map
}

pub fn top(
    records: &[Record],
    graph: &Graph<Record, f32>,
    song_name: &str,
    centrality: &HashMap<NodeIndex, usize>,
) {
    let mut results = Vec::new();
    let map = index(records);
    if let Some((node_index, _)) = graph.node_indices().find_map(|n| {
        if &graph[n].track_id.trim() == &song_name {
            Some((n, &graph[n]))
        } else {
            None
        }
    }) {
        println!("Song found, getting neighbors...");
        let mut neighbors: Vec<(usize, &String)> = graph
            .neighbors(node_index)
            .map(|n| (centrality[&n], &graph[n].track_id))
            .collect();
        neighbors.sort_by(|a, b| b.0.cmp(&a.0));
        results = neighbors
            .iter()
            .take(5)
            .map(|(_, name)| name.clone())
            .collect();
        println!(
            "Found {} neighbors for {}.",
            neighbors.len(),
            map.get(song_name).unwrap().track_name
        );
        println!("Top 5 recommendations");
        for song in &results {
            println!(
                "- {} by {}",
                map.get(song.as_str()).unwrap().track_name,
                map.get(song.as_str()).unwrap().artists
            );
        }
    }
}

pub fn euclidean_d(track1: &Record, track2: &Record) -> f32 {
    let dance_diff = (track1.danceability - track2.danceability).abs();
    let energy_diff = (track1.energy - track2.energy).abs();
    let popularity_diff = (track1.popularity as f32 - track2.popularity as f32).abs() / 100.0;
    let valence_diff = (track1.valence - track2.valence).abs();
    let tempo_diff = (track1.tempo - track2.tempo).abs();
    (dance_diff.powi(2)
        + energy_diff.powi(2)
        + popularity_diff.powi(2)
        + valence_diff.powi(2)
        + tempo_diff.powi(2))
    .sqrt()
}

pub fn find_similar<'a>(
    data: &'a Vec<Record>,
    target: &'a Record,
    top_n: usize,
) -> Vec<&'a Record> {
    let mut distances: Vec<(&Record, f32)> = data
        .iter()
        .filter(|track| {
            track.track_genre.to_lowercase() == target.track_genre.to_lowercase()
                && track.track_id != target.track_id
        })
        .map(|track| (track, euclidean_d(track, target)))
        .collect();
    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    distances.iter().take(top_n).map(|d| d.0).collect()
}

pub fn sim_calc(track1: &Record, track2: &Record) -> f32 {
    let distance = euclidean_d(track1, track2);
    1.0 - (distance / f32::sqrt(5.0)).min(1.0)
}

pub fn search<'a>(
    data: &'a Vec<Record>,
    track_name: &'a str,
    artist: Option<&'a str>,
) -> Option<&'a Record> {
    let name = track_name.to_lowercase();
    data.iter().find(|track| {
        let name_match = track.track_name.to_lowercase() == name;
        let artist_match = if let Some(artist) = artist {
            track
                .artists
                .to_lowercase()
                .contains(&artist.to_lowercase())
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
    fn test_knn_graph() {
        let record1: Record = Record {
            track_id: "001".to_string(),
            artists: "Artist One".to_string(),
            album_name: "First Album".to_string(),
            track_name: "Hit Single".to_string(),
            popularity: 85,
            explicit: "No".to_string(),
            danceability: 0.7,
            energy: 0.8,
            key: 5,
            loudness: -5.4,
            mode: 1,
            valence: 0.6,
            tempo: 120.0,
            track_genre: "Pop".to_string(),
        };

        let record2: Record = Record {
            track_id: "002".to_string(),
            artists: "Band Two".to_string(),
            album_name: "Second Wave".to_string(),
            track_name: "Smooth Ride".to_string(),
            popularity: 75,
            explicit: "Yes".to_string(),
            danceability: 0.6,
            energy: 0.65,
            key: 8,
            loudness: -6.2,
            mode: 0,
            valence: 0.4,
            tempo: 100.0,
            track_genre: "Jazz".to_string(),
        };

        let record3: Record = Record {
            track_id: "003".to_string(),
            artists: "DJ Example".to_string(),
            album_name: "Beats to Study To".to_string(),
            track_name: "Lo-fi Vibes".to_string(),
            popularity: 82,
            explicit: "No".to_string(),
            danceability: 0.8,
            energy: 0.5,
            key: 2,
            loudness: -9.8,
            mode: 1,
            valence: 0.8,
            tempo: 85.0,
            track_genre: "Lo-fi".to_string(),
        };

        let record4: Record = Record {
            track_id: "004".to_string(),
            artists: "Singer Songwriter".to_string(),
            album_name: "Acoustic Sessions".to_string(),
            track_name: "Coffeehouse Favorite".to_string(),
            popularity: 65,
            explicit: "Yes".to_string(),
            danceability: 0.4,
            energy: 0.3,
            key: 1,
            loudness: -14.0,
            mode: 0,
            valence: 0.5,
            tempo: 95.0,
            track_genre: "Acoustic".to_string(),
        };

        let record5: Record = Record {
            track_id: "005".to_string(),
            artists: "Electronic Duo".to_string(),
            album_name: "Synth Adventures".to_string(),
            track_name: "Night Drive".to_string(),
            popularity: 90,
            explicit: "No".to_string(),
            danceability: 0.9,
            energy: 0.9,
            key: 7,
            loudness: -3.5,
            mode: 1,
            valence: 0.7,
            tempo: 130.0,
            track_genre: "Electronic".to_string(),
        };
        let records = vec![record1, record2, record3, record4, record5];
        let k = 2;
        let graph = knn_graph(&records, k);
        assert_eq!(graph.node_count(), records.len());
        assert_eq!(graph.edge_count(), k * records.len());
    }
    fn sim_test() {
        let record1: Record = Record {
            track_id: "001".to_string(),
            artists: "Artist One".to_string(),
            album_name: "First Album".to_string(),
            track_name: "Hit Single".to_string(),
            popularity: 85,
            explicit: "No".to_string(),
            danceability: 0.7,
            energy: 0.8,
            key: 5,
            loudness: -5.4,
            mode: 1,
            valence: 0.6,
            tempo: 120.0,
            track_genre: "Pop".to_string(),
        };

        let record2: Record = Record {
            track_id: "002".to_string(),
            artists: "Band Two".to_string(),
            album_name: "Second Wave".to_string(),
            track_name: "Smooth Ride".to_string(),
            popularity: 75,
            explicit: "Yes".to_string(),
            danceability: 0.6,
            energy: 0.65,
            key: 8,
            loudness: -6.2,
            mode: 0,
            valence: 0.4,
            tempo: 100.0,
            track_genre: "Jazz".to_string(),
        };
        let records = vec![record1.clone(), record2.clone()];
        assert_eq!(sim_calc(&record1, &record1), 1.0);
        assert!(sim_calc(&record1, &record2) > 0.8);
    }
}
