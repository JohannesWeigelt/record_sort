use rand::Rng;

/// Generates vectors of random numbers
pub struct RandomNumberGenerator;

impl RandomNumberGenerator {
    pub fn generate_u8_numbers(&self, size: usize) -> Vec<u8> {
        let mut result = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..size {
            result.push(rng.gen());
        }

        result
    }
}