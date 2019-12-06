use crate::sort::sort::Sort;

pub struct MergeSort;

impl MergeSort {
    fn merge<T: PartialOrd + Clone>(&self, input: &mut Vec<T>, start: usize, mid: usize, end: usize) {
        let left = input[start..mid + 1].to_vec();
        let right = input[mid + 1..end + 1].to_vec();

        let mut i = start;
        let mut l = 0;
        let mut r = 0;
        let left_bound = mid - start + 1;
        let right_bound = end - mid;

        while l < left_bound && r < right_bound {
            if left[l] <= right[r] {
                input[i] = left[l].clone();
                l += 1;
            } else {
                input[i] = right[r].clone();
                r += 1;
            }

            i += 1;
        }

        while l < left_bound {
            input[i] = left[l].clone();
            i += 1;
            l += 1;
        }

        while r < right_bound {
            input[i] = right[r].clone();
            i += 1;
            r += 1;
        }
    }

    fn merge_sort<T: PartialOrd + Clone>(&self, input: &mut Vec<T>, start: usize, end: usize) {
        if start < end {
            let mid = (start + end) / 2;
            self.merge_sort(input, start, mid);
            self.merge_sort(input, mid + 1, end);
            self.merge(input, start, mid, end);
        }
    }
}

impl<T: PartialOrd + Clone> Sort<T> for MergeSort {
    fn sort(&self, input: &mut Vec<T>) {
        self.merge_sort(input, 0, input.len() - 1);
    }
}