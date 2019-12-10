use std::time::Instant;

use crate::benchmark::benchmark::Benchmark;
use crate::benchmark::metering_result::MeteringResult;
use crate::sort::sort::Sort;
use crate::util::random_number_generator::RandomNumberGenerator;

/// Checks the duration of sorting a Vec with 10-Million u8-Values
///
/// Result is the average time of 'times' sorts
pub struct SimpleBenchmark {
    rng: RandomNumberGenerator,
    times: u64,
}

impl SimpleBenchmark {
    pub fn new(rng: RandomNumberGenerator) -> Self {
        SimpleBenchmark { times: 10, rng }
    }
}

impl Benchmark for SimpleBenchmark {
    type Item = u8;

    fn execute(&self, sort: &dyn Sort<Self::Item>) -> Vec<MeteringResult> {
        let mut result = Vec::new();
        let values = self.rng.generate_u8_numbers(10000000);
        let mut seconds_sum: f64 = 0.0;

        for _ in 0..self.times {
            let mut values_copy = values.clone();

            let start = Instant::now();
            sort.sort(&mut values_copy);
            seconds_sum += start.elapsed().as_secs_f64();
        }

        result.push(MeteringResult::new(values.len(), seconds_sum / self.times as f64));
        result
    }
}