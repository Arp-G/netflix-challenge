#[derive(Debug)]
pub struct Rating {
    pub rating: u8,
    pub centered_rating: f64,
}

impl Rating {
    pub fn new(rating: u8) -> Rating {
        Rating {
            rating: rating,
            centered_rating: 0.0,
        }
    }

    pub fn center_rating(&mut self, avg_rating: f64) {
        self.centered_rating = self.rating as f64 - avg_rating;
    }
}
