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
    use crate::data::generation::review_generator::ReviewGenerator;
    use crate::data::writer::json_writer::JSONWriter;
    use crate::data::writer::record_writer::RecordWriter;

    #[test]
    fn success() {
        //TODO Test with tmp-File and delete afterwards

        let writer = JSONWriter;
        writer.write("data_sets/gen_test.json", &mut ReviewGenerator::new(), 10).unwrap();
    }
}