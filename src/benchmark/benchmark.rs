use crate::benchmark::benchmark_error::BenchmarkError;
use crate::benchmark::measurement::Measurement;
use crate::sort::sort::Sort;

/// Tests the speed of the implementation of a given sorting-algorithm.
pub trait Benchmark {
    type Item: PartialOrd;

    fn execute(&self, sort: &dyn Sort<Self::Item>) -> Result<Vec<Measurement>, BenchmarkError>;
}