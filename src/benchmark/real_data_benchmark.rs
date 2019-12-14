use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use crate::benchmark::benchmark::Benchmark;
use crate::benchmark::metering_result::MeteringResult;
use crate::data::reader::json_reader::JSONReader;
use crate::data::reader::record_reader::RecordReader;
use crate::data::review::Review;
use crate::sort::sort::Sort;

pub struct RealDataBenchmark {
    reader: JSONReader,
    path: String,
    limit: Option<usize>,
    step: usize,
}

impl RealDataBenchmark {
    pub fn new(reader: JSONReader, path: String, limit: Option<usize>, step: usize) -> Self {
        RealDataBenchmark {
            reader,
            path,
            limit,
            step,
        }
    }

    pub fn default() -> Self {
        RealDataBenchmark {
            reader: JSONReader,
            path: String::from("data_sets/foo_bar.json"),
            limit: Some(10000000),
            step: 100000,
        }
    }

    fn measure_sort(&self, sort: &dyn Sort<Review>, lines: usize) -> Vec<MeteringResult> {
        let mut result = Vec::new();

        let mut i = self.step;

        while i <= lines {
            result.push(self.sort_elements(sort, Some(i)));
            i += self.step
        }

        result
    }

    fn sort_elements(&self, sort: &dyn Sort<Review>, limit: Option<usize>) -> MeteringResult {
        let mut records = self.reader.read(&self.path, limit).unwrap();
        let len = records.len();
        println!("Elements: {}", len);

        let start = Instant::now();
        sort.sort(&mut records);
        let duration = start.elapsed().as_secs_f64();

        println!("Duration: {} seconds", duration);

        MeteringResult::new(len, duration)
    }
}

impl Benchmark for RealDataBenchmark {
    type Item = Review;

    fn execute(&self, sort: &dyn Sort<Self::Item>) -> Vec<MeteringResult> {
        let lines = match self.limit {
            Some(n) => n,
            None => {
                println!("Count lines");
                BufReader::new(File::open(self.path.clone()).unwrap()).lines().count()
            }
        };

        self.measure_sort(sort, lines)
    }
}