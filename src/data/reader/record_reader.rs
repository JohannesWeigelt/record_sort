pub trait RecordReader<T: PartialOrd> {
    fn read(&self, path: &String, limit: Option<usize>) -> Result<Vec<T>, String>;
}