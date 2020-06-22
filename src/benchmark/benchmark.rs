use crate::benchmark::benchmark_error::BenchmarkError;
use crate::benchmark::measurement::Measurement;
use crate::sort::sort::Sort;

/// Tests the speed of the implementation of a given sorting-algorithm.
pub trait Benchmark {
    type Item: PartialOrd;

    fn execute<T: Sort<Self::Item>>(&self, sort: &T) -> Result<Vec<Measurement>, BenchmarkError>;
}