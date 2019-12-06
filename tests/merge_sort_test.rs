extern crate record_sort;

use record_sort::sort::merge_sort::MergeSort;
use record_sort::sort::sort::Sort;
use record_sort::util::permutation::HeapPermutaionIterator;

#[test]
fn success() {
    let merge_sort = MergeSort;
    let permutation_iterator = HeapPermutaionIterator::new(10);
    let expected = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for mut permutation in permutation_iterator {
        merge_sort.sort(&mut permutation);
        assert_eq!(expected, permutation)
    }
}