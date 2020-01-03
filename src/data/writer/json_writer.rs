use std::fs::File;
use std::io::Write;

use serde::Serialize;

use crate::data::generation::record_generator::RecordGenerator;
use crate::data::record::Record;
use crate::data::record_io_error::RecordIOError;
use crate::data::writer::record_writer::RecordWriter;

pub struct JSONWriter;

impl<T: Record + Serialize> RecordWriter<T> for JSONWriter {
    fn write(&self, path: &str, generator: &mut dyn RecordGenerator<T>, limit: usize) -> Result<(), RecordIOError> {
        //TODO Not existing Path will not automatically be created
        let mut file = File::create(path)?;

        for record in generator.take(limit) {
            let string = serde_json::to_string(&record)?;
            writeln!(file, "{}", string)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;

    use crate::data::generation::review_generator::ReviewGenerator;
    use crate::data::writer::json_writer::JSONWriter;
    use crate::data::writer::record_writer::RecordWriter;

    const TEST_FILE: &'static str = "tests/data_sets/write_test.json";
    const RECORDS: usize = 10;

    #[test]
    fn success() {
        assert!(!Path::new(TEST_FILE).exists());

        let writer = JSONWriter;
        writer.write(TEST_FILE, &mut ReviewGenerator::new(), RECORDS).unwrap();

        let file = File::open(TEST_FILE).unwrap();
        let lines = BufReader::new(file).lines().count();

        assert_eq!(lines, RECORDS);

        fs::remove_file(TEST_FILE).unwrap();
    }
}