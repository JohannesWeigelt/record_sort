extern crate record_sort;

use record_sort::util::fac::fac;
use record_sort::util::permutation::HeapPermutaionIterator;

#[test]
fn success() {
    let n: u64 = 3;
    let all_permutations = vec![
        vec![1u8,2,3],
        vec![1,3,2],
        vec![2,1,3],
        vec![2,3,1],
        vec![3,2,1],
        vec![3,1,2]
    ];
    let permutations = fac(n);
    let it = HeapPermutaionIterator::new(n as usize);

    let mut i = 1;
    for permutation in it {
        assert!(all_permutations.contains(&permutation));
        assert!(i <= permutations);
        i += 1;
    }
}