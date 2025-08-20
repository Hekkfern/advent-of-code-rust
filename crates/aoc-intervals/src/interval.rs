#[cfg(test)]
mod tests;

use crate::interval_value::{IntervalValue, maximum_interval_value, minimum_interval_value};
use num::cast::cast;

/// Represents the location of a value relative to an interval.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    /// The value is to the left of the interval (less than the minimum).
    LeftOutside,
    /// The value is within the interval (between min and max, exclusive).
    Within,
    /// The value is to the right of the interval (greater than the maximum).
    RightOutside,
    /// The value equals the left boundary (minimum) of the interval.
    LeftBoundary,
    /// The value equals the right boundary (maximum) of the interval.
    RightBoundary,
}

/// Represents the name of each of the two boundaries of an interval
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Boundary {
    /// The left boundary of the interval, which is the minimum value.
    Start,
    /// The right boundary of the interval, which is the maximum value.
    End,
}

/// Represents the relationship between two intervals.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Relationship {
    /// One interval completely contains the other, or they are identical.
    Subsumed,
    /// The intervals partially overlap but neither completely contains the other.
    Overlapped,
    /// The intervals do not overlap or touch each other.
    Isolated,
}

/// An interval of continuous integer values where both boundary values are included.
///
/// # Type Parameters
///
/// * `T` - The type of values in the interval.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Interval<T: IntervalValue> {
    /// The minimum (left boundary) value of the interval, inclusive.
    min: T,
    /// The maximum (right boundary) value of the interval, inclusive.
    max: T,
}

impl<T: IntervalValue> Interval<T> {
    /// Creates a new interval from two boundary values.
    ///
    /// The interval will span from the minimum to the maximum of the two provided values,
    /// regardless of their order. Both boundaries are included in the interval.
    ///
    /// # Arguments
    ///
    /// * `boundary1` - The first boundary value
    /// * `boundary2` - The second boundary value
    ///
    /// # Returns
    ///
    /// A new `Interval` spanning from `min(boundary1, boundary2)` to `max(boundary1, boundary2)`
    ///
    /// # Panics
    ///
    /// Panics if either boundary value is outside the valid interval range to prevent overflow in count calculations.
    pub fn from_boundaries(boundary1: T, boundary2: T) -> Self {
        assert!(
            boundary1 >= minimum_interval_value(),
            "boundary1 cannot be less than the minimum interval value to prevent overflow"
        );
        assert!(
            boundary2 >= minimum_interval_value(),
            "boundary2 cannot be less than the minimum interval value to prevent overflow"
        );
        assert!(
            boundary1 <= maximum_interval_value(),
            "boundary1 cannot be greater than the maximum interval value"
        );
        assert!(
            boundary2 <= maximum_interval_value(),
            "boundary2 cannot be greater than the maximum interval value"
        );

        Self {
            min: std::cmp::min(boundary1, boundary2),
            max: std::cmp::max(boundary1, boundary2),
        }
    }

    /// Creates a new interval from a starting position and size.
    ///
    /// The interval will span from `start` to `start + size - 1`, inclusive.
    ///
    /// # Arguments
    ///
    /// * `start` - The starting value of the interval (minimum boundary)
    /// * `size` - The size of the interval (must be positive)
    ///
    /// # Returns
    ///
    /// A new `Interval` spanning from `start` to `start + size - 1`
    ///
    /// # Panics
    ///
    /// Panics if `size` is not positive (greater than zero) or if the start value is outside the valid interval range to prevent overflow in count calculations.
    pub fn from_size(start: T, size: T) -> Self {
        assert!(size > T::zero(), "size must be positive");
        assert!(
            start >= minimum_interval_value(),
            "start cannot be less than the minimum interval value to prevent overflow"
        );
        assert!(
            start <= maximum_interval_value(),
            "start cannot be greater than the maximum interval value"
        );

        // Check for overflow in the addition before subtracting T::one()
        // We need to ensure start + size - 1 doesn't overflow
        // This is equivalent to checking start + size <= T::max_value() + 1
        // Or: start <= T::max_value() + 1 - size
        // But since we can't add 1 to T::max_value(), we check: start <= T::max_value() - size + 1
        let max_allowed_start = match maximum_interval_value::<T>().checked_sub(&(size - T::one()))
        {
            Some(val) => val,
            None => {
                panic!("size is too large for the given type");
            }
        };
        assert!(
            start <= max_allowed_start,
            "start + size would exceed the maximum interval value"
        );

        Self {
            min: start,
            max: start + size - T::one(),
        }
    }

    /// Creates a new interval that spans the entire usable range of the type `T`.
    ///
    /// The interval will span from `T::min_value() + 1` to `T::max_value()`, inclusive.
    /// This excludes the minimum value to prevent overflow in count calculations.
    ///
    /// # Returns
    ///
    /// A new `Interval` spanning almost the complete range of type `T`
    pub fn whole() -> Self {
        Self {
            min: minimum_interval_value(),
            max: maximum_interval_value(),
        }
    }

    /// Returns number of contained items in the interval.
    ///
    /// The size is calculated as `max - min + 1`, representing the number of
    /// integer values contained in the interval.
    ///
    /// # Returns
    ///
    /// The number of values contained in the interval
    pub fn count(&self) -> u64 {
        // Use wider integer arithmetic to prevent overflow
        // Convert to i128 for the calculation, then back to u64
        let min_wide: i128 = cast(self.min).unwrap_or(0);
        let max_wide: i128 = cast(self.max).unwrap_or(0);
        let size_wide = max_wide - min_wide + 1;

        // Clamp to u64::MAX if the result is too large
        if size_wide > (u64::MAX as i128) {
            u64::MAX
        } else {
            size_wide as u64
        }
    }

    /// Returns the minimum (left boundary) value of the interval.
    ///
    /// # Returns
    ///
    /// The minimum value in the interval
    pub fn get_min(&self) -> T {
        self.min
    }

    /// Returns the maximum (right boundary) value of the interval.
    ///
    /// # Returns
    ///
    /// The maximum value in the interval
    pub fn get_max(&self) -> T {
        self.max
    }

    /// Returns both boundary values as a tuple.
    ///
    /// # Returns
    ///
    /// A tuple `(min, max)` containing the minimum and maximum values
    pub fn get_boundaries(&self) -> (T, T) {
        (self.min, self.max)
    }

    /// Checks if a value is contained within the interval.
    ///
    /// Returns `true` if the value is between the minimum and maximum boundaries,
    /// inclusive of both boundaries.
    ///
    /// # Arguments
    ///
    /// * `val` - The value to check
    ///
    /// # Returns
    ///
    /// `true` if `min <= val <= max`, `false` otherwise
    pub fn contains(&self, val: T) -> bool {
        val >= self.min && val <= self.max
    }

    /// Checks if the interval contains exactly one value.
    ///
    /// Returns `true` if the minimum and maximum boundaries are equal.
    ///
    /// # Returns
    ///
    /// `true` if the interval contains exactly one value, `false` otherwise
    pub fn has_one_value(&self) -> bool {
        self.min == self.max
    }

    /// Checks if this interval is completely contained within another interval.
    ///
    /// Returns `true` if this interval's boundaries are within or equal to
    /// the other interval's boundaries.
    ///
    /// # Arguments
    ///
    /// * `other` - The interval to check against
    ///
    /// # Returns
    ///
    /// `true` if this interval is subsumed by the other interval, `false` otherwise
    pub fn subsumes(&self, other: &Self) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    /// Checks if this interval overlaps with another interval.
    ///
    /// Returns `true` if the intervals share any common values, including
    /// touching at boundaries.
    ///
    /// # Arguments
    ///
    /// * `other` - The interval to check for overlap
    ///
    /// # Returns
    ///
    /// `true` if the intervals overlap or touch, `false` otherwise
    pub fn overlaps(&self, other: &Self) -> bool {
        (other.min >= self.min && other.min <= self.max)
            || (other.max >= self.min && other.max <= self.max)
            || other.subsumes(self)
            || self.subsumes(other)
    }

    /// Determines the relationship between this interval and another interval.
    ///
    /// Returns one of three possible relationships: Subsumed (one completely contains the other),
    /// Overlapped (they partially overlap), or Isolated (they don't touch or overlap).
    ///
    /// # Arguments
    ///
    /// * `other` - The interval to compare with
    ///
    /// # Returns
    ///
    /// The `Relationship` between the two intervals
    pub fn get_relationship_with(&self, other: &Self) -> Relationship {
        if self.subsumes(other) || other.subsumes(self) {
            Relationship::Subsumed
        } else if self.max < other.min || other.max < self.min {
            Relationship::Isolated
        } else {
            Relationship::Overlapped
        }
    }

    /// Joins this interval with another interval if they overlap or are contiguous.
    ///
    /// Returns a new interval that spans from the minimum of both intervals to the
    /// maximum of both intervals, but only if they overlap or touch each other.
    ///
    /// # Arguments
    ///
    /// * `other` - The interval to join with
    ///
    /// # Returns
    ///
    /// `Some(Interval)` containing the union if the intervals can be joined,
    /// `None` if they are separate and cannot be joined
    pub fn join(&self, other: &Self) -> Option<Self> {
        if self.overlaps(other) || self.is_contiguous_to(other) {
            Some(Interval {
                min: std::cmp::min(self.min, other.min),
                max: std::cmp::max(self.max, other.max),
            })
        } else {
            None
        }
    }

    /// Finds the intersection of this interval with another interval.
    ///
    /// Returns a new interval containing only the values that are present in both
    /// intervals, or `None` if the intervals don't overlap.
    ///
    /// # Arguments
    ///
    /// * `other` - The interval to intersect with
    ///
    /// # Returns
    ///
    /// `Some(Interval)` containing the intersection if the intervals overlap,
    /// `None` if they don't overlap
    pub fn intersect(&self, other: &Self) -> Option<Self> {
        if other.min >= self.min && other.min <= self.max && other.max >= self.max {
            Some(Interval {
                min: other.min,
                max: self.max,
            })
        } else if other.min <= self.min && other.max >= self.min && other.max <= self.max {
            return Some(Interval {
                min: self.min,
                max: other.max,
            });
        } else if other.subsumes(self) {
            return Some(*self);
        } else if self.subsumes(other) {
            return Some(*other);
        } else {
            None
        }
    }

    /// Computes the difference between this interval and another interval.
    ///
    /// Returns a vector of intervals representing the values that are in either
    /// interval but not in both. The result can contain 0, 1, or 2 intervals
    /// depending on how the intervals relate to each other.
    ///
    /// # Arguments
    ///
    /// * `other` - The interval to compute the difference with
    ///
    /// # Returns
    ///
    /// A `Vec<Interval>` containing the difference intervals, sorted by their minimum values
    pub fn difference(&self, other: &Self) -> Vec<Self> {
        let mut results = Vec::new();
        if self.min < other.min && other.min <= self.max && self.max < other.max {
            results.push(Interval {
                min: self.min,
                max: other.min - T::one(),
            });
            results.push(Interval {
                min: self.max + T::one(),
                max: other.max,
            });
        } else if other.min < self.min && self.min <= other.max && other.max < self.max {
            results.push(Interval {
                min: other.min,
                max: self.min - T::one(),
            });
            results.push(Interval {
                min: other.max + T::one(),
                max: self.max,
            });
        } else if self.min == other.min {
            if self.max < other.max {
                results.push(Interval {
                    min: self.max + T::one(),
                    max: other.max,
                });
            } else if other.max < self.max {
                results.push(Interval {
                    min: other.max + T::one(),
                    max: self.max,
                });
            }
        } else if self.max == other.max {
            if self.min < other.min {
                results.push(Interval {
                    min: self.min,
                    max: other.min - T::one(),
                });
            } else if other.min < self.min {
                results.push(Interval {
                    min: other.min,
                    max: self.min - T::one(),
                });
            }
        } else if other.subsumes(self) {
            results.push(Interval {
                min: other.min,
                max: self.min - T::one(),
            });
            results.push(Interval {
                min: self.max + T::one(),
                max: other.max,
            });
        } else if self.subsumes(other) {
            results.push(Interval {
                min: self.min,
                max: other.min - T::one(),
            });
            results.push(Interval {
                min: other.max + T::one(),
                max: self.max,
            });
        } else {
            results.push(*self);
            results.push(*other);
        }
        results.sort_by(|a, b| a.min.cmp(&b.min));
        results
    }

    /// Checks if this interval is contiguous (adjacent) to another interval.
    ///
    /// Returns `true` if the intervals touch each other but don't overlap,
    /// meaning one interval's maximum value plus one equals the other's minimum value.
    ///
    /// # Arguments
    ///
    /// * `other` - The interval to check contiguity with
    ///
    /// # Returns
    ///
    /// `true` if the intervals are adjacent, `false` otherwise
    pub fn is_contiguous_to(&self, other: &Self) -> bool {
        self.max + T::one() == other.min || other.max + T::one() == self.min
    }

    /// Determines the location of a value relative to this interval.
    ///
    /// Returns a `Location` enum indicating whether the value is outside the interval
    /// (left or right), on a boundary, or within the interval.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to locate
    ///
    /// # Returns
    ///
    /// A `Location` indicating the position of the value relative to the interval
    pub fn get_location(&self, value: T) -> Location {
        if value < self.min {
            Location::LeftOutside
        } else if value > self.max {
            Location::RightOutside
        } else if value == self.min {
            Location::LeftBoundary
        } else if value == self.max {
            Location::RightBoundary
        } else {
            Location::Within
        }
    }

    /// Creates a new interval by shifting this interval by a given offset.
    ///
    /// Both the minimum and maximum values are moved by the same offset amount.
    /// The size of the interval remains unchanged.
    ///
    /// # Arguments
    ///
    /// * `offset` - The amount to shift the interval (can be positive or negative)
    ///
    /// # Returns
    ///
    /// A new `Interval` shifted by the specified offset
    pub fn shift(&self, offset: i32) -> Self {
        let offset: T = cast(offset).expect("Failed to convert offset to type T");
        let new_min = self.min + offset;
        let new_max = self.max + offset;
        assert!(
            new_min >= minimum_interval_value() && new_min <= maximum_interval_value(),
            "Shifted minimum value is out of bounds"
        );
        assert!(
            new_max >= minimum_interval_value() && new_max <= maximum_interval_value(),
            "Shifted maximum value is out of bounds"
        );
        Interval {
            min: new_min,
            max: new_max,
        }
    }

    /// Creates a new interval by expanding this interval equally on both sides.
    ///
    /// The minimum value is decreased by the offset and the maximum value is
    /// increased by the offset, effectively growing the interval by twice the offset.
    ///
    /// # Arguments
    ///
    /// * `offset` - The amount to resize. If positive, the interval expands; if negative, it contracts.
    ///
    /// # Returns
    ///
    /// A new `Interval` expanded by the specified offset on both sides
    pub fn expand_equally(&self, offset: i32) -> Self {
        let offset: T = cast(offset).expect("Failed to convert offset to type T");
        let new_min = self.min - offset;
        let new_max = self.max + offset;
        assert!(new_min <= new_max);
        assert!(
            new_min >= minimum_interval_value() && new_min <= maximum_interval_value(),
            "Shifted minimum value is out of bounds"
        );
        assert!(
            new_max >= minimum_interval_value() && new_max <= maximum_interval_value(),
            "Shifted maximum value is out of bounds"
        );
        Interval {
            min: new_min,
            max: new_max,
        }
    }

    /// Creates a new interval by expanding this interval with different offsets for each side.
    ///
    /// The minimum value is decreased by the left offset and the maximum value is
    /// increased by the right offset, allowing asymmetric expansion.
    ///
    /// # Arguments
    ///
    /// * `left_offset` - The amount to expand on the left (minimum) side. If positive, the interval expands; if negative, it contracts.
    /// * `right_offset` - The amount to expand on the right (maximum) side. If positive, the interval expands; if negative, it contracts.
    ///
    /// # Returns
    ///
    /// A new `Interval` expanded by the specified offsets
    pub fn expand(&self, left_offset: i32, right_offset: i32) -> Self {
        let left_offset: T = cast(left_offset).expect("Failed to convert left offset to type T");
        let right_offset: T = cast(right_offset).expect("Failed to convert right offset to type T");
        let new_min = self.min - left_offset;
        let new_max = self.max + right_offset;
        assert!(new_min <= new_max);
        assert!(
            new_min >= minimum_interval_value() && new_min <= maximum_interval_value(),
            "Shifted minimum value is out of bounds"
        );
        assert!(
            new_max >= minimum_interval_value() && new_max <= maximum_interval_value(),
            "Shifted maximum value is out of bounds"
        );
        Interval {
            min: new_min,
            max: new_max,
        }
    }

    /// Calculates the relative position of a given value with respect to a specified boundary.
    ///
    /// The boundary can be either `Boundary::Start` or `Boundary::End`, corresponding to
    /// the lower (`self.min`) and upper (`self.max`) bounds respectively. This method computes
    /// the difference between the provided `value` and the selected boundary.
    ///
    /// # Arguments
    ///
    /// * `boundary` - The boundary to compare against (`Start` or `End`).
    /// * `value` - The value whose position is to be evaluated relative to the boundary.
    ///
    /// # Returns
    ///
    /// A value of the same type representing the relative position. A positive result
    /// indicates that the `value` is greater than the boundary, while a negative result
    /// means it is smaller.
    pub fn get_relative_position_from(&self, boundary: Boundary, value: T) -> T {
        match boundary {
            Boundary::Start => value - self.min,
            Boundary::End => value - self.max,
        }
    }
}

impl<T: IntervalValue> std::fmt::Display for Interval<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.min, self.max)
    }
}

/// Left shift operator implementation for Interval.
///
/// Shifts both the minimum and maximum values to the left (decreases them)
/// by the specified amount. The size of the interval remains unchanged.
impl<T: IntervalValue> std::ops::Shl<i32> for Interval<T> {
    type Output = Self;

    fn shl(self, rhs: i32) -> Self::Output {
        self.shift(-rhs)
    }
}

/// Right shift operator implementation for Interval.
///
/// Shifts both the minimum and maximum values to the right (increases them)
/// by the specified amount. The size of the interval remains unchanged.
impl<T: IntervalValue> std::ops::Shr<i32> for Interval<T> {
    type Output = Self;

    fn shr(self, rhs: i32) -> Self::Output {
        self.shift(rhs)
    }
}
