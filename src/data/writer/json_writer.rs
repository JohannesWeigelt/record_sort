use std::fs::{File, OpenOptions};

use crate::data::generation::record_generator::RecordGenerator;
use crate::data::review::Review;
use crate::data::writer::record_writer::RecordWriter;
use std::io::Write;
use crate::data::record::Record;
use serde::Serialize;

pub struct JSONWriter;

impl<T: Record + Serialize> RecordWriter<T> for JSONWriter {
    fn write(&self, path: &String, generator: &mut dyn RecordGenerator<T>, limit: usize) -> Result<(), &str> {
        //TODO Not existing Path will not automatically be created
        let mut file = OpenOptions::new().append(true).create(true).open(path).unwrap();

        for record in generator.take(limit) {
            let string = serde_json::to_string(&record).unwrap();
            writeln!(file, "{}", string);
//            file.write(string.as_bytes()).unwrap();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::data::writer::json_writer::JSONWriter;
    use crate::data::writer::record_writer::RecordWriter;
    use crate::data::generation::review_generator::ReviewGenerator;

    #[test]
    fn success() {
        let writer = JSONWriter;
        writer.write(&String::from("data_sets/gen.json"), &mut ReviewGenerator::new(&mut rand::thread_rng()), 5);

        assert_eq!(42,42)
    }
}