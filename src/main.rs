mod collaborative_filtering;
mod common;
mod data_loaders;
mod similarity_cache;
use crate::common::Rating;
use std::collections::HashMap;
use std::time::SystemTime;
// extern crate colored;
extern crate num_cpus;
use rayon::prelude::*;
// use colored::*;

fn main() {
    let now = SystemTime::now();

    // Serailly loading files takes 53 seconds
    // Concurrently loading files takes 17 seconds
    // Merging hashes takes ~4 seconds,
    // Centering ratings: ~6 seconds
    // So total time to load data parallelly = 21 seconds(release mode) (221 seconds in debug mode)
    // For 100 ratings...
    // taking floor of prediction gave, Correct = 24, Almost Correct = 45
    // taking ceil of predictions gave, Correct = 41, Almost Correct = 47
    // Rounding up predictions gave, Correct = 38, Almost Correct = 48

    // 862 sec for 500 data with  0.9704625880814568 RMSE

    let mut data: HashMap<u32, HashMap<u32, Rating>> = data_loaders::parallel_loader(vec![
        "data/combined_data_1.txt",
        "data/combined_data_2.txt",
        "data/combined_data_3.txt",
        "data/combined_data_4.txt",
    ]);

    collaborative_filtering::center_ratings(&mut data);

    let probe_data: Vec<(u32, u32)> = data_loaders::load_probe_data("data/probe.txt")
        .into_iter()
        .take(500)
        .collect();

    // let mut correct_count = 0;
    // let mut almost_correct_count = 0;
    // let mut cache:HashMap<String, f64> = HashMap::new();

    // This test took 28.63 min,
    // total predictions made: 500,
    // correct predictions: 207,
    // almost correct predictions: 243
    // Without caching for 100 ratings time = 131 sec, with caching: 123 sec

    let predictions = parallel_predict(probe_data, &data);

    let square_error = predictions
        .iter()
        .fold(0.0, |acc, (user_id, movie_id, prediction)| {
            let actual = data.get(&user_id).unwrap().get(&movie_id).unwrap().rating;

            println!("Actual: {}    Predicted: {}", actual, prediction);
            acc + (prediction - actual as f64).powf(2.0)
        });

    println!("FINAL ACC = {}", square_error);
    let root_mean_square_error = (square_error / 500.0).sqrt();

    println!("RMS = {}", root_mean_square_error);

    // for (user_id, movie_id) in probe_data.iter().take(100) {
    //     let actual = data.get(&user_id).unwrap().get(&movie_id).unwrap().rating;
    //     let prediction =
    //         collaborative_filtering::predict_rating(*user_id, *movie_id, &data, &mut cache);

    //     let diff = (actual as i32 - prediction as i32).abs();

    //     let op = if diff == 0 {
    //         correct_count += 1;
    //         prediction.to_string().green().bold()
    //     } else if diff == 1 {
    //         almost_correct_count += 1;
    //         prediction.to_string().yellow().bold()
    //     } else {
    //         prediction.to_string().red().bold()
    //     };

    //     println!("Actual Rating: {}, Predicted Rating: {}", actual, op);
    // }

    // println!(
    //     "Correct = {}, Almost Correct = {}",
    //     correct_count, almost_correct_count,
    // );

    match now.elapsed() {
        Ok(elapsed) => {
            println!("{}", elapsed.as_secs());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

fn parallel_predict(
    prob_data: Vec<(u32, u32)>,
    data: &HashMap<u32, HashMap<u32, Rating>>,
) -> Vec<(u32, u32, f64)> {
    let cpu_count = num_cpus::get();
    let mut iter = prob_data.into_iter();
    let iter = iter.by_ref(); // Borrows an iterator, rather than consuming it.
    let chunk_len = (iter.len() / cpu_count) as usize + 1;
    let mut chunks: Vec<Vec<(u32, u32)>> = Vec::new();

    for _ in 0..cpu_count {
        let chunk: Vec<(u32, u32)> = iter.take(chunk_len).collect();
        chunks.push(chunk);
    }

    let chunks: Vec<Vec<(u32, u32, f64)>> = chunks
        .par_iter()
        .map(|chunk| {
            let mut cache = HashMap::new();
            chunk
                .iter()
                .map(|(user_id, movie_id)| {
                    let prediction = collaborative_filtering::predict_rating(
                        *user_id, *movie_id, &data, &mut cache,
                    );
                    (*user_id, *movie_id, prediction)
                })
                .collect()
        })
        .collect();

    let mut result: Vec<(u32, u32, f64)> = Vec::new();

    for mut chunk in chunks {
        result.append(&mut chunk);
    }

    result

    // let (tx, rx) = crossbeam_channel::unbounded();
    // let mut i = 0;
    // for chunk in chunks.into_iter() {
    //     let sender = tx.clone();

    //     crossbeam_utils::thread::scope(|s| {
    //         // This thread is scoped, meaning it's guaranteed to terminate before the scope exits,
    //         // allowing it to reference variables outside the scope.
    //         // Using scoped threads from crossbeam crate instead of mpsc allows us to avoid static lifetime
    //         // problems with threads
    //         s.spawn(|_| {
    //             println!("NEW THREAD SPAWNED !");
    //             let mut cache = HashMap::new();
    //             let predictions: Vec<(u32, u32, f64)> = chunk
    //                 .iter()
    //                 .map(|(user_id, movie_id)| {
    //                     let prediction = collaborative_filtering::predict_rating(
    //                         *user_id, *movie_id, &data, &mut cache,
    //                     );
    //                     println!("FROM THREAD {}", i);
    //                     (*user_id, *movie_id, prediction)
    //                 })
    //                 .collect();

    //             sender.send(predictions).unwrap();
    //         });
    //     });

    //     i = i+1;
    // }

    // Collect all the results from the channel into a single vector
    // let mut chunk_iter = rx.iter();
    // let mut predictions: Vec<(u32, u32, f64)> = Vec::new();

    // for _ in 0..cpu_count {
    //     predictions.append(&mut chunk_iter.next().unwrap());
    // }

    // predictions
}
