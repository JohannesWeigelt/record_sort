use std::fs::File;

use crate::data::reader::record_read_error::RecordReadError;

//Trait of all objects that can read sortable records from a given file
pub trait RecordReader<T: PartialOrd> {
    fn read(&self, path: &File, limit: Option<usize>) -> Result<Vec<T>, RecordReadError>;
}