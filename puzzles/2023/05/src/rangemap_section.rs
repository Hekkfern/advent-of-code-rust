use aoc_intervals::interval::Interval;

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Hash, Clone)]
pub struct RangeMapSection {
    source: Interval<i64>,
    destination: Interval<i64>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use assertables::{assert_none, assert_some};

    #[test]
    fn convert_some() {
        let section = RangeMapSection::new(50, 98, 2);
        let result = section.convert(99);
        assert_some!(result);
        assert_eq!(result, Some(51));
    }

    #[test]
    fn convert_none() {
        let section = RangeMapSection::new(50, 98, 2);
        let result = section.convert(40);
        assert_none!(result);
    }
}
