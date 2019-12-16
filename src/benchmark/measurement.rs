#[derive(Debug)]
pub struct Measurement {
    sorted_elements: usize,
    duration: f64,
}

impl Measurement {
    pub fn new(sorted_elements: usize, duration: f64) -> Self {
        Measurement { sorted_elements, duration }
    }

    pub fn get_sorted_elements(&self) -> usize {
        self.sorted_elements
    }

    pub fn get_duration(&self) -> f64 {
        self.duration
    }
}