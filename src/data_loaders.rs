use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
// use crate::helpers;

#[derive(Debug)]
pub struct Rating {
    rating: u8,
    centered_rating: u8
}

impl Rating {
    pub fn new(rating: u8) -> Rating {
        Rating {
            rating: rating,
            centered_rating: 0
        }
    }
}

pub fn load(file_path: &str) -> HashMap<u32, HashMap<u32, Rating>> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut ratings_hash = HashMap::new();
    let mut movie_id = 1;
    // 348960,1,2005-09-07
    for line in reader.lines() {
        let mut result = line.unwrap();

        if result.chars().last().unwrap() == ':' {
            result.pop(); // remove ":"
            movie_id = result.parse::<u32>().unwrap();
        } else {

            let mut iter = result.split(",");
            let user_id = iter.next().unwrap().parse::<u32>().unwrap();

            let rating = iter.next().unwrap().parse::<u8>().unwrap();
            ratings_hash
                .entry(user_id)
                .or_insert(HashMap::new())
                .insert(movie_id, Rating::new(rating));
        }
    }

    println!("{:?}", ratings_hash);
    ratings_hash
}
