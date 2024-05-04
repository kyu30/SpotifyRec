#[warn(unused_imports)]
pub mod record;
pub mod utils;
use std::io::{self, Write};
fn main() {
    if let Err(err) = utils::random_sample("dataset.csv", "dataset1.csv", 1000) {
        println!("Error sampling data: {}", err);
    }
    let file = "dataset1.csv";
    let mut data = utils::read(file);
    for record in &mut data {
        record.clean()
    }
    let graph = utils::graph(&data);
    let centrality = utils::calc_centrality(&graph);
    loop {
        print!("Enter a song name: ");
        io::stdout().flush().unwrap();
        let mut track_name = String::new();
        io::stdin().read_line(&mut track_name).unwrap();
        track_name = track_name.trim().to_string();

        print!("Enter an artist name: ");
        io::stdout().flush().unwrap();
        let mut artist_name = String::new();
        io::stdin().read_line(&mut artist_name).unwrap();
        artist_name = artist_name.trim().to_string();
        let found_track = utils::search(
            &data,
            &track_name,
            if artist_name.is_empty() {
                None
            } else {
                Some(&artist_name)
            },
        );

        if let Some(target_track) = found_track {
            utils::top(&data, &graph, &target_track.track_id, &centrality);
            break;
        } else {
            println!(
                "Track '{}' not found. Would you like to try again? (y/n): ",
                track_name
            );
            let mut response = String::new();
            io::stdin().read_line(&mut response).unwrap();
            if response.trim().to_lowercase() != "y" {
                break;
            }
        }
    }
}
