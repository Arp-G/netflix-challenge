#![allow(unused)]
use crate::common::Rating;
use crate::similarity_cache;
use std::cmp::Ordering;
use std::collections::HashMap;

const MOVIE_IDS: u32 = 17771; // Movie Ids from 1 to 17770
const K: usize = 500; // PICK "K" most similar users for prediction

pub fn center_ratings(all_ratings: &mut HashMap<u32, HashMap<u32, Rating>>) {
    for (_user_id, user_ratings) in all_ratings {
        let total = user_ratings
            .values()
            .map(|user_movie_rating| user_movie_rating.rating as u64)
            .sum::<u64>();

        let avg_rating = total as f64 / user_ratings.values().count() as f64;

        for (_movie_id, user_movie_rating) in user_ratings {
            user_movie_rating.center_rating(avg_rating)
        }
    }
}

pub fn predict_rating(
    user_id: u32,
    movie_id: u32,
    all_ratings: &HashMap<u32, HashMap<u32, Rating>>,
    cache: &mut HashMap<String, f64>,
) -> u8 {
    let similar_users = find_k_most_similar_users(user_id, movie_id, all_ratings, cache);

    calculate_rating(similar_users, movie_id, all_ratings)
}

fn calculate_rating(
    similar_users: Vec<(u32, f64)>,
    target_movie_id: u32,
    all_ratings: &HashMap<u32, HashMap<u32, Rating>>,
) -> u8 {
    let (numerator, denominator) = similar_users.iter().fold(
        (0.0, 0.0),
        |(numerator, denominator), (user_id, similarity)| {
            let rating = all_ratings
                .get(user_id)
                .unwrap()
                .get(&target_movie_id)
                .unwrap()
                .rating;

            (
                numerator + rating as f64 * similarity,
                denominator + similarity,
            )
        },
    );

    let prediction = (numerator / denominator);

    if (prediction - prediction.floor() < 0.5) {
        prediction.floor() as u8
    } else {
        prediction.ceil() as u8
    }
}

fn find_k_most_similar_users(
    target_user_id: u32,
    target_movie_id: u32,
    all_ratings: &HashMap<u32, HashMap<u32, Rating>>,
    cache: &mut HashMap<String, f64>,
) -> Vec<(u32, f64)> {
    let mut similar_users: Vec<(u32, f64)> = all_ratings
        .iter()
        .filter_map(|(user_id, user_ratings)| {
            if user_ratings.contains_key(&target_movie_id) && *user_id != target_user_id {
                let key = similarity_cache::get_key(target_user_id, *user_id);

                let similarity = match cache.get(&key) {
                    Some(similarity) => {
                       // println!("Cache hit for {}", key);
                        *similarity
                    },
                    None => {
                        let sim = cosine_similarity(target_user_id, *user_id, &all_ratings);
                        similarity_cache::store_in_cache(cache, key, sim);
                        sim
                    }
                };

                Some((*user_id, similarity))
            } else {
                None
            }
        })
        .collect::<Vec<(u32, f64)>>();

    // here .unwrap_or(Ordering::Less) is for a case where similarity was Nan,
    // Ordering::Less ensures they will appear last in the sorted list
    // this happens when a user as rated all movies equally so centered_rating = 0,
    // thus magnitude(users_rating) = 0, thus denominator when
    // calcutaing cosine similarity becomes 0, resulting in Nan value
    similar_users
        .sort_unstable_by(|(_, sim1), (_, sim2)| sim2.partial_cmp(sim1).unwrap_or(Ordering::Less));

    similar_users
        .into_iter()
        .take(K)
        .collect::<Vec<(u32, f64)>>()
}

fn cosine_similarity(
    user1_id: u32,
    user2_id: u32,
    all_ratings: &HashMap<u32, HashMap<u32, Rating>>,
) -> f64 {
    let user1_ratings: &HashMap<u32, Rating> = all_ratings.get(&user1_id).unwrap();
    let user2_ratings: &HashMap<u32, Rating> = all_ratings.get(&user2_id).unwrap();
    // A.B
    let numerator = (1..MOVIE_IDS).fold(0.0, |acc, movie_id| {
        let user1_movie_rating = match user1_ratings.get(&movie_id) {
            Some(rating) => rating.centered_rating,
            None => 0.0,
        };
        let user2_movie_rating = match user2_ratings.get(&movie_id) {
            Some(rating) => rating.centered_rating,
            None => 0.0,
        };
        acc + (user1_movie_rating * user2_movie_rating)
    });

    // |A|.|B|
    let denominator = magnitude(user1_ratings) * magnitude(user2_ratings);

    let similarity = numerator / denominator;

    if (similarity.is_normal()) {
        similarity
    } else {
        if (similarity == 0.0) {
            0.0
        } else {
            -1.0
        }
    }
}

fn magnitude(ratings: &HashMap<u32, Rating>) -> f64 {
    ratings
        .values()
        .fold(0.0, |acc, rating| {
            acc + rating.centered_rating * rating.centered_rating
        })
        .sqrt()
}
