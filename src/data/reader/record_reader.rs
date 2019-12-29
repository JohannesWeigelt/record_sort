use std::fs::File;

use crate::data::reader::record_read_error::RecordReadError;
use crate::data::record::Record;

//Trait of all objects that can read sortable records from a given file
pub trait RecordReader<T: Record> {
    fn read(&self, path: &String, limit: Option<usize>) -> Result<Vec<T>, RecordReadError>;
}