use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};

use crate::data::reader::json_reader::JSONReader;
use crate::data::reader::record_reader::RecordReader;
use crate::sort::merge_sort::MergeSort;
use crate::sort::sort::Sort;

pub struct ConsoleApplication {
    review_reader: JSONReader,
    review_sort: MergeSort,
}

impl ConsoleApplication {
    pub fn new(review_reader: JSONReader, review_sort: MergeSort) -> Self {
        ConsoleApplication {
            review_reader,
            review_sort,
        }
    }

    pub fn run(&self, path: &str, limit: Option<usize>, step: usize) {
        let lines = match limit {
            Some(n) => n,
            None => {
                println!("Count lines");
                BufReader::new(File::open(path).unwrap()).lines().count()
            }
        };

        self.print_measurements(self.measure_sort(path, step, lines));
    }

    fn measure_sort(&self, path: &str, step: usize, lines: usize) -> Vec<(usize, Duration)> {
        let mut result = Vec::new();

        let mut i = step;

        while i <= lines {
            result.push(self.sort_elements(path, Some(i)));
            i += step
        }

        result
    }


    fn sort_elements(&self, path: &str, limit: Option<usize>) -> (usize, Duration) {
        let mut records = self.review_reader.read(path, limit).unwrap();
        let len = records.len();
        println!("Elements: {}", len);

        let start = Instant::now();
        self.review_sort.sort(&mut records);
        let duration = start.elapsed();

        println!("Duration: {:?}", duration);

        (len, duration)
    }

    fn print_measurements(&self, measurements: Vec<(usize, Duration)>) {
        println!("Results: ");

        for (k, v) in measurements {
            println!("Lines: {}, Duration: {:?}", k, v)
        }
    }
}