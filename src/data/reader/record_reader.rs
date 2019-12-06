pub trait RecordReader<T: PartialOrd> {
    fn read(&self, path: &str, limit: Option<usize>) -> Result<Vec<T>, &str>;
}