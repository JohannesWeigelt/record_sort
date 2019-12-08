use std::time::Duration;

use crate::sort::sort::Sort;

/// Tests the speed of the implementation of a given sorting-algorithm.
pub trait Benchmark {
    type Item: PartialOrd;

    fn execute(&self, sort: &dyn Sort<Self::Item>) -> Duration;
}