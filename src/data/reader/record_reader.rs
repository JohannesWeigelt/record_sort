use std::fs::File;

pub trait RecordReader<T: PartialOrd> {
    fn read(&self, path: &File, limit: Option<usize>) -> Result<Vec<T>, String>;
}