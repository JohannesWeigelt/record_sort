use rand::distributions::Alphanumeric;
use rand::prelude::ThreadRng;
use rand::Rng;

use crate::data::generation::record_generator::RecordGenerator;
use crate::data::review::{Review, Style};

pub struct ReviewGenerator {
    rng: ThreadRng,
}

impl ReviewGenerator {
    const MIN_OVERALL: f32 = 1.0;
    const MAX_OVERALL: f32 = 5.0;

    const MIN_STRING_SIZE: usize = 16;
    const MAX_STRING_SIZE: usize = 256;

    pub fn new() -> Self {
        ReviewGenerator {
            rng: rand::thread_rng()
        }
    }

    fn generate_string(&mut self) -> String {
        let len: usize = self.rng.gen_range(ReviewGenerator::MIN_STRING_SIZE, ReviewGenerator::MAX_STRING_SIZE);
        self.rng.sample_iter(&Alphanumeric).take(len).collect()
    }

    fn generate_optional_string(&mut self) -> Option<String> {
        match self.rng.gen::<bool>() {
            true => Some(self.generate_string()),
            false => None
        }
    }
}

impl RecordGenerator<Review> for ReviewGenerator {
    fn generate(&mut self) -> Review {
        let overall: f32 = self.rng.gen_range(ReviewGenerator::MIN_OVERALL, ReviewGenerator::MAX_OVERALL);
        let verified: bool = self.rng.gen();
        let review_time = self.generate_string();
        let reviewer_id = self.generate_string();
        let asin = self.generate_string();

        let style: Option<Style> = match self.rng.gen::<bool>() {
            true => Some(Style::new(self.generate_optional_string())),
            false => None
        };

        let reviewer_name = self.generate_optional_string();
        let review_text = self.generate_optional_string();
        let unix_review_time: u128 = self.rng.gen();

        Review::new(overall, verified, review_time, reviewer_id, asin, style, reviewer_name, review_text, unix_review_time)
    }
}