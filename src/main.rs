mod collaborative_filtering;
mod common;
mod data_loaders;
mod similarity_cache;
use std::collections::HashMap;
use std::time::SystemTime;

extern crate colored;
use colored::*;

fn main() {
    let now = SystemTime::now();

    // Serailly loading files takes 53 seconds
    // Concurrently loading files takes 17 seconds
    // Merging hashes takes ~4 seconds,
    // Centering ratings: ~6 seconds
    // So total time to load data parallelly = 21 seconds(release mode) (221 seconds in debug mode)
    let mut data = data_loaders::parallel_loader(vec![
        "data/combined_data_1.txt",
        "data/combined_data_2.txt",
        "data/combined_data_3.txt",
        "data/combined_data_4.txt",
    ]);

    collaborative_filtering::center_ratings(&mut data);

    let probe_data = data_loaders::load_probe_data("data/probe.txt");

    let mut correct_count = 0;
    let mut almost_correct_count = 0;
    let mut cache = HashMap::new();

    // This test took 28.63 min,
    // total predictions made: 500,
    // correct predictions: 207,
    // almost correct predictions: 243
    // Without caching for 100 ratings time = 131 sec, with caching: 123 sec
    for (user_id, movie_id) in probe_data.iter().take(500) {
        let actual = data.get(&user_id).unwrap().get(&movie_id).unwrap().rating;
        let prediction =
            collaborative_filtering::predict_rating(*user_id, *movie_id, &data, &mut cache);

        let diff = (actual as i32 - prediction as i32).abs();

        let op = if diff == 0 {
            correct_count += 1;
            prediction.to_string().green().bold()
        } else if diff == 1 {
            almost_correct_count += 1;
            prediction.to_string().yellow().bold()
        } else {
            prediction.to_string().red().bold()
        };

        println!("Actual Rating: {}, Predicted Rating: {}", actual, op);
    }

    println!(
        "Correct = {}, Almost Correct = {}",
        correct_count, almost_correct_count,
    );

    match now.elapsed() {
        Ok(elapsed) => {
            println!("{}", elapsed.as_secs());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
