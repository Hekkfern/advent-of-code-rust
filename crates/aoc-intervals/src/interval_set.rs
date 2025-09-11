#[cfg(test)]
mod tests;

use crate::interval::Interval;
use crate::interval_value::{IntervalValue, maximum_interval_value, minimum_interval_value};
use std::collections::HashSet;

/// Represents a set of non-overlapping intervals.
///
/// This struct provides operations to add and remove intervals or individual values,
/// automatically merging overlapping intervals to maintain a set of non-overlapping intervals.
///
/// # Type Parameters
///
/// * `T` - The type of the interval boundaries, must implement the required traits for integer operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntervalSet<T: IntervalValue> {
    /// Stores intervals in a hash set, maintaining no overlaps through merging.
    intervals: HashSet<Interval<T>>,
}

impl<T: IntervalValue> IntervalSet<T> {
    /// Default constructor. Initializes an empty `IntervalSet.
    pub fn new() -> Self {
        Self {
            intervals: HashSet::new(),
        }
    }

    /// Constructs an `IntervalSet` from a vector of `Interval`.
    /// Overlapping intervals are merged.
    ///
    /// # Arguments
    ///
    /// * `intervals` - A vector of `Interval` to initialize the `IntervalSet`.
    pub fn from_vec(intervals: Vec<Interval<T>>) -> Self {
        let mut interval_set = Self {
            intervals: intervals.into_iter().collect(),
        };
        interval_set.reduce();
        interval_set
    }

    /// Gets a copy of the intervals, as a vector.
    ///
    /// # Returns
    ///
    /// A vector containing all intervals in the set.
    pub fn get(&self) -> Vec<Interval<T>> {
        let mut intervals: Vec<_> = self.intervals.iter().copied().collect();
        intervals.sort();
        intervals
    }

    /// Adds an interval, merging it with any overlapping intervals.
    ///
    /// # Arguments
    ///
    /// * `interval` - The interval to add.
    pub fn add_interval(&mut self, interval: Interval<T>) {
        self.intervals.insert(interval);
        self.reduce();
    }

    /// Adds a new interval defined by a single value.
    ///
    /// This is equivalent to adding an interval where both boundaries are this value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to add as a single-value interval.
    pub fn add_value(&mut self, value: T) {
        self.add_interval(Interval::from_boundaries(value, value));
    }

    /// Removes a specific value, splitting intervals as necessary.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to remove.
    pub fn remove_value(&mut self, value: T) {
        let mut temp_intervals = HashSet::new();

        for item in &self.intervals {
            if item.contains(value) {
                if !item.has_one_value() {
                    if item.get_min() == value {
                        temp_intervals
                            .insert(Interval::from_boundaries(value + T::one(), item.get_max()));
                    } else if item.get_max() == value {
                        temp_intervals
                            .insert(Interval::from_boundaries(item.get_min(), value - T::one()));
                    } else {
                        temp_intervals
                            .insert(Interval::from_boundaries(item.get_min(), value - T::one()));
                        temp_intervals
                            .insert(Interval::from_boundaries(value + T::one(), item.get_max()));
                    }
                }
            } else {
                temp_intervals.insert(*item);
            }
        }

        self.intervals = temp_intervals;
        self.reduce();
    }

    /// Removes an interval, adjusting or splitting intervals as needed.
    ///
    /// # Arguments
    ///
    /// * `erase_interval` - The interval to remove.
    pub fn remove_interval(&mut self, erase_interval: &Interval<T>) {
        let mut temp_intervals = HashSet::new();

        for inner_interval in &self.intervals {
            if inner_interval == erase_interval {
                continue;
            }
            if inner_interval.subsumes(erase_interval) {
                // this interval is split in half because of the erased value
                temp_intervals.insert(Interval::from_boundaries(
                    inner_interval.get_min(),
                    erase_interval.get_min() - T::one(),
                ));
                temp_intervals.insert(Interval::from_boundaries(
                    erase_interval.get_max() + T::one(),
                    inner_interval.get_max(),
                ));
            } else if erase_interval.get_min() >= inner_interval.get_min()
                && erase_interval.get_min() <= inner_interval.get_max()
            {
                // this interval is not totally subsumed, so it is partially deleted
                temp_intervals.insert(Interval::from_boundaries(
                    inner_interval.get_min(),
                    erase_interval.get_min() - T::one(),
                ));
            } else if erase_interval.get_max() >= inner_interval.get_min()
                && erase_interval.get_max() <= inner_interval.get_max()
            {
                // this interval is not totally subsumed so it is partially deleted
                temp_intervals.insert(Interval::from_boundaries(
                    erase_interval.get_max() + T::one(),
                    inner_interval.get_max(),
                ));
            } else if !erase_interval.subsumes(inner_interval) {
                temp_intervals.insert(*inner_interval);
            }
        }

        self.intervals = temp_intervals;
        self.reduce();
    }

    /// Merges two interval sets.
    ///
    /// # Arguments
    ///
    /// * `other` - The other interval set.
    ///
    /// # Returns
    ///
    /// The result of merging both interval sets.
    pub fn join(&self, other: &Self) -> Self {
        let mut joined_intervals = self.intervals.clone();
        for item in &other.intervals {
            joined_intervals.insert(*item);
        }
        let mut result = Self {
            intervals: joined_intervals,
        };
        result.reduce();
        result
    }

    /// Checks if another interval set is completely included in this one.
    ///
    /// # Arguments
    ///
    /// * `other` - The other interval set.
    ///
    /// # Returns
    ///
    /// `true` if the other interval set is subsumed by this one, `false` otherwise.
    pub fn subsumes_set(&self, other: &Self) -> bool {
        other
            .intervals
            .iter()
            .all(|other_interval| self.subsumes_interval(other_interval))
    }

    /// Checks if an interval is completely included in this interval set.
    ///
    /// # Arguments
    ///
    /// * `other` - The other interval.
    ///
    /// # Returns
    ///
    /// `true` if the other interval is subsumed by this interval set, `false` otherwise.
    pub fn subsumes_interval(&self, other: &Interval<T>) -> bool {
        self.intervals
            .iter()
            .any(|interval| interval.subsumes(other))
    }

    /// Checks if both interval sets overlap partially or totally.
    ///
    /// # Arguments
    ///
    /// * `other` - The other interval set.
    ///
    /// # Returns
    ///
    /// `true` if they overlap in any way, `false` otherwise.
    pub fn overlaps_set(&self, other: &Self) -> bool {
        other
            .intervals
            .iter()
            .any(|other_interval| self.overlaps_interval(other_interval))
    }

    /// Checks if an interval overlaps with any interval in this set.
    ///
    /// # Arguments
    ///
    /// * `other` - The other interval.
    ///
    /// # Returns
    ///
    /// `true` if they overlap in any way, `false` otherwise.
    pub fn overlaps_interval(&self, other: &Interval<T>) -> bool {
        self.intervals
            .iter()
            .any(|interval| interval.overlaps(other))
    }

    /// Checks if a specific value is contained in the interval set.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to check.
    ///
    /// # Returns
    ///
    /// `true` if the interval set contains the value, `false` otherwise.
    pub fn contains(&self, value: T) -> bool {
        self.intervals
            .iter()
            .any(|interval| interval.contains(value))
    }

    /// Gets the number of contained items.
    ///
    /// # Returns
    ///
    /// Total number of items.
    pub fn count(&self) -> u64 {
        self.intervals.iter().map(Interval::count).sum()
    }

    /// Extracts a sub `IntervalSet` contained between the selected values.
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum value of the range to extract from.
    /// * `max` - The maximum value of the range to extract from.
    ///
    /// # Returns
    ///
    /// A sub interval set.
    pub fn extract(&self, min: T, max: T) -> Self {
        let mut result_interval = self.clone();
        result_interval.remove_interval(&Interval::from_boundaries(
            minimum_interval_value(),
            min - T::one(),
        ));
        result_interval.remove_interval(&Interval::from_boundaries(
            max + T::one(),
            maximum_interval_value(),
        ));
        result_interval
    }

    /// Gets the sub Interval that contains the selected value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value.
    ///
    /// # Returns
    ///
    /// The interval containing this value, or None.
    pub fn get_interval_for(&self, value: T) -> Option<Interval<T>> {
        self.intervals
            .iter()
            .find(|interval| interval.contains(value))
            .copied()
    }

    /// Calculates the overlapping parts of this interval set with the provided interval.
    ///
    /// # Arguments
    ///
    /// * `other` - The other interval.
    ///
    /// # Returns
    ///
    /// A new `IntervalSet` with the overlapped fragment.
    pub fn intersect(&self, other: &Interval<T>) -> Self {
        let mut overlapped_intervals = Vec::new();
        for internal_interval in &self.intervals {
            if let Some(result) = internal_interval.intersect(other) {
                overlapped_intervals.push(result);
            }
        }
        Self::from_vec(overlapped_intervals)
    }

    /// Checks if there is any interval.
    ///
    /// # Returns
    ///
    /// `true` if the interval set is empty, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.intervals.is_empty()
    }

    /// Returns an iterator over the intervals.
    pub fn iter(&self) -> impl Iterator<Item = &Interval<T>> {
        self.intervals.iter()
    }

    /// Merges overlapping intervals within the set to maintain the non-overlapping invariant.
    fn reduce(&mut self) {
        if self.intervals.is_empty() {
            return;
        }

        let mut intervals: Vec<_> = self.intervals.iter().copied().collect();
        intervals.sort_by_key(Interval::get_min);

        let mut merged_intervals = Vec::new();
        let mut current_interval = intervals[0];

        for &interval in intervals.iter().skip(1) {
            if current_interval.get_max() >= interval.get_min() - T::one() {
                current_interval = Interval::from_boundaries(
                    current_interval.get_min(),
                    std::cmp::max(current_interval.get_max(), interval.get_max()),
                );
            } else {
                merged_intervals.push(current_interval);
                current_interval = interval;
            }
        }
        merged_intervals.push(current_interval);

        self.intervals = merged_intervals.into_iter().collect();
    }
}

impl<T: IntervalValue> Default for IntervalSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: IntervalValue> std::fmt::Display for IntervalSet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.intervals.is_empty() {
            return write!(f, "");
        }

        let mut intervals: Vec<_> = self.intervals.iter().collect();
        intervals.sort_by_key(|a| a.get_min());

        let intervals_str: Vec<String> = intervals
            .iter()
            .map(|interval| format!("{interval}"))
            .collect();

        write!(f, "{}", intervals_str.join(","))
    }
}
