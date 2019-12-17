use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::time::Instant;

use crate::benchmark::benchmark::Benchmark;
use crate::benchmark::measurement::Measurement;
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
    const DEFAULT_FILE: &'static str = "data_sets/Sports_and_Outdoors.json";
    const DEFAULT_LIMIT: usize = 10000000;
    const DEFAULT_STEP: usize = 500000;


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
            path: String::from(RealDataBenchmark::DEFAULT_FILE),
            limit: Some(RealDataBenchmark::DEFAULT_LIMIT),
            step: RealDataBenchmark::DEFAULT_STEP,
        }
    }

    fn measure_sort(&self, sort: &dyn Sort<Review>, lines: usize) -> Vec<Measurement> {
        let mut result = Vec::new();
        let mut file = File::open(&self.path).unwrap();

        let mut i = self.step;

        while i <= lines {
            result.push(self.sort_elements(&mut file, sort, Some(i)));
            i += self.step
        }

        result
    }

    fn sort_elements(&self, file: &mut File, sort: &dyn Sort<Review>, limit: Option<usize>) -> Measurement {
        file.seek(SeekFrom::Start(0)).unwrap();

        let mut records = self.reader.read(file, limit).unwrap();
        let len = records.len();
        println!("Elements: {}", len);

        let start = Instant::now();
        sort.sort(&mut records);
        let duration = start.elapsed().as_secs_f64();

        println!("Duration: {} seconds", duration);

        Measurement::new(len, duration)
    }
}

impl Benchmark for RealDataBenchmark {
    type Item = Review;

    fn execute(&self, sort: &dyn Sort<Self::Item>) -> Vec<Measurement> {
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