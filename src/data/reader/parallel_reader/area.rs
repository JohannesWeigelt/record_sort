#[derive(Debug, Copy, Clone)]
pub struct Area {
    start: u64,
    end: u64,
}

impl Area {
    pub fn new(start: u64, end: u64) -> Self {
        Area {
            start,
            end,
        }
    }

    pub fn get_start(&self) -> &u64 {
        &self.start
    }

    pub fn get_end(&self) -> &u64 {
        &self.end
    }
}