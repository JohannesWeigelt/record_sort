use crate::data::record::Record;

///Trait for every object that generates a new record.
pub trait RecordGenerator<T: Record>  {
    fn generate(&mut self) -> T;
}

impl<T: Record> Iterator for &mut dyn RecordGenerator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.generate())
    }
}