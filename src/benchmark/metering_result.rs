use std::time::Duration;

#[derive(Debug)]
pub struct MeteringResult {
    sorted_elements: usize,
    duration: f64,
}

impl MeteringResult {
    pub fn new(sorted_elements: usize, duration: f64) -> Self {
        MeteringResult { sorted_elements, duration }
    }

    pub fn get_sorted_elements(&self) -> usize {
        self.sorted_elements
    }

    pub fn get_duration(&self) -> f64 {
        self.duration
    }
}