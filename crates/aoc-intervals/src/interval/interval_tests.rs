use super::*;
use assertables::{assert_none, assert_some};
use pretty_assertions::{assert_eq, assert_ne};

// Tests for from_boundaries

#[test]
fn from_boundaries_positive_values_ordered() {
    let interval = Interval::from_boundaries(5, 10);
    assert_eq!(interval.min, 5);
    assert_eq!(interval.max, 10);
    assert_eq!(interval.count(), 6);
    assert_eq!(interval.get_min(), 5);
    assert_eq!(interval.get_max(), 10);
    assert_eq!(interval.get_boundaries(), (5, 10));
}

#[test]
fn from_boundaries_positive_values_reversed() {
    let interval = Interval::from_boundaries(10, 5);
    assert_eq!(interval.min, 5);
    assert_eq!(interval.max, 10);
    assert_eq!(interval.count(), 6);
    assert_eq!(interval.get_min(), 5);
    assert_eq!(interval.get_max(), 10);
    assert_eq!(interval.get_boundaries(), (5, 10));
}

#[test]
fn from_boundaries_negative_left_positive_right() {
    let interval = Interval::from_boundaries(-5, 10);
    assert_eq!(interval.min, -5);
    assert_eq!(interval.max, 10);
    assert_eq!(interval.count(), 16);
    assert_eq!(interval.get_min(), -5);
    assert_eq!(interval.get_max(), 10);
    assert_eq!(interval.get_boundaries(), (-5, 10));
}

#[test]
fn from_boundaries_positive_left_negative_right() {
    let interval = Interval::from_boundaries(5, -10);
    assert_eq!(interval.min, -10);
    assert_eq!(interval.max, 5);
    assert_eq!(interval.count(), 16);
    assert_eq!(interval.get_min(), -10);
    assert_eq!(interval.get_max(), 5);
    assert_eq!(interval.get_boundaries(), (-10, 5));
}

#[test]
fn from_boundaries_both_negative() {
    let interval = Interval::from_boundaries(-10, -5);
    assert_eq!(interval.min, -10);
    assert_eq!(interval.max, -5);
    assert_eq!(interval.count(), 6);
    assert_eq!(interval.get_min(), -10);
    assert_eq!(interval.get_max(), -5);
    assert_eq!(interval.get_boundaries(), (-10, -5));
}

#[test]
fn from_boundaries_both_negative_reversed() {
    let interval = Interval::from_boundaries(-5, -10);
    assert_eq!(interval.min, -10);
    assert_eq!(interval.max, -5);
    assert_eq!(interval.count(), 6);
    assert_eq!(interval.get_min(), -10);
    assert_eq!(interval.get_max(), -5);
    assert_eq!(interval.get_boundaries(), (-10, -5));
}

#[test]
fn from_boundaries_equal_values() {
    let interval = Interval::from_boundaries(7, 7);
    assert_eq!(interval.min, 7);
    assert_eq!(interval.max, 7);
    assert_eq!(interval.count(), 1);
    assert!(interval.has_one_value());
    assert_eq!(interval.get_min(), 7);
    assert_eq!(interval.get_max(), 7);
    assert_eq!(interval.get_boundaries(), (7, 7));
}

#[test]
fn from_boundaries_zero_boundaries() {
    let interval = Interval::from_boundaries(0, 0);
    assert_eq!(interval.min, 0);
    assert_eq!(interval.max, 0);
    assert_eq!(interval.count(), 1);
    assert!(interval.has_one_value());
    assert_eq!(interval.get_min(), 0);
    assert_eq!(interval.get_max(), 0);
    assert_eq!(interval.get_boundaries(), (0, 0));
}

#[test]
fn from_boundaries_zero_and_positive() {
    let interval = Interval::from_boundaries(0, 5);
    assert_eq!(interval.min, 0);
    assert_eq!(interval.max, 5);
    assert_eq!(interval.count(), 6);
    assert_eq!(interval.get_min(), 0);
    assert_eq!(interval.get_max(), 5);
    assert_eq!(interval.get_boundaries(), (0, 5));
}

#[test]
fn from_boundaries_zero_and_negative() {
    let interval = Interval::from_boundaries(0, -5);
    assert_eq!(interval.min, -5);
    assert_eq!(interval.max, 0);
    assert_eq!(interval.count(), 6);
    assert_eq!(interval.get_min(), -5);
    assert_eq!(interval.get_max(), 0);
    assert_eq!(interval.get_boundaries(), (-5, 0));
}

#[test]
fn from_boundaries_large_values() {
    let interval = Interval::from_boundaries(1_000_000, 2_000_000);
    assert_eq!(interval.min, 1_000_000);
    assert_eq!(interval.max, 2_000_000);
    assert_eq!(interval.count(), 1_000_001);
    assert_eq!(interval.get_min(), 1_000_000);
    assert_eq!(interval.get_max(), 2_000_000);
    assert_eq!(interval.get_boundaries(), (1_000_000, 2_000_000));
}

#[test]
fn from_boundaries_large_values_reversed() {
    let interval = Interval::from_boundaries(2_000_000, 1_000_000);
    assert_eq!(interval.min, 1_000_000);
    assert_eq!(interval.max, 2_000_000);
    assert_eq!(interval.count(), 1_000_001);
    assert_eq!(interval.get_min(), 1_000_000);
    assert_eq!(interval.get_max(), 2_000_000);
    assert_eq!(interval.get_boundaries(), (1_000_000, 2_000_000));
}

#[test]
fn from_boundaries_i32_type() {
    let interval: Interval<i32> = Interval::from_boundaries(-100, 50);
    assert_eq!(interval.min, -100);
    assert_eq!(interval.max, 50);
    assert_eq!(interval.count(), 151);
    assert_eq!(interval.get_min(), -100);
    assert_eq!(interval.get_max(), 50);
    assert_eq!(interval.get_boundaries(), (-100, 50));
}

#[test]
fn from_boundaries_i64_type() {
    let interval: Interval<i64> = Interval::from_boundaries(-1_000_000_000, 1_000_000_000);
    assert_eq!(interval.min, -1_000_000_000);
    assert_eq!(interval.max, 1_000_000_000);
    assert_eq!(interval.count(), 2_000_000_001);
    assert_eq!(interval.get_min(), -1_000_000_000);
    assert_eq!(interval.get_max(), 1_000_000_000);
    assert_eq!(interval.get_boundaries(), (-1_000_000_000, 1_000_000_000));
}

#[test]
fn from_boundaries_consecutive_values() {
    let interval = Interval::from_boundaries(10, 11);
    assert_eq!(interval.min, 10);
    assert_eq!(interval.max, 11);
    assert_eq!(interval.count(), 2);
    assert_eq!(interval.get_min(), 10);
    assert_eq!(interval.get_max(), 11);
    assert_eq!(interval.get_boundaries(), (10, 11));
}

#[test]
fn from_boundaries_consecutive_values_reversed() {
    let interval = Interval::from_boundaries(11, 10);
    assert_eq!(interval.min, 10);
    assert_eq!(interval.max, 11);
    assert_eq!(interval.count(), 2);
    assert_eq!(interval.get_min(), 10);
    assert_eq!(interval.get_max(), 11);
    assert_eq!(interval.get_boundaries(), (10, 11));
}

#[test]
fn from_boundaries_minimum_and_maximum_valid_values_i32_type() {
    let interval: Interval<i32> = Interval::from_boundaries(i32::MIN + 1, i32::MAX);
    assert_eq!(interval.min, i32::MIN + 1);
    assert_eq!(interval.max, i32::MAX);
    // This should use the safe count calculation
    assert_eq!(interval.get_min(), i32::MIN + 1);
    assert_eq!(interval.get_max(), i32::MAX);
    assert_eq!(interval.get_boundaries(), (i32::MIN + 1, i32::MAX));
}

#[test]
fn from_boundaries_minimum_and_maximum_valid_values_i64_type() {
    let interval: Interval<i64> = Interval::from_boundaries(i64::MIN + 1, i64::MAX);
    assert_eq!(interval.min, i64::MIN + 1);
    assert_eq!(interval.max, i64::MAX);
    assert_eq!(interval.count(), 18_446_744_073_709_551_615);
    assert_eq!(interval.get_min(), i64::MIN + 1);
    assert_eq!(interval.get_max(), i64::MAX);
    assert_eq!(interval.get_boundaries(), (i64::MIN + 1, i64::MAX));
}

#[test]
#[should_panic(
    expected = "boundary1 cannot be less than the minimum interval value to prevent overflow"
)]
fn from_boundaries_boundary1_too_small_i32() {
    let _interval: Interval<i32> = Interval::from_boundaries(i32::MIN, 10);
}

#[test]
#[should_panic(
    expected = "boundary2 cannot be less than the minimum interval value to prevent overflow"
)]
fn from_boundaries_boundary2_too_small_i32() {
    let _interval: Interval<i32> = Interval::from_boundaries(10, i32::MIN);
}

#[test]
#[should_panic(
    expected = "boundary1 cannot be less than the minimum interval value to prevent overflow"
)]
fn from_boundaries_both_boundaries_too_small_i32() {
    let _interval: Interval<i32> = Interval::from_boundaries(i32::MIN, i32::MIN);
}

#[test]
#[should_panic(
    expected = "boundary1 cannot be less than the minimum interval value to prevent overflow"
)]
fn from_boundaries_boundary1_too_small_i64() {
    let _interval: Interval<i64> = Interval::from_boundaries(i64::MIN, 100);
}

#[test]
#[should_panic(
    expected = "boundary2 cannot be less than the minimum interval value to prevent overflow"
)]
fn from_boundaries_boundary2_too_small_i64() {
    let _interval: Interval<i64> = Interval::from_boundaries(100, i64::MIN);
}

#[test]
#[should_panic(
    expected = "boundary1 cannot be less than the minimum interval value to prevent overflow"
)]
fn from_boundaries_both_boundaries_too_small_i64() {
    let _interval: Interval<i64> = Interval::from_boundaries(i64::MIN, i64::MIN);
}

// Tests for from_size

#[test]
fn from_size_positive_start_positive_size() {
    let interval = Interval::from_size(5, 3);
    assert_eq!(interval.min, 5);
    assert_eq!(interval.max, 7);
    assert_eq!(interval.count(), 3);
    assert_eq!(interval.get_min(), 5);
    assert_eq!(interval.get_max(), 7);
    assert_eq!(interval.get_boundaries(), (5, 7));
}

#[test]
fn from_size_zero_start_positive_size() {
    let interval = Interval::from_size(0, 5);
    assert_eq!(interval.min, 0);
    assert_eq!(interval.max, 4);
    assert_eq!(interval.count(), 5);
    assert_eq!(interval.get_min(), 0);
    assert_eq!(interval.get_max(), 4);
    assert_eq!(interval.get_boundaries(), (0, 4));
}

#[test]
fn from_size_negative_start_positive_size() {
    let interval = Interval::from_size(-10, 4);
    assert_eq!(interval.min, -10);
    assert_eq!(interval.max, -7);
    assert_eq!(interval.count(), 4);
    assert_eq!(interval.get_min(), -10);
    assert_eq!(interval.get_max(), -7);
    assert_eq!(interval.get_boundaries(), (-10, -7));
}

#[test]
fn from_size_size_one() {
    let interval = Interval::from_size(100, 1);
    assert_eq!(interval.min, 100);
    assert_eq!(interval.max, 100);
    assert_eq!(interval.count(), 1);
    assert!(interval.has_one_value());
    assert_eq!(interval.get_min(), 100);
    assert_eq!(interval.get_max(), 100);
    assert_eq!(interval.get_boundaries(), (100, 100));
}

#[test]
fn from_size_large_values() {
    let interval = Interval::from_size(1000, 500);
    assert_eq!(interval.min, 1000);
    assert_eq!(interval.max, 1499);
    assert_eq!(interval.count(), 500);
    assert_eq!(interval.get_min(), 1000);
    assert_eq!(interval.get_max(), 1499);
    assert_eq!(interval.get_boundaries(), (1000, 1499));
}

#[test]
fn from_size_i32_type() {
    let interval: Interval<i32> = Interval::from_size(-50, 10);
    assert_eq!(interval.min, -50);
    assert_eq!(interval.max, -41);
    assert_eq!(interval.count(), 10);
    assert_eq!(interval.get_min(), -50);
    assert_eq!(interval.get_max(), -41);
    assert_eq!(interval.get_boundaries(), (-50, -41));
}

#[test]
fn from_size_i64_type() {
    let interval: Interval<i64> = Interval::from_size(-1_000_000, 2_000_000);
    assert_eq!(interval.min, -1_000_000);
    assert_eq!(interval.max, 999_999);
    assert_eq!(interval.count(), 2_000_000);
    assert_eq!(interval.get_min(), -1_000_000);
    assert_eq!(interval.get_max(), 999_999);
    assert_eq!(interval.get_boundaries(), (-1_000_000, 999_999));
}

#[test]
fn from_size_minimum_valid_start() {
    let interval: Interval<i32> = Interval::from_size(i32::MIN + 1, 5);
    assert_eq!(interval.min, i32::MIN + 1);
    assert_eq!(interval.max, i32::MIN + 5);
    assert_eq!(interval.count(), 5);
    assert_eq!(interval.get_min(), i32::MIN + 1);
    assert_eq!(interval.get_max(), i32::MIN + 5);
    assert_eq!(interval.get_boundaries(), (i32::MIN + 1, i32::MIN + 5));
}

#[test]
fn from_size_maximum_valid_start() {
    let interval: Interval<i32> = Interval::from_size(i32::MAX - 5, 5);
    assert_eq!(interval.min, i32::MAX - 5);
    assert_eq!(interval.max, i32::MAX - 1);
    assert_eq!(interval.count(), 5);
    assert_eq!(interval.get_min(), i32::MAX - 5);
    assert_eq!(interval.get_max(), i32::MAX - 1);
    assert_eq!(interval.get_boundaries(), (i32::MAX - 5, i32::MAX - 1));
}

#[test]
#[should_panic(
    expected = "start cannot be less than the minimum interval value to prevent overflow"
)]
fn from_size_invalid_start_too_small() {
    let _interval: Interval<i32> = Interval::from_size(i32::MIN, 5);
}

#[test]
#[should_panic(expected = "start + size would exceed the maximum interval value")]
fn from_size_invalid_resulting_max_too_large() {
    let _interval: Interval<i32> = Interval::from_size(i32::MAX - 2, 5);
}

#[test]
#[should_panic(expected = "size must be positive")]
fn from_size_zero_size() {
    let _interval = Interval::from_size(10, 0);
}

#[test]
#[should_panic(expected = "size must be positive")]
fn from_size_negative_size() {
    let _interval = Interval::from_size(10, -5);
}

// Tests for whole

#[test]
fn whole_i32() {
    let interval: Interval<i32> = Interval::whole();
    assert_eq!(interval.min, i32::MIN + 1);
    assert_eq!(interval.max, i32::MAX);
    let count = (i32::MAX as i128) - (i32::MIN as i128);
    assert_eq!(interval.count(), count as u64);
    assert_eq!(interval.get_min(), i32::MIN + 1);
    assert_eq!(interval.get_max(), i32::MAX);
    assert_eq!(interval.get_boundaries(), (i32::MIN + 1, i32::MAX));
}

#[test]
fn whole_i64() {
    let interval: Interval<i64> = Interval::whole();
    assert_eq!(interval.min, i64::MIN + 1);
    assert_eq!(interval.max, i64::MAX);
    let count = (i64::MAX as i128) - (i64::MIN as i128);
    assert_eq!(interval.count(), count as u64);
    assert_eq!(interval.get_min(), i64::MIN + 1);
    assert_eq!(interval.get_max(), i64::MAX);
    assert_eq!(interval.get_boundaries(), (i64::MIN + 1, i64::MAX));
}

// Tests for contains

#[test]
fn contains_positive_values() {
    let interval = Interval::from_boundaries(10, 20);
    assert!(interval.contains(15));
    assert!(interval.contains(10)); // left boundary
    assert!(interval.contains(20)); // right boundary
    assert!(!interval.contains(5));
    assert!(!interval.contains(25));
    assert!(!interval.contains(-2));
    assert!(!interval.contains(100));
}

#[test]
fn contains_negative_values() {
    let interval = Interval::from_boundaries(-20, -10);
    assert!(interval.contains(-15));
    assert!(interval.contains(-20));
    assert!(interval.contains(-10));
    assert!(!interval.contains(-25));
    assert!(!interval.contains(-5));
    assert!(!interval.contains(15));
}

// Tests for has_one_value

#[test]
fn has_one_value_true() {
    let interval = Interval::from_boundaries(7, 7);
    assert!(interval.has_one_value());
    assert_eq!(interval.count(), 1);
}

#[test]
fn has_one_value_false() {
    let interval = Interval::from_boundaries(5, 10);
    assert!(!interval.has_one_value());
    assert_ne!(interval.count(), 1);
}

// Tests for subsumes

#[test]
fn subsumes_completely_inside() {
    let outer = Interval::from_boundaries(5, 15);
    let inner = Interval::from_boundaries(8, 12);
    assert!(outer.subsumes(&inner));
    assert!(!inner.subsumes(&outer));
}

#[test]
fn subsumes_same_interval() {
    let interval1 = Interval::from_boundaries(5, 15);
    let interval2 = Interval::from_boundaries(5, 15);
    assert!(interval1.subsumes(&interval2));
    assert!(interval2.subsumes(&interval1));
}

#[test]
fn subsumes_partial_overlap() {
    let interval1 = Interval::from_boundaries(5, 15);
    let interval2 = Interval::from_boundaries(10, 20);
    assert!(!interval1.subsumes(&interval2));
    assert!(!interval2.subsumes(&interval1));
}

#[test]
fn subsumes_no_overlap() {
    let interval1 = Interval::from_boundaries(5, 10);
    let interval2 = Interval::from_boundaries(15, 20);
    assert!(!interval1.subsumes(&interval2));
    assert!(!interval2.subsumes(&interval1));
}

// Tests for overlaps

#[test]
fn overlaps_partial() {
    let interval1 = Interval::from_boundaries(5, 15);
    let interval2 = Interval::from_boundaries(10, 20);
    assert!(interval1.overlaps(&interval2));
    assert!(interval2.overlaps(&interval1));
}

#[test]
fn overlaps_touching_boundary() {
    let interval1 = Interval::from_boundaries(5, 10);
    let interval2 = Interval::from_boundaries(10, 15);
    assert!(interval1.overlaps(&interval2));
    assert!(interval2.overlaps(&interval1));
}

#[test]
fn overlaps_separate() {
    let interval1 = Interval::from_boundaries(5, 10);
    let interval2 = Interval::from_boundaries(15, 20);
    assert!(!interval1.overlaps(&interval2));
    assert!(!interval2.overlaps(&interval1));
}

#[test]
fn overlaps_one_subsumes_other() {
    let outer = Interval::from_boundaries(5, 20);
    let inner = Interval::from_boundaries(8, 12);
    assert!(outer.overlaps(&inner));
    assert!(inner.overlaps(&outer));
}

// Tests for get_relationship_with

#[test]
fn get_relationship_subsumed() {
    let outer = Interval::from_boundaries(5, 20);
    let inner = Interval::from_boundaries(8, 12);
    assert_eq!(outer.get_relationship_with(&inner), Relationship::Subsumed);
    assert_eq!(inner.get_relationship_with(&outer), Relationship::Subsumed);
}

#[test]
fn get_relationship_overlapped() {
    let interval1 = Interval::from_boundaries(5, 15);
    let interval2 = Interval::from_boundaries(10, 20);
    assert_eq!(
        interval1.get_relationship_with(&interval2),
        Relationship::Overlapped
    );
    assert_eq!(
        interval2.get_relationship_with(&interval1),
        Relationship::Overlapped
    );
}

#[test]
fn get_relationship_isolated() {
    let interval1 = Interval::from_boundaries(5, 10);
    let interval2 = Interval::from_boundaries(15, 20);
    assert_eq!(
        interval1.get_relationship_with(&interval2),
        Relationship::Isolated
    );
    assert_eq!(
        interval2.get_relationship_with(&interval1),
        Relationship::Isolated
    );
}

// Tests for join

#[test]
fn join_overlapping_intervals() {
    let interval1 = Interval::from_boundaries(5, 15);
    let interval2 = Interval::from_boundaries(10, 20);
    {
        let result = interval1.join(&interval2);
        assert_some!(result);
        assert_eq!(result.unwrap().get_boundaries(), (5, 20));
    }
    {
        let result = interval2.join(&interval1);
        assert_some!(result);
        assert_eq!(result.unwrap().get_boundaries(), (5, 20));
    }
}

#[test]
fn join_contiguous_intervals() {
    let interval1 = Interval::from_boundaries(5, 10);
    let interval2 = Interval::from_boundaries(11, 15);
    {
        let result = interval1.join(&interval2);
        assert_some!(result);
        assert_eq!(result.unwrap().get_boundaries(), (5, 15));
    }
    {
        let result = interval2.join(&interval1);
        assert_some!(result);
        assert_eq!(result.unwrap().get_boundaries(), (5, 15));
    }
}

#[test]
fn join_separate_intervals_returns_none() {
    let interval1 = Interval::from_boundaries(5, 10);
    let interval2 = Interval::from_boundaries(15, 20);
    assert_none!(interval1.join(&interval2));
    assert_none!(interval2.join(&interval1));
}

#[test]
fn join_identical_intervals() {
    let interval1 = Interval::from_boundaries(5, 10);
    let interval2 = Interval::from_boundaries(5, 10);
    {
        let result = interval1.join(&interval2);
        assert_some!(result);
        assert_eq!(result.unwrap().get_boundaries(), (5, 10));
    }
    {
        let result = interval2.join(&interval1);
        assert_some!(result);
        assert_eq!(result.unwrap().get_boundaries(), (5, 10));
    }
}

// Tests for intersect

#[test]
fn intersect_overlapping_intervals() {
    let interval1 = Interval::from_boundaries(5, 15);
    let interval2 = Interval::from_boundaries(10, 20);
    {
        let result = interval1.intersect(&interval2);
        assert_some!(result);
        assert_eq!(result.unwrap().get_boundaries(), (10, 15));
    }
    {
        let result = interval2.intersect(&interval1);
        assert_some!(result);
        assert_eq!(result.unwrap().get_boundaries(), (10, 15));
    }
}

#[test]
fn intersect_one_subsumes_other() {
    let outer = Interval::from_boundaries(5, 20);
    let inner = Interval::from_boundaries(8, 12);
    {
        let result = outer.intersect(&inner);
        assert_some!(result);
        assert_eq!(result.unwrap().get_boundaries(), (8, 12));
    }
    {
        let result = inner.intersect(&outer);
        assert_some!(result);
        assert_eq!(result.unwrap().get_boundaries(), (8, 12));
    }
}

#[test]
fn intersect_no_overlap() {
    let interval1 = Interval::from_boundaries(5, 10);
    let interval2 = Interval::from_boundaries(15, 20);
    assert_none!(interval1.intersect(&interval2));
}

#[test]
fn intersect_touching_boundaries() {
    let interval1 = Interval::from_boundaries(5, 10);
    let interval2 = Interval::from_boundaries(10, 15);
    {
        let result = interval1.intersect(&interval2);
        assert_some!(result);
        assert_eq!(result.unwrap().get_boundaries(), (10, 10));
    }
    {
        let result = interval2.intersect(&interval1);
        assert_some!(result);
        assert_eq!(result.unwrap().get_boundaries(), (10, 10));
    }
}

// Tests for is_contiguous_to

#[test]
fn is_contiguous_to_right_adjacent() {
    let interval1 = Interval::from_boundaries(5, 10);
    let interval2 = Interval::from_boundaries(11, 15);
    assert!(interval1.is_contiguous_to(&interval2));
    assert!(interval2.is_contiguous_to(&interval1));
}

#[test]
fn is_contiguous_to_gap() {
    let interval1 = Interval::from_boundaries(5, 10);
    let interval2 = Interval::from_boundaries(13, 18);
    assert!(!interval1.is_contiguous_to(&interval2));
    assert!(!interval2.is_contiguous_to(&interval1));
}

#[test]
fn is_contiguous_to_overlapping() {
    let interval1 = Interval::from_boundaries(5, 12);
    let interval2 = Interval::from_boundaries(10, 15);
    assert!(!interval1.is_contiguous_to(&interval2));
    assert!(!interval2.is_contiguous_to(&interval1));
}

// Tests for get_location

#[test]
fn get_location_within() {
    let interval = Interval::from_boundaries(10, 20);
    assert_eq!(interval.get_location(15), Location::Within);
    assert_eq!(interval.get_location(11), Location::Within);
    assert_eq!(interval.get_location(19), Location::Within);
}

#[test]
fn get_location_left_boundary() {
    let interval = Interval::from_boundaries(10, 20);
    assert_eq!(interval.get_location(10), Location::LeftBoundary);
}

#[test]
fn get_location_right_boundary() {
    let interval = Interval::from_boundaries(10, 20);
    assert_eq!(interval.get_location(20), Location::RightBoundary);
}

#[test]
fn get_location_left_outside() {
    let interval = Interval::from_boundaries(10, 20);
    assert_eq!(interval.get_location(5), Location::LeftOutside);
    assert_eq!(interval.get_location(-7), Location::LeftOutside);
}

#[test]
fn get_location_right_outside() {
    let interval = Interval::from_boundaries(10, 20);
    assert_eq!(interval.get_location(25), Location::RightOutside);
    assert_eq!(interval.get_location(101), Location::RightOutside);
}

// Tests for shift

#[test]
fn shift_positive_interval_positive_offset() {
    let interval = Interval::from_boundaries(10, 20);
    let shifted = interval.shift(5);
    assert_eq!(shifted.get_boundaries(), (15, 25));
}

#[test]
fn shift_positive_interval_negative_offset() {
    let interval = Interval::from_boundaries(10, 20);
    let shifted = interval.shift(-3);
    assert_eq!(shifted.get_boundaries(), (7, 17));
}

#[test]
fn shift_positive_interval_zero_offset() {
    let interval = Interval::from_boundaries(10, 20);
    let shifted = interval.shift(0);
    assert_eq!(shifted.get_boundaries(), (10, 20));
}

#[test]
fn shift_negative_interval_positive_offset() {
    let interval = Interval::from_boundaries(-20, -10);
    let shifted = interval.shift(5);
    assert_eq!(shifted.get_boundaries(), (-15, -5));
}

#[test]
fn shift_negative_interval_negative_offset() {
    let interval = Interval::from_boundaries(-20, -10);
    let shifted = interval.shift(-5);
    assert_eq!(shifted.get_boundaries(), (-25, -15));
}

// Tests for expand_equally

#[test]
fn expand_equally_positive_interval_positive_offset() {
    let interval = Interval::from_boundaries(10, 20);
    let expanded = interval.expand_equally(5);
    assert_eq!(expanded.get_boundaries(), (5, 25));
}

#[test]
fn expand_equally_positive_interval_negative_offset() {
    let interval = Interval::from_boundaries(10, 20);
    let expanded = interval.expand_equally(-5);
    assert_eq!(expanded.get_boundaries(), (15, 15));
}

#[test]
fn expand_equally_positive_interval_zero_offset() {
    let interval = Interval::from_boundaries(10, 20);
    let expanded = interval.expand_equally(0);
    assert_eq!(expanded.get_boundaries(), (10, 20));
}

#[test]
fn expand_equally_negative_interval_positive_offset() {
    let interval = Interval::from_boundaries(-20, -10);
    let expanded = interval.expand_equally(5);
    assert_eq!(expanded.get_boundaries(), (-25, -5));
}

#[test]
fn expand_equally_negative_interval_negative_offset() {
    let interval = Interval::from_boundaries(-20, -10);
    let expanded = interval.expand_equally(-5);
    assert_eq!(expanded.get_boundaries(), (-15, -15));
}

#[test]
#[should_panic]
fn expand_equally_wrong_offset() {
    let interval = Interval::from_boundaries(10, 20);
    let _expanded = interval.expand_equally(-10);
}

// Tests for expand

#[test]
fn expand_positive_offsets() {
    let interval = Interval::from_boundaries(10, 20);
    let expanded = interval.expand(3, 7);
    assert_eq!(expanded.get_boundaries(), (7, 27));
}

#[test]
fn expand_negative_offsets() {
    let interval = Interval::from_boundaries(10, 20);
    let expanded = interval.expand(-2, -3);
    assert_eq!(expanded.get_boundaries(), (12, 17));
}

#[test]
fn expand_zero_offsets() {
    let interval = Interval::from_boundaries(10, 20);
    let expanded = interval.expand(0, 0);
    assert_eq!(expanded.get_boundaries(), (10, 20));
}

#[test]
fn expand_only_left_offset() {
    let interval = Interval::from_boundaries(10, 20);
    let expanded = interval.expand(3, 0);
    assert_eq!(expanded.get_boundaries(), (7, 20));
}

#[test]
fn expand_only_right_offset() {
    let interval = Interval::from_boundaries(10, 20);
    let expanded = interval.expand(0, 3);
    assert_eq!(expanded.get_boundaries(), (10, 23));
}

#[test]
#[should_panic]
fn expand_wrong_offset() {
    let interval = Interval::from_boundaries(10, 20);
    let _expanded = interval.expand(-10, -10);
}

// Tests for difference

#[test]
fn difference_no_overlap() {
    let interval1 = Interval::from_boundaries(5, 10);
    let interval2 = Interval::from_boundaries(15, 20);
    {
        let result = interval1.difference(&interval2);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].get_boundaries(), (5, 10));
        assert_eq!(result[1].get_boundaries(), (15, 20));
    }
    {
        let result = interval2.difference(&interval1);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].get_boundaries(), (5, 10));
        assert_eq!(result[1].get_boundaries(), (15, 20));
    }
}

#[test]
fn difference_partial_overlap_left() {
    let interval1 = Interval::from_boundaries(5, 15);
    let interval2 = Interval::from_boundaries(10, 20);
    {
        let result = interval1.difference(&interval2);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].get_boundaries(), (5, 9));
        assert_eq!(result[1].get_boundaries(), (16, 20));
    }
    {
        let result = interval2.difference(&interval1);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].get_boundaries(), (5, 9));
        assert_eq!(result[1].get_boundaries(), (16, 20));
    }
}

#[test]
fn difference_partial_overlap_right() {
    let interval1 = Interval::from_boundaries(10, 20);
    let interval2 = Interval::from_boundaries(5, 15);
    {
        let result = interval1.difference(&interval2);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].get_boundaries(), (5, 9));
        assert_eq!(result[1].get_boundaries(), (16, 20));
    }
    {
        let result = interval2.difference(&interval1);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].get_boundaries(), (5, 9));
        assert_eq!(result[1].get_boundaries(), (16, 20));
    }
}

#[test]
fn difference_one_subsumes_other() {
    let outer = Interval::from_boundaries(5, 20);
    let inner = Interval::from_boundaries(8, 12);
    {
        let result = outer.difference(&inner);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].get_boundaries(), (5, 7));
        assert_eq!(result[1].get_boundaries(), (13, 20));
    }
    {
        let result = inner.difference(&outer);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].get_boundaries(), (5, 7));
        assert_eq!(result[1].get_boundaries(), (13, 20));
    }
}

#[test]
fn difference_same_left_boundary() {
    let interval1 = Interval::from_boundaries(5, 15);
    let interval2 = Interval::from_boundaries(5, 20);
    {
        let result = interval1.difference(&interval2);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].get_boundaries(), (16, 20));
    }
    {
        let result = interval2.difference(&interval1);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].get_boundaries(), (16, 20));
    }
}

#[test]
fn difference_same_right_boundary() {
    let interval1 = Interval::from_boundaries(10, 20);
    let interval2 = Interval::from_boundaries(5, 20);
    {
        let result = interval1.difference(&interval2);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].get_boundaries(), (5, 9));
    }
    {
        let result = interval2.difference(&interval1);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].get_boundaries(), (5, 9));
    }
}

#[test]
fn difference_identical_intervals() {
    let interval1 = Interval::from_boundaries(5, 15);
    let interval2 = Interval::from_boundaries(5, 15);
    {
        let result = interval1.difference(&interval2);
        assert_eq!(result.len(), 0);
    }
    {
        let result = interval2.difference(&interval1);
        assert_eq!(result.len(), 0);
    }
}

// Tests for get_relative_position_from

#[test]
fn get_relative_position_from_start_boundary_positive() {
    let interval = Interval::from_boundaries(10, 20);
    assert_eq!(interval.get_relative_position_from(Boundary::Start, 15), 5);
    assert_eq!(interval.get_relative_position_from(Boundary::Start, 10), 0);
    assert_eq!(interval.get_relative_position_from(Boundary::Start, 5), -5);
}

#[test]
fn get_relative_position_from_end_boundary_positive() {
    let interval = Interval::from_boundaries(10, 20);
    assert_eq!(interval.get_relative_position_from(Boundary::End, 25), 5);
    assert_eq!(interval.get_relative_position_from(Boundary::End, 20), 0);
    assert_eq!(interval.get_relative_position_from(Boundary::End, 15), -5);
}

#[test]
fn get_relative_position_from_start_boundary_negative() {
    let interval = Interval::from_boundaries(-20, -10);
    assert_eq!(interval.get_relative_position_from(Boundary::Start, -15), 5);
    assert_eq!(interval.get_relative_position_from(Boundary::Start, -20), 0);
    assert_eq!(
        interval.get_relative_position_from(Boundary::Start, -25),
        -5
    );
}

#[test]
fn get_relative_position_from_end_boundary_negative() {
    let interval = Interval::from_boundaries(-20, -10);
    assert_eq!(interval.get_relative_position_from(Boundary::End, -5), 5);
    assert_eq!(interval.get_relative_position_from(Boundary::End, -10), 0);
    assert_eq!(interval.get_relative_position_from(Boundary::End, -15), -5);
}

#[test]
fn get_relative_position_from_zero_interval() {
    let interval = Interval::from_boundaries(0, 0);
    assert_eq!(interval.get_relative_position_from(Boundary::Start, 5), 5);
    assert_eq!(interval.get_relative_position_from(Boundary::Start, 0), 0);
    assert_eq!(interval.get_relative_position_from(Boundary::Start, -3), -3);
    assert_eq!(interval.get_relative_position_from(Boundary::End, 5), 5);
    assert_eq!(interval.get_relative_position_from(Boundary::End, 0), 0);
    assert_eq!(interval.get_relative_position_from(Boundary::End, -3), -3);
}

#[test]
fn get_relative_position_from_crossing_zero() {
    let interval = Interval::from_boundaries(-5, 15);
    assert_eq!(interval.get_relative_position_from(Boundary::Start, 0), 5);
    assert_eq!(interval.get_relative_position_from(Boundary::Start, -5), 0);
    assert_eq!(
        interval.get_relative_position_from(Boundary::Start, -10),
        -5
    );
    assert_eq!(interval.get_relative_position_from(Boundary::End, 20), 5);
    assert_eq!(interval.get_relative_position_from(Boundary::End, 15), 0);
    assert_eq!(interval.get_relative_position_from(Boundary::End, 10), -5);
}

#[test]
fn get_relative_position_from_large_values() {
    let interval = Interval::from_boundaries(1000000, 2000000);
    assert_eq!(
        interval.get_relative_position_from(Boundary::Start, 1500000),
        500000
    );
    assert_eq!(
        interval.get_relative_position_from(Boundary::Start, 1000000),
        0
    );
    assert_eq!(
        interval.get_relative_position_from(Boundary::Start, 500000),
        -500000
    );
    assert_eq!(
        interval.get_relative_position_from(Boundary::End, 2500000),
        500000
    );
    assert_eq!(
        interval.get_relative_position_from(Boundary::End, 2000000),
        0
    );
    assert_eq!(
        interval.get_relative_position_from(Boundary::End, 1500000),
        -500000
    );
}

#[test]
fn get_relative_position_from_i64_type() {
    let interval: Interval<i64> = Interval::from_boundaries(-100, 50);
    assert_eq!(
        interval.get_relative_position_from(Boundary::Start, -50),
        50
    );
    assert_eq!(
        interval.get_relative_position_from(Boundary::Start, -100),
        0
    );
    assert_eq!(interval.get_relative_position_from(Boundary::End, 100), 50);
    assert_eq!(interval.get_relative_position_from(Boundary::End, 50), 0);
}

// Tests for Shl (left shift) operator

#[test]
fn shl_positive_shift() {
    let interval = Interval::from_boundaries(10, 20);
    let shifted = interval << 5;
    assert_eq!(shifted.get_boundaries(), (5, 15)); // Shl now decreases values
    assert_eq!(shifted.count(), interval.count()); // Size remains the same
}

#[test]
fn shl_zero_shift() {
    let interval = Interval::from_boundaries(10, 20);
    let shifted = interval << 0;
    assert_eq!(shifted.get_boundaries(), (10, 20));
    assert_eq!(shifted.count(), interval.count());
    assert_eq!(shifted.get_min(), 10);
    assert_eq!(shifted.get_max(), 20);
}

#[test]
fn shl_negative_shift() {
    let interval = Interval::from_boundaries(10, 20);
    let shifted = interval << -5;
    assert_eq!(shifted.get_boundaries(), (15, 25)); // Negative shift increases values
    assert_eq!(shifted.count(), interval.count());
}

#[test]
fn shl_negative_values() {
    let interval = Interval::from_boundaries(-20, -10);
    let shifted = interval << 15;
    assert_eq!(shifted.get_boundaries(), (-35, -25)); // Shl decreases values
    assert_eq!(shifted.count(), interval.count());
}

#[test]
fn shl_single_value_interval() {
    let interval = Interval::from_boundaries(7, 7);
    let shifted = interval << 3;
    assert_eq!(shifted.get_boundaries(), (4, 4)); // Shl decreases values
    assert_eq!(shifted.count(), interval.count());
    assert!(shifted.has_one_value());
}

#[test]
fn shl_large_shift() {
    let interval = Interval::from_boundaries(1100, 1200);
    let shifted = interval << 1000;
    assert_eq!(shifted.get_boundaries(), (100, 200)); // Shl decreases values
    assert_eq!(shifted.count(), interval.count());
}

#[test]
fn shl_i32_type() {
    let interval: Interval<i32> = Interval::from_boundaries(-50, 50);
    let shifted = interval << 25;
    assert_eq!(shifted.get_boundaries(), (-75, 25)); // Shl decreases values
    assert_eq!(shifted.count(), interval.count());
    assert_eq!(shifted.get_min(), -75);
    assert_eq!(shifted.get_max(), 25);
}

#[test]
fn shl_i64_type() {
    let interval: Interval<i64> = Interval::from_boundaries(-1_000_000, 1_000_000);
    let shifted = interval << 500_000;
    assert_eq!(shifted.get_boundaries(), (-1_500_000, 500_000)); // Shl decreases values
    assert_eq!(shifted.count(), interval.count());
    assert_eq!(shifted.get_min(), -1_500_000);
    assert_eq!(shifted.get_max(), 500_000);
}

// Tests for Shr (right shift) operator

#[test]
fn shr_positive_shift() {
    let interval = Interval::from_boundaries(10, 20);
    let shifted = interval >> 5;
    assert_eq!(shifted.get_boundaries(), (15, 25)); // Shr now increases values
    assert_eq!(shifted.count(), interval.count()); // Size remains the same
}

#[test]
fn shr_zero_shift() {
    let interval = Interval::from_boundaries(10, 20);
    let shifted = interval >> 0;
    assert_eq!(shifted.get_boundaries(), (10, 20));
    assert_eq!(shifted.count(), interval.count());
}

#[test]
fn shr_negative_shift() {
    let interval = Interval::from_boundaries(10, 20);
    let shifted = interval >> -5;
    assert_eq!(shifted.get_boundaries(), (5, 15)); // Negative shift decreases values
    assert_eq!(shifted.count(), interval.count());
}

#[test]
fn shr_negative_values() {
    let interval = Interval::from_boundaries(-15, -5);
    let shifted = interval >> 10;
    assert_eq!(shifted.get_boundaries(), (-5, 5)); // Shr increases values
    assert_eq!(shifted.count(), interval.count());
}

#[test]
fn shr_crossing_zero() {
    let interval = Interval::from_boundaries(-5, 5);
    let shifted = interval >> 10;
    assert_eq!(shifted.get_boundaries(), (5, 15)); // Shr increases values
    assert_eq!(shifted.count(), interval.count());
}

#[test]
fn shr_single_value_interval() {
    let interval = Interval::from_boundaries(7, 7);
    let shifted = interval >> 3;
    assert_eq!(shifted.get_boundaries(), (10, 10)); // Shr increases values
    assert_eq!(shifted.count(), interval.count());
    assert!(shifted.has_one_value());
}

#[test]
fn shr_large_shift() {
    let interval = Interval::from_boundaries(100, 200);
    let shifted = interval >> 1000;
    assert_eq!(shifted.get_boundaries(), (1100, 1200)); // Shr increases values
    assert_eq!(shifted.count(), interval.count());
}

#[test]
fn shr_i32_type() {
    let interval: Interval<i32> = Interval::from_boundaries(-75, 25);
    let shifted = interval >> 25;
    assert_eq!(shifted.get_boundaries(), (-50, 50)); // Shr increases values
    assert_eq!(shifted.count(), interval.count());
}

#[test]
fn shr_i64_type() {
    let interval: Interval<i64> = Interval::from_boundaries(-1500000, 500000);
    let shifted = interval >> 500000;
    assert_eq!(shifted.get_boundaries(), (-1000000, 1000000)); // Shr increases values
    assert_eq!(shifted.count(), interval.count());
}

// Tests comparing shift operators with shift method

#[test]
fn shl_equivalent_to_shift_negative() {
    let interval = Interval::from_boundaries(10, 20);
    let shl_result = interval << 5;
    let shift_result = interval.shift(-5); // Shl now equals negative shift
    assert_eq!(shl_result.get_boundaries(), shift_result.get_boundaries());
    assert_eq!(shl_result.count(), shift_result.count());
    assert_eq!(shl_result.get_min(), shift_result.get_min());
    assert_eq!(shl_result.get_max(), shift_result.get_max());
}

#[test]
fn shr_equivalent_to_shift_positive() {
    let interval = Interval::from_boundaries(10, 20);
    let shr_result = interval >> 5;
    let shift_result = interval.shift(5); // Shr now equals positive shift
    assert_eq!(shr_result.get_boundaries(), shift_result.get_boundaries());
    assert_eq!(shr_result.count(), shift_result.count());
    assert_eq!(shr_result.get_min(), shift_result.get_min());
    assert_eq!(shr_result.get_max(), shift_result.get_max());
}

#[test]
fn shl_negative_equivalent_to_shr_positive() {
    let interval = Interval::from_boundaries(10, 20);
    let shl_negative = interval << -3; // Negative shl increases values
    let shr_positive = interval >> 3; // Positive shr increases values
    assert_eq!(shl_negative.get_boundaries(), shr_positive.get_boundaries());
    assert_eq!(shl_negative.count(), shr_positive.count());
    assert_eq!(shl_negative.get_min(), shr_positive.get_min());
    assert_eq!(shl_negative.get_max(), shr_positive.get_max());
}

#[test]
fn shr_negative_equivalent_to_shl_positive() {
    let interval = Interval::from_boundaries(10, 20);
    let shr_negative = interval >> -3; // Negative shr decreases values
    let shl_positive = interval << 3; // Positive shl decreases values
    assert_eq!(shr_negative.get_boundaries(), shl_positive.get_boundaries());
    assert_eq!(shr_negative.count(), shl_positive.count());
    assert_eq!(shr_negative.get_min(), shl_positive.get_min());
    assert_eq!(shr_negative.get_max(), shl_positive.get_max());
}

// Tests for Display trait

#[test]
fn fmt_display_positive_values() {
    let interval = Interval::from_boundaries(10, 20);
    assert_eq!(format!("{}", interval), "[10, 20]");
}

#[test]
fn fmt_display_negative_values() {
    let interval = Interval::from_boundaries(-20, -10);
    assert_eq!(format!("{}", interval), "[-20, -10]");
}

// Tests for PartialOrd and Ord trait

#[test]
fn ord() {
    let a = Interval::from_boundaries(1, 5);
    let b = Interval::from_boundaries(2, 6);
    let c = Interval::from_boundaries(0, 3);
    let d = Interval::from_boundaries(1, 5);

    let mut intervals = vec![&b, &a, &c, &d];
    intervals.sort();
    let mins: Vec<_> = intervals.iter().map(|i| i.get_min()).collect();
    assert_eq!(mins, vec![0, 1, 1, 2]);
}

#[test]
fn partial_ord() {
    let a = Interval::from_boundaries(1, 5);
    let b = Interval::from_boundaries(2, 6);
    let c = Interval::from_boundaries(0, 3);
    let d = Interval::from_boundaries(1, 5);

    assert!(a < b);
    assert!(c < a);
    assert!(b > c);
    assert_eq!(a.partial_cmp(&d), Some(std::cmp::Ordering::Equal));
    assert_eq!(a.cmp(&d), std::cmp::Ordering::Equal);
}

// Tests for Iterator trait

#[test]
fn iterator_basic_positive() {
    let interval = Interval::from_boundaries(3, 7);
    let values: Vec<_> = interval.into_iter().collect();
    assert_eq!(values, vec![3, 4, 5, 6, 7]);
}

#[test]
fn iterator_basic_negative() {
    let interval = Interval::from_boundaries(-3, 1);
    let values: Vec<_> = interval.into_iter().collect();
    assert_eq!(values, vec![-3, -2, -1, 0, 1]);
}

#[test]
fn iterator_single_value() {
    let interval = Interval::from_boundaries(5, 5);
    let values: Vec<_> = interval.into_iter().collect();
    assert_eq!(values, vec![5]);
}
