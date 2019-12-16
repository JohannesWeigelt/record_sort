use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::data::reader::record_reader::RecordReader;
use crate::data::review::Review;
use serde::Deserialize;

pub struct JSONReader;

impl JSONReader {
    fn read_full<'a, T: PartialOrd + Deserialize<'a>>(&self, reader: BufReader<File>) -> Vec<T> {
        let mut reviews = Vec::new();

        for line in reader.lines() {
            let json = line.unwrap();
            let review = serde_json::from_str(json.as_str()).unwrap();

            reviews.push(review);
        }

        reviews
    }

    fn read_limited<'a, T: PartialOrd + Deserialize<'a>>(&self, reader: BufReader<File>, mut limit: usize) -> Vec<T> {
        let mut reviews = Vec::new();

        for line in reader.lines() {
            if limit == 0 { break; }

            let json = line.unwrap();
            let review = serde_json::from_str(json.as_str()).unwrap();

            reviews.push(review);

            limit -= 1;
        }

        reviews
    }
}

impl<'a, T: PartialOrd + Deserialize<'a>> RecordReader<T> for JSONReader {
    fn read(&self, path: &String, limit: Option<usize>) -> Result<Vec<T>, String> {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        Ok(
            match limit {
                Some(n) => self.read_limited(reader, n),
                None => self.read_full(reader)
            }
        )
    }
}

#[cfg(test)]
mod test {

}