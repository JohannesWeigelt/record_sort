use std::str::FromStr;

use crate::sort::algorithm::Algorithm;
use crate::sort::merge_sort::MergeSort;
use crate::sort::sort::Sort;

pub struct SortFactory;

impl SortFactory {
    pub fn create<T: PartialOrd + Clone>(&self, algorithm: &String) -> Option<&dyn Sort<T>> {
        let algorithm_optional = Algorithm::from_str(algorithm.as_str());

        match algorithm_optional {
            Ok(algorithm) => {
                match algorithm {
                    Algorithm::Merge => Some(&MergeSort)
                }
            },
            Err(_) => None
        }
    }
}