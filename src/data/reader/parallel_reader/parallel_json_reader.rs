use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;

use crate::data::reader::parallel_reader::area::Area;
use crate::data::reader::parallel_reader::area_splitter::split;
use crate::data::reader::parallel_reader::parallel_json_reader::indexed_part::IndexedPart;
use crate::data::reader::record_reader::RecordReader;
use crate::data::review::Review;

pub struct ParallelJSONReader {
    threads: u8,
}

impl ParallelJSONReader {
    pub fn new(threads: u8) -> Self {
        ParallelJSONReader { threads }
    }

    fn read_part(index: usize, area: Area, file: File, sender: Sender<IndexedPart>) {
        let mut result: Vec<Review> = Vec::new();

        let mut lines = BufReader::new(&file).lines();
        let first_line = lines.nth(*area.get_start() as usize).unwrap().unwrap();
        let mut k = *area.get_start();
        let end = *area.get_end();

        result.push(serde_json::from_str(first_line.as_str()).unwrap());

        for line in lines {
            let json = line.unwrap();
            result.push(serde_json::from_str(json.as_str()).unwrap());

            k = k + 1;
            if k == end { break; }
        }

        sender.send(IndexedPart { index, part: result }).unwrap();
    }
}

impl RecordReader<Review> for ParallelJSONReader {
    fn read(&self, path: &String, limit: Option<usize>) -> Result<Vec<Review>, &str> {
        let (tx, rx) = mpsc::channel::<IndexedPart>();
        let lines = match limit {
            Some(n) => n,
            None => BufReader::new(File::open(&path).unwrap()).lines().count()
        };

        let areas = split(self.threads, lines as u64).unwrap();

        for i in 0usize..self.threads as usize {
            let sender = mpsc::Sender::clone(&tx);
            let area = *areas.get(i).unwrap();
            let file_copy = File::open(&path).unwrap();

            thread::spawn(move || ParallelJSONReader::read_part(i, area, file_copy, sender));
        }
        drop(tx);

        let mut iparts: Vec<IndexedPart> = Vec::new();
        for part in rx {
            iparts.push(part);
        }
        iparts.sort();

        let mut result: Vec<Review> = Vec::new();

        for ipart in iparts {
            let part = ipart.part;

            for review in part {
                result.push(review);
            }
        }

        Ok(result)
    }
}

pub mod indexed_part {
    use std::cmp::Ordering;

    use crate::data::review::Review;

    #[derive(Debug)]
    pub struct IndexedPart {
        pub index: usize,
        pub part: Vec<Review>,
    }

    impl PartialOrd for IndexedPart {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.index.partial_cmp(&other.index)
        }
    }

    impl PartialEq for IndexedPart {
        fn eq(&self, other: &Self) -> bool {
            self.index == other.index
        }
    }

    impl Ord for IndexedPart {
        fn cmp(&self, other: &Self) -> Ordering {
            self.index.cmp(&other.index)
        }
    }

    impl Eq for IndexedPart {}
}
