use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::data::reader::record_reader::RecordReader;
use crate::data::review::Review;

pub struct JSONReader;

impl JSONReader {
    fn read_full(&self, reader: BufReader<File>) -> Vec<Review> {
        let mut reviews = Vec::new();

        for line in reader.lines() {
            let json = line.unwrap();
            let review = serde_json::from_str(json.as_str()).unwrap();

            reviews.push(review);
        }

        reviews
    }

    fn read_limited(&self, reader: BufReader<File>, mut limit: usize) -> Vec<Review> {
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

impl RecordReader<Review> for JSONReader {
    fn read(&self, path: &String, limit: Option<usize>) -> Result<Vec<Review>, &str> {
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