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

    const ASIN_LENGTH: usize = 10;
    const ID_LENGTH: usize = 14;

    const NAME_LENGTH_MIN: usize = 6;
    const NAME_LENGTH_MAX: usize = 10;

    const MIN_STYLE_STRING_SIZE: usize = 8;
    const MAX_STYLE_STRING_SIZE: usize = 10;

    const DD_MM_LENGTH: usize = 2;
    const YYYY_LENGTH: usize = 4;

    const UNIX_TIME_MIN: u128 = 1000000000;
    const UNIX_TIME_MAX: u128 = 16000000000;

    pub fn new() -> Self {
        ReviewGenerator {
            rng: rand::thread_rng()
        }
    }

    fn generate_string(&mut self, min: usize, max: usize) -> String {
        let len: usize = self.rng.gen_range(min, max);
        self.rng.sample_iter(&Alphanumeric).take(len).collect()
    }

    fn generate_string_fixed(&mut self, length: usize) -> String {
        self.rng.sample_iter(&Alphanumeric).take(length).collect()
    }

    fn generate_optional_string(&mut self, min: usize, max: usize) -> Option<String> {
        match self.rng.gen::<bool>() {
            true => Some(self.generate_string(min, max)),
            false => None
        }
    }

    fn generate_asin(&mut self) -> String {
        self.rng.sample_iter(&Alphanumeric).take(ReviewGenerator::ASIN_LENGTH).collect()
    }

    fn generate_id(&self) -> String {
        self.rng.sample_iter(&Alphanumeric).take(ReviewGenerator::ID_LENGTH).collect()
    }

    fn generate_style(&mut self) -> Option<Style> {
        match self.rng.gen::<bool>() {
            true => Some(Style::new(self.generate_optional_string(ReviewGenerator::MIN_STYLE_STRING_SIZE, ReviewGenerator::MAX_STYLE_STRING_SIZE))),
            false => None
        }
    }

    fn generate_time(&mut self) -> String {
        let mut date = String::new();
        date.push_str(self.generate_string_fixed(ReviewGenerator::DD_MM_LENGTH).as_str());
        date.push_str(" ");
        date.push_str(self.generate_string_fixed(ReviewGenerator::DD_MM_LENGTH).as_str());
        date.push_str(", ");
        date.push_str(self.generate_string_fixed(ReviewGenerator::YYYY_LENGTH).as_str());

        date
    }

    fn generate_name(&mut self) -> Option<String> {
        match self.rng.gen::<bool>() {
            false => None,
            true => {
                let mut name = String::new();
                name.push_str(self.generate_string(ReviewGenerator::NAME_LENGTH_MIN, ReviewGenerator::NAME_LENGTH_MAX).as_str());
                name.push_str(" ");
                name.push_str(self.generate_string(ReviewGenerator::NAME_LENGTH_MIN, ReviewGenerator::NAME_LENGTH_MAX).as_str());

                Some(name)
            }
        }
    }

    fn generate_unix_time(&mut self) -> u128 {
        self.rng.gen_range(ReviewGenerator::UNIX_TIME_MIN, ReviewGenerator::UNIX_TIME_MAX)
    }
}

impl RecordGenerator<Review> for ReviewGenerator {
    fn generate(&mut self) -> Review {
        let overall: f32 = self.rng.gen_range(ReviewGenerator::MIN_OVERALL, ReviewGenerator::MAX_OVERALL);
        let verified: bool = self.rng.gen();
        let review_time = self.generate_time();
        let reviewer_id = self.generate_id();
        let asin = self.generate_asin();
        let style = self.generate_style();
        let reviewer_name = self.generate_name();
        let review_text = self.generate_optional_string(ReviewGenerator::MIN_STRING_SIZE, ReviewGenerator::MAX_STRING_SIZE);
        let unix_review_time: u128 = self.generate_unix_time();

        Review::new(overall, verified, review_time, reviewer_id, asin, style, reviewer_name, review_text, unix_review_time)
    }
}

#[cfg(test)]
mod test {
    use crate::data::generation::record_generator::RecordGenerator;
    use crate::data::generation::review_generator::ReviewGenerator;
    use crate::data::review::Review;

    #[test]
    fn success() {
        let generator: &mut dyn RecordGenerator<Review> = &mut ReviewGenerator::new();

        for review in generator.take(10) {
            println!("{:?}", review)
        }
    }
}