use crate::record::Record;
use crate::utils::{find_similar, search};

pub fn recommend(data: &Vec<Record>, target: &str, artist: &str, top_n: usize) {
    let target_track = search(data, target, Some(artist));
    if let Some(track) = target_track {
        let similar = find_similar(data, track, top_n);
        println!("Recommendations for '{} by {}:", target, artist);
        for similar_track in similar {
            println!("- '{}' by {}", similar_track.track_name, similar_track.artists);
        }
    } else {
        println!("Track '{}' by '{}' not found", target, artist);
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
