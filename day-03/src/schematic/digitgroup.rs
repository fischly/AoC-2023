#[derive(Debug, Clone)]
pub struct DigitGroup {
    pub digit: u32,
    pub row: u32,
    pub range: std::ops::RangeInclusive<u32>, //(u32, u32),
}
