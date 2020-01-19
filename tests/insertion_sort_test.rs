use record_sort::sort::insertion_sort::InsertionSort;
use record_sort::util::permutation::HeapPermutaionIterator;
use record_sort::sort::sort::Sort;

#[test]
fn success() {
    let sort = InsertionSort;
    let permutation_iterator = HeapPermutaionIterator::new(10);
    let expected = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for mut permutation in permutation_iterator {
        sort.sort(&mut permutation);
        assert_eq!(expected, permutation)
    }
}