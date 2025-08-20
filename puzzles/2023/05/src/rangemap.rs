use crate::rangemap_section::RangeMapSection;
use aoc_intervals::interval::Interval;
use aoc_intervals::interval_set::IntervalSet;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct RangeMap {
    sections: HashSet<RangeMapSection>,
}

impl RangeMap {
    pub fn new() -> Self {
        Self {
            sections: HashSet::new(),
        }
    }

    pub fn add_section(&mut self, dest: i64, src: i64, length: i64) {
        self.sections
            .insert(RangeMapSection::new(dest, src, length));
    }

    pub fn convert_key(&self, key: i64) -> i64 {
        self.sections
            .iter()
            .find_map(|section| section.convert(key))
            .unwrap_or(key)
    }

    pub fn convert_interval(&self, interval: &Interval<i64>) -> IntervalSet<i64> {
        // If there are no sections, return the interval as-is
        if self.sections.is_empty() {
            let mut result = IntervalSet::new();
            result.add(*interval);
            return result;
        }

        // Collect and sort sections by source interval min
        let mut sections: Vec<_> = self.sections.iter().collect();
        sections.sort_by_key(|s| s.source().get_min());

        // Find the first section whose source min is greater than interval min (upper_bound)
        let mut section_idx = sections
            .iter()
            .position(|s| s.source().get_min() > interval.get_min())
            .unwrap_or(sections.len());
        if section_idx > 0 {
            section_idx -= 1;
        }

        let mut result = IntervalSet::new();
        let mut start = interval.get_min();
        let mut length = interval.count() as i64;
        while length > 0 {
            if section_idx >= sections.len() {
                // No more mappings, add the rest as-is
                result.add(Interval::from_size(start, length));
                break;
            }
            let section = sections[section_idx];
            let map_start = section.source().get_min();
            let map_length = section.source().count() as i64;

            if start < map_start {
                // No conversion for this part
                let actual_length = std::cmp::min(length, map_start - start);
                result.add(Interval::from_size(start, actual_length));
                start += actual_length;
                length -= actual_length;
            } else if (start - map_start) >= map_length {
                // Move to next section
                section_idx += 1;
            } else {
                // Actual conversion
                let max_in_section = section.source().get_max();
                let actual_length = std::cmp::min(
                    Interval::from_boundaries(start, max_in_section).count() as i64,
                    length,
                );
                let dest_start = section.convert(start).unwrap();
                result.add(Interval::from_size(dest_start, actual_length));
                start += actual_length;
                length -= actual_length;
            }
        }
        result
    }
}
