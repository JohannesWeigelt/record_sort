pub struct HeapPermutaionIterator {
    n: usize,
    permutation: Vec<u8>,
    c: Vec<usize>,
    i: usize,
    first: bool
}

impl HeapPermutaionIterator {
    pub fn new(n: usize) -> Self {
        let mut c = Vec::new();
        for _ in 0..n {
            c.push(0);
        }

        let mut permutation = Vec::new();
        for k in 1u8..(n + 1) as u8 {
            permutation.push(k);
        }

        HeapPermutaionIterator {
            n,
            permutation,
            c,
            i: 0,
            first: true
        }
    }
}

impl Iterator for HeapPermutaionIterator {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(self.permutation.to_vec());
        }

        while self.i < self.n {
            if self.c[self.i] < self.i {
                if self.i % 2 == 0 {
                    self.permutation.swap(0, self.i);
                } else {
                    self.permutation.swap(self.c[self.i], self.i);
                }

                self.c[self.i] += 1;
                self.i = 0;

                return Some(self.permutation.to_vec());
            } else {
                self.c[self.i] = 0;
                self.i += 1;
            }
        }

        None
    }
}