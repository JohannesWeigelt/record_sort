use std::fs::File;
use std::io::{BufRead, BufReader};

use serde::de::DeserializeOwned;

use crate::data::reader::record_read_error::RecordReadError;
use crate::data::reader::record_reader::RecordReader;
use crate::data::record::Record;

//Implementation of RecordReader. The JSONReader reads files, where every line is a valid JSON-Object
pub struct JSONReader;

impl JSONReader {
    fn read_full<T: Record + DeserializeOwned>(&self, reader: BufReader<&File>) -> Result<Vec<T>, RecordReadError> {
        let mut reviews = Vec::new();

        for line in reader.lines() {
            let json = line?;
            let review = serde_json::from_str(json.as_str())?;

            reviews.push(review);
        }

        Ok(reviews)
    }

    fn read_limited<T: Record + DeserializeOwned>(&self, reader: BufReader<&File>, mut limit: usize) -> Result<Vec<T>, RecordReadError> {
        let mut reviews = Vec::new();

        for line in reader.lines() {
            if limit == 0 { break; }

            let json = line?;
            let review = serde_json::from_str(json.as_str())?;

            reviews.push(review);

            limit -= 1;
        }

        Ok(reviews)
    }
}

impl<T: Record + DeserializeOwned> RecordReader<T> for JSONReader {
    fn read(&self, path: &String, limit: Option<usize>) -> Result<Vec<T>, RecordReadError> {
        let file = &File::open(path)?;
        let reader = BufReader::new(file);

        match limit {
            Some(n) => self.read_limited(reader, n),
            None => self.read_full(reader)
        }
    }
}

#[cfg(test)]
mod test {}