#[derive(Clone, Copy)]
pub struct Range {
    pub start: u32,
    pub end: u32,
}

impl Range {

    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn includes(&self, other: Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn intersects(&self, other: Range) -> bool {
        (self.start >= other.start && self.start <= other.end) || 
        (self.end >= other.start && self.end <= other.end)
    }

}