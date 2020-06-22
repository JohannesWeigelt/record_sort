use crate::sort::algorithm::Algorithm;
use crate::sort::merge_sort::MergeSort;
use crate::sort::sort::Sort;
use crate::sort::quick_sort::QuickSort;
use crate::sort::insertion_sort::InsertionSort;

pub struct SortFactory;

impl SortFactory {
    pub fn create<T: PartialOrd + Clone>(&self, algorithm: Algorithm) -> Box<dyn Sort<T>> {
        match algorithm {
            Algorithm::Merge => Box::new(MergeSort),
            Algorithm::Quick => Box::new(QuickSort),
            Algorithm::Insertion => Box::new(InsertionSort)
        }
    }
}