use crate::common::Rating;
use std::collections::HashMap;

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
