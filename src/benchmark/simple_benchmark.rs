use std::time::{Duration, Instant};

use crate::benchmark::benchmark::Benchmark;
use crate::sort::sort::Sort;
use crate::util::random_number_generator::RandomNumberGenerator;

/// Checks the duration of sorting a Vec with 10-Million u8-Values
pub struct SimpleBenchmark {
    rng: RandomNumberGenerator
}

impl SimpleBenchmark {
    pub fn new(rng: RandomNumberGenerator) -> Self {
        SimpleBenchmark { rng }
    }
}

impl Benchmark for SimpleBenchmark {
    type Item = u8;

    fn execute(&self, sort: &dyn Sort<Self::Item>) -> Duration {
        let mut values = self.rng.generate_u8_numbers(10000000);

        let start = Instant::now();
        sort.sort(&mut values);

        start.elapsed()
    }
}