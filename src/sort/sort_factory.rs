use crate::sort::algorithm::Algorithm;
use crate::sort::merge_sort::MergeSort;
use crate::sort::sort::Sort;
use crate::sort::quick_sort::QuickSort;

pub struct SortFactory;

impl SortFactory {
    pub fn create<T: PartialOrd + Clone>(&self, algorithm: Algorithm) -> &dyn Sort<T> {
        match algorithm {
            Algorithm::Merge => &MergeSort,
            Algorithm::Quick => &QuickSort
        }
    }
}