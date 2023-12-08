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

struct ElfMap {
    source: String,
    dest: String,
    mapped_ranges: Vec<MappedRange>
}

impl ElfMap {
    fn new(source: &str, dest: &str) -> Self {
        Self {
            source: source.to_owned(),
            dest: dest.to_owned(),
            mapped_ranges: Vec::new()
        }
    }
}