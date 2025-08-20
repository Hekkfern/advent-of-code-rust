use aoc_intervals::interval::Interval;

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
pub struct RangeMapSection {
    destination: Interval<i64>,
    source: Interval<i64>,
}

impl RangeMapSection {
    pub fn new(dest: i64, src: i64, length: i64) -> Self {
        Self {
            destination: Interval::from_size(dest, length),
            source: Interval::from_size(src, length),
        }
    }

    #[allow(unused)]
    pub fn destination(&self) -> &Interval<i64> {
        &self.destination
    }

    #[allow(unused)]
    pub fn source(&self) -> &Interval<i64> {
        &self.source
    }

    pub fn convert(&self, key: i64) -> Option<i64> {
        if !self.source.contains(key) {
            return None;
        }
        Some(self.destination.get_min() + (key - self.source.get_min()))
    }
}
