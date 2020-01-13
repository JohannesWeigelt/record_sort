use crate::sort::sort::Sort;

pub struct QuickSort;

impl QuickSort {
    fn quick_sort<T: PartialOrd + Clone>(&self, input: &mut Vec<T>, left: usize, right: usize) {
        if left < right {
            let s = self.split(input, left, right);
            self.quick_sort(input, left, s - 1);
            self.quick_sort(input, s + 1, right);
        }
    }

    fn split<T: PartialOrd + Clone>(&self, input: &mut Vec<T>, left: usize, right: usize) -> usize {
        let mut i = left;
        let mut j = right - 1;
        let pivot = &mut input[right].clone();

        while {
            while i < right && &mut input[i] < pivot {
                i += 1;
            }

            while j > left && &mut input[j] >= pivot {
                j -= 1;
            }

            if i < j {
                input.swap(i, j);
            }

            i < j
        } {}

        input.swap(i, right);
        i
    }
}

impl<T: PartialOrd + Clone> Sort<T> for QuickSort {
    fn sort(&self, input: &mut Vec<T>) {
        self.quick_sort(input, 0, input.len() - 1)
    }
}