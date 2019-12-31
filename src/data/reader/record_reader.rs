use crate::data::record::Record;
use crate::data::record_io_error::RecordIOError;

//Trait of all objects that can read sortable records from a given file
pub trait RecordReader<T: Record> {
    fn read(&self, path: &str, limit: Option<usize>) -> Result<Vec<T>, RecordIOError>;
}