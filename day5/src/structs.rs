#[derive(Clone, Debug)]
pub struct ValueRange {
    pub start: i128,
    pub length: i128,
}

impl ValueRange {
    /// This stop value in inclusive.
    pub fn stop (&self) -> i128 {
        self.start + self.length - 1
    }
}

#[derive(Clone)]
pub struct MappedRange {
    pub source_start: i128,
    pub dest_start: i128,
    pub length: i128
}

impl MappedRange {
    pub fn num_in_source_range(&self, s: i128) -> bool {
        s >= self.source_start && s <= self.source_stop()
    }

    pub fn get_mapped_value(&self, s: i128) -> i128 {
        let diff = self.dest_start - self.source_start;
        s + diff
    }

    pub fn source_stop(&self) -> i128{
        self.source_start + self.length - 1
    }
}