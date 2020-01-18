use crate::sort::sort::Sort;

///Implementation of a quicksort-algorithm.
///
/// Heavy inspired by https://rosettacode.org/wiki/Sorting_algorithms/Quicksort#Rust
pub struct QuickSort;

impl QuickSort {
    fn quick_sort<T: PartialOrd>(&self, v: &mut [T]) {
        let len = v.len();
        if len >= 2 {
            let pivot_index = self.partition(v);
            self.quick_sort(&mut v[0..pivot_index]);
            self.quick_sort(&mut v[pivot_index + 1..len]);
        }
    }

    fn partition<T: PartialOrd>(&self, v: &mut [T]) -> usize {
        let len = v.len();
        let pivot_index = len / 2;
        let last_index = len - 1;

        v.swap(pivot_index, last_index);

        let mut store_index = 0;
        for i in 0..last_index {
            if &v[i] < &v[last_index] {
                v.swap(i, store_index);
                store_index += 1;
            }
        }

        v.swap(store_index, len - 1);
        store_index
    }
}

impl<T: PartialOrd> Sort<T> for QuickSort {
    fn sort(&self, input: &mut Vec<T>) {
        self.quick_sort(input.as_mut_slice())
    }
}