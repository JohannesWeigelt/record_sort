use crate::sort::sort::Sort;

pub struct InsertionSort;

impl <T: PartialOrd> Sort<T> for InsertionSort {
    fn sort(&self, input: &mut Vec<T>) {
        let arr = input.as_mut_slice();

        for i in 1..arr.len() {
            let mut j = i;
            while j > 0 && arr[j] < arr[j-1] {
                arr.swap(j, j-1);
                j = j-1;
            }
        }
    }
}