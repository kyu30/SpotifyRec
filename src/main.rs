//Add artist lookup/recommendations, avg distance calc in impl, mod library of something
#[warn(unused_imports)]
pub mod record;
pub mod utils;
pub mod recs;
//pub mod graphs;
use std::io::{self, Write, Read};
fn main(){
    if let Err(err) = utils::random_sample("dataset.csv", "dataset1.csv", 500) {
        println!("Error sampling data: {}", err);
    }
    let file = "dataset1.csv";
    let mut data = utils::read(file);
    for mut record in &mut data{
        record.clean()
    }
    let graph = utils::knn_graph(&data, 25);
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
        let found_track = utils::search(&data, &track_name, if artist_name.is_empty() {
            None
        } else {
            Some(&artist_name)
        });

        if let Some(target_track) = found_track {
            let dist = found_track.unwrap().avg_dist(&data);
            //println!("The average distance of {} to other songs is {}", &track_name, dist);
            utils::top(&data, &graph, &target_track.track_id, &centrality);
            //recs::recommend(&data, &track_name, &artist_name, 5);
            break;
        } else {
            println!("Track '{}' not found. Would you like to try again? (y/n): ", track_name);
            let mut response = String::new();
            io::stdin().read_line(&mut response).unwrap();
            if response.trim().to_lowercase() != "y" {
                break;
            }
        }
        }
    }