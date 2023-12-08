use std::ops::Range;

#[derive(Debug)]
pub struct Map {
    pub source_range: Range<u64>,
    pub dest_range: Range<u64>,
}

impl Map {
    pub fn contains_seed(&self, seed: u64) -> bool {
        self.source_range.contains(&seed)
    }

    pub fn map(&self, seed: u64) -> u64 {
        self.dest_range.end - (self.source_range.end - seed)
    }
}
