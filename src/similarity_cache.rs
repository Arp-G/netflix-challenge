use std::collections::HashMap;

pub fn store_in_cache(cache: &mut HashMap<String, f64>, key: String, similarity: f64) {
    cache.insert(key, similarity);
}

pub fn get_key(user1_id: u32, user2_id: u32) -> String {
    if user1_id > user2_id {
        format!("{}-{}", user1_id, user2_id)
    } else {
        format!("{}-{}", user2_id, user1_id)
    }
}
