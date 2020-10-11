use crate::common::Rating;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::sync::mpsc;
use std::thread;

pub fn parallel_loader(files: Vec<&'static str>) -> HashMap<u32, HashMap<u32, Rating>> {
    let (tx, rx) = mpsc::channel();

    for file in &files {
        let sender = mpsc::Sender::clone(&tx);

        let file = file.clone();
        thread::spawn(move || {
            sender.send(load(file)).unwrap();
        });
    }

    let mut files_iter = rx.iter();
    let mut combined_ratings = files_iter.next().unwrap();

    combine_hashes(&mut combined_ratings, files_iter.next().unwrap());
    combine_hashes(&mut combined_ratings, files_iter.next().unwrap());
    combine_hashes(&mut combined_ratings, files_iter.next().unwrap());

    combined_ratings
}

fn load(file_path: &str) -> HashMap<u32, HashMap<u32, Rating>> {
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

    println!("Finished processing file {}", file_path);
    ratings_hash
}

pub fn load_probe_data(file_path: &str) -> Vec<(u32, u32)> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut data_vec: Vec<(u32, u32)> = Vec::new();
    let mut movie_id = 1;

    for line in reader.lines() {
        let mut result = line.unwrap();

        if result.chars().last().unwrap() == ':' {
            result.pop(); // remove ":"
            movie_id = result.parse::<u32>().unwrap();
        } else {
            let mut iter = result.split(",");
            let user_id = iter.next().unwrap().parse::<u32>().unwrap();

            data_vec.push((user_id, movie_id));
        }
    }

    data_vec
}

fn combine_hashes(
    combined_ratings: &mut HashMap<u32, HashMap<u32, Rating>>,
    new_ratings: HashMap<u32, HashMap<u32, Rating>>,
) {
    for (user_id, movie_ratings_for_user) in new_ratings {
        match combined_ratings.get_mut(&user_id) {
            Some(old_hash_map) => {
                old_hash_map.extend(movie_ratings_for_user.into_iter());
            }

            None => {
                combined_ratings.insert(user_id, movie_ratings_for_user);
            }
        };
    }
}
