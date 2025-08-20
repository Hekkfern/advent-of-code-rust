use super::*;
use pretty_assertions::{assert_eq, assert_ne};

// Tests for new()

#[test]
fn new_creates_empty_set() {
    let set: IntervalSet<i32> = IntervalSet::new();
    assert!(set.is_empty());
    assert_eq!(set.count(), 0);
    assert_eq!(set.get().len(), 0);
}

// Tests for from_vec()

#[test]
fn from_vec_empty_vector() {
    let set: IntervalSet<i32> = IntervalSet::from_vec(vec![]);
    assert!(set.is_empty());
    assert_eq!(set.count(), 0);
    assert_eq!(set.get().len(), 0);
}

#[test]
fn from_vec_single_interval() {
    let interval = Interval::from_boundaries(5, 10);
    let set = IntervalSet::from_vec(vec![interval]);
    assert!(!set.is_empty());
    assert_eq!(set.count(), 6);
    let intervals = set.get();
    assert_eq!(intervals.len(), 1);
    assert!(intervals.contains(&interval));
}

#[test]
fn from_vec_non_overlapping_intervals() {
    let interval1 = Interval::from_boundaries(1, 3);
    let interval2 = Interval::from_boundaries(7, 9);
    let interval3 = Interval::from_boundaries(15, 20);
    let set = IntervalSet::from_vec(vec![interval1, interval2, interval3]);
    assert!(!set.is_empty());
    assert_eq!(
        set.count(),
        interval1.count() + interval2.count() + interval3.count()
    );
    let intervals = set.get();
    assert_eq!(intervals.len(), 3);
    assert!(intervals.contains(&interval1));
    assert!(intervals.contains(&interval2));
    assert!(intervals.contains(&interval3));
}

#[test]
fn from_vec_overlapping_intervals() {
    let interval1 = Interval::from_boundaries(1, 10);
    let interval2 = Interval::from_boundaries(5, 15);
    let interval3 = Interval::from_boundaries(12, 20);
    let set = IntervalSet::from_vec(vec![interval1, interval2, interval3]);
    assert!(!set.is_empty());
    let intervals = set.get();
    assert_eq!(intervals.len(), 1);
    assert_eq!(intervals[0], Interval::from_boundaries(1, 20));
}

#[test]
fn from_vec_contiguous_intervals() {
    let interval1 = Interval::from_boundaries(1, 5);
    let interval2 = Interval::from_boundaries(6, 10);
    let interval3 = Interval::from_boundaries(11, 15);
    let set = IntervalSet::from_vec(vec![interval1, interval2, interval3]);
    assert!(!set.is_empty());
    let intervals = set.get();
    assert_eq!(intervals.len(), 1);
    assert_eq!(intervals[0], Interval::from_boundaries(1, 15));
}

#[test]
fn from_vec_identical_intervals() {
    let interval = Interval::from_boundaries(5, 10);
    let set = IntervalSet::from_vec(vec![interval, interval, interval]);
    assert!(!set.is_empty());
    let intervals = set.get();
    assert_eq!(intervals.len(), 1);
    assert_eq!(intervals[0], interval);
}

#[test]
fn from_vec_negative_values() {
    let interval1 = Interval::from_boundaries(-20, -10);
    let interval2 = Interval::from_boundaries(-11, 5);
    let interval3 = Interval::from_boundaries(10, 20);
    let set = IntervalSet::from_vec(vec![interval1, interval2, interval3]);
    let intervals = set.get();
    assert_eq!(intervals.len(), 2);
    assert!(intervals.contains(&Interval::from_boundaries(-20, 5)));
    assert!(intervals.contains(&Interval::from_boundaries(10, 20)));
}

// Tests for add()

#[test]
fn add_to_empty_set() {
    let mut set = IntervalSet::new();
    let interval1 = Interval::from_boundaries(5, 10);
    let interval2 = Interval::from_boundaries(20, 30);
    set.add(interval1);
    set.add(interval2);
    assert!(!set.is_empty());
    let intervals = set.get();
    assert_eq!(intervals.len(), 2);
    assert!(intervals.contains(&interval1));
    assert!(intervals.contains(&interval2));
}

#[test]
fn add_non_overlapping_interval() {
    let interval1 = Interval::from_boundaries(1, 5);
    let mut set = IntervalSet::from_vec(vec![interval1]);
    let interval2 = Interval::from_boundaries(10, 15);
    set.add(interval2);
    assert!(!set.is_empty());
    let intervals = set.get();
    assert_eq!(intervals.len(), 2);
    assert!(intervals.contains(&interval1));
    assert!(intervals.contains(&interval2));
}

#[test]
fn add_overlapping_interval() {
    let interval1 = Interval::from_boundaries(1, 10);
    let mut set = IntervalSet::from_vec(vec![interval1]);
    let interval2 = Interval::from_boundaries(5, 15);
    set.add(interval2);
    assert!(!set.is_empty());
    let intervals = set.get();
    assert_eq!(intervals.len(), 1);
    assert_eq!(intervals[0], Interval::from_boundaries(1, 15));
}

#[test]
fn add_contiguous_interval() {
    let interval1 = Interval::from_boundaries(1, 5);
    let mut set = IntervalSet::from_vec(vec![interval1]);
    let interval2 = Interval::from_boundaries(6, 10);
    set.add(Interval::from_boundaries(6, 10));
    assert!(!set.is_empty());
    let intervals = set.get();
    assert_eq!(intervals.len(), 1);
    assert_eq!(intervals[0], Interval::from_boundaries(1, 10));
}

#[test]
fn add_subsumed_interval() {
    let interval1 = Interval::from_boundaries(1, 20);
    let mut set = IntervalSet::from_vec(vec![interval1]);
    let interval2 = Interval::from_boundaries(5, 15);
    set.add(interval2);
    assert!(!set.is_empty());
    let intervals = set.get();
    assert_eq!(intervals.len(), 1);
    assert_eq!(intervals[0], interval1);
}

#[test]
fn add_subsuming_interval() {
    let interval1 = Interval::from_boundaries(5, 15);
    let mut set = IntervalSet::from_vec(vec![interval1]);
    let interval2 = Interval::from_boundaries(1, 20);
    set.add(interval2);
    assert!(!set.is_empty());
    let intervals = set.get();
    assert_eq!(intervals.len(), 1);
    assert_eq!(intervals[0], interval2);
}

// Tests for add_value()

#[test]
fn add_value_to_empty_set() {
    let mut set = IntervalSet::new();
    set.add_value(10);
    assert!(!set.is_empty());
    assert_eq!(set.count(), 1);
    assert!(set.contains(10));
    let intervals = set.get();
    assert_eq!(intervals[0], Interval::from_boundaries(10, 10));
}

#[test]
fn add_value_isolated() {
    let interval1 = Interval::from_boundaries(1, 5);
    let mut set = IntervalSet::from_vec(vec![interval1]);
    set.add_value(10);
    assert!(!set.is_empty());
    assert!(set.contains(10));
    let intervals = set.get();
    assert_eq!(intervals.len(), 2);
    assert_eq!(intervals[0], interval1);
    assert_eq!(intervals[1], Interval::from_boundaries(10, 10));
}

#[test]
fn add_value_extends_interval() {
    let interval1 = Interval::from_boundaries(1, 5);
    let mut set = IntervalSet::from_vec(vec![interval1]);
    set.add_value(6);
    assert!(!set.is_empty());
    assert!(set.contains(6));
    let intervals = set.get();
    assert_eq!(intervals.len(), 1);
    assert_eq!(intervals[0], Interval::from_boundaries(1, 6));
}

#[test]
fn add_value_already_contained() {
    let interval1 = Interval::from_boundaries(1, 10);
    let mut set = IntervalSet::from_vec(vec![interval1]);
    set.add_value(5);
    assert!(!set.is_empty());
    assert!(set.contains(6));
    let intervals = set.get();
    assert_eq!(intervals.len(), 1);
    assert_eq!(intervals[0], Interval::from_boundaries(1, 10));
}

#[test]
fn add_value_bridges_intervals() {
    let interval1 = Interval::from_boundaries(1, 5);
    let interval2 = Interval::from_boundaries(7, 10);
    let mut set = IntervalSet::from_vec(vec![interval1, interval2]);
    set.add_value(6); // Bridges the gap
    assert!(!set.is_empty());
    assert!(set.contains(6));
    let intervals = set.get();
    assert_eq!(intervals.len(), 1);
    assert_eq!(intervals[0], Interval::from_boundaries(1, 10));
}

// Tests for remove_value()

#[test]
fn remove_value_from_empty_set() {
    let mut set: IntervalSet<i32> = IntervalSet::new();
    set.remove_value(5);
    assert!(set.is_empty());
}

#[test]
fn remove_value_not_in_set() {
    let interval1 = Interval::from_boundaries(1, 5);
    let mut set = IntervalSet::from_vec(vec![interval1]);
    set.remove_value(15);
    assert!(!set.is_empty());
    assert!(!set.contains(15));
    let intervals = set.get();
    assert_eq!(intervals.len(), 1);
    assert_eq!(intervals[0], interval1);
}

#[test]
fn remove_value_single_value_interval() {
    let interval1 = Interval::from_boundaries(5, 5);
    let mut set = IntervalSet::from_vec(vec![interval1]);
    set.remove_value(5);
    assert!(set.is_empty());
}

#[test]
fn remove_value_at_left_boundary() {
    let interval1 = Interval::from_boundaries(5, 10);
    let mut set = IntervalSet::from_vec(vec![interval1]);
    set.remove_value(5);
    assert!(!set.is_empty());
    let intervals = set.get();
    assert_eq!(intervals.len(), 1);
    assert_eq!(intervals[0], Interval::from_boundaries(6, 10));
}

#[test]
fn remove_value_at_right_boundary() {
    let interval1 = Interval::from_boundaries(5, 10);
    let mut set = IntervalSet::from_vec(vec![interval1]);
    set.remove_value(10);
    assert!(!set.is_empty());
    let intervals = set.get();
    assert_eq!(intervals.len(), 1);
    assert_eq!(intervals[0], Interval::from_boundaries(5, 9));
}

#[test]
fn remove_value_in_middle() {
    let interval1 = Interval::from_boundaries(5, 15);
    let mut set = IntervalSet::from_vec(vec![interval1]);
    set.remove_value(10);
    assert!(!set.is_empty());
    let intervals = set.get();
    assert_eq!(intervals.len(), 2);
    assert_eq!(intervals[0], Interval::from_boundaries(5, 9));
    assert_eq!(intervals[1], Interval::from_boundaries(11, 15));
}

#[test]
fn remove_value_multiple_times() {
    let interval1 = Interval::from_boundaries(1, 10);
    let mut set = IntervalSet::from_vec(vec![interval1]);
    set.remove_value(3);
    set.remove_value(7);
    assert!(!set.is_empty());
    let intervals = set.get();
    assert_eq!(intervals.len(), 3);
    assert_eq!(intervals[0], Interval::from_boundaries(1, 2));
    assert_eq!(intervals[1], Interval::from_boundaries(4, 6));
    assert_eq!(intervals[2], Interval::from_boundaries(8, 10));
}

// Tests for remove()

#[test]
fn remove_interval_from_empty_set() {
    let mut set: IntervalSet<i32> = IntervalSet::new();
    let interval = Interval::from_boundaries(5, 10);
    set.remove(&interval);

    assert!(set.is_empty());
    assert_eq!(set.count(), 0);
}

#[test]
fn remove_interval_not_overlapping() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(1, 5));
    let remove_interval = Interval::from_boundaries(10, 15);
    set.remove(&remove_interval);

    assert_eq!(set.count(), 5);
    let intervals = set.get();
    assert_eq!(intervals.len(), 1);
}

#[test]
fn remove_exact_interval() {
    let mut set = IntervalSet::new();
    let interval = Interval::from_boundaries(5, 10);
    let interval_copy = interval.clone();
    set.add(interval);
    set.remove(&interval_copy);

    assert!(set.is_empty());
}

#[test]
fn remove_subsumed_interval() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(1, 20));
    let remove_interval = Interval::from_boundaries(5, 15);
    set.remove(&remove_interval);

    assert_eq!(set.count(), 9); // [1, 4] and [16, 20]
    let intervals = set.get();
    assert_eq!(intervals.len(), 2);
    assert!(set.contains(4));
    assert!(!set.contains(5));
    assert!(!set.contains(15));
    assert!(set.contains(16));
}

#[test]
fn remove_partial_overlap_left() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(5, 15));
    let remove_interval = Interval::from_boundaries(1, 10);
    set.remove(&remove_interval);

    assert_eq!(set.count(), 5); // [11, 15]
    let intervals = set.get();
    assert_eq!(intervals.len(), 1);
    assert!(!set.contains(10));
    assert!(set.contains(11));
}

#[test]
fn remove_partial_overlap_right() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(5, 15));
    let remove_interval = Interval::from_boundaries(10, 20);
    set.remove(&remove_interval);

    assert_eq!(set.count(), 5); // [5, 9]
    let intervals = set.get();
    assert_eq!(intervals.len(), 1);
    assert!(set.contains(9));
    assert!(!set.contains(10));
}

#[test]
fn remove_completely_subsuming() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(5, 10));
    set.add(Interval::from_boundaries(15, 20));
    let remove_interval = Interval::from_boundaries(1, 25);
    set.remove(&remove_interval);

    assert!(set.is_empty());
}

// Tests for join()

#[test]
fn join_empty_sets() {
    let set1: IntervalSet<i32> = IntervalSet::new();
    let set2: IntervalSet<i32> = IntervalSet::new();
    let result = set1.join(&set2);

    assert!(result.is_empty());
}

#[test]
fn join_empty_with_non_empty() {
    let empty_set: IntervalSet<i32> = IntervalSet::new();
    let mut non_empty_set = IntervalSet::new();
    non_empty_set.add(Interval::from_boundaries(5, 10));

    let result1 = empty_set.join(&non_empty_set);
    let result2 = non_empty_set.join(&empty_set);

    assert_eq!(result1.count(), 6);
    assert_eq!(result2.count(), 6);
}

#[test]
fn join_non_overlapping_sets() {
    let mut set1 = IntervalSet::new();
    set1.add(Interval::from_boundaries(1, 5));
    set1.add(Interval::from_boundaries(10, 15));

    let mut set2 = IntervalSet::new();
    set2.add(Interval::from_boundaries(20, 25));
    set2.add(Interval::from_boundaries(30, 35));

    let result = set1.join(&set2);
    assert_eq!(result.count(), 5 + 6 + 6 + 6); // 23 total values
    let intervals = result.get();
    assert_eq!(intervals.len(), 4);
}

#[test]
fn join_overlapping_sets() {
    let mut set1 = IntervalSet::new();
    set1.add(Interval::from_boundaries(1, 10));

    let mut set2 = IntervalSet::new();
    set2.add(Interval::from_boundaries(5, 15));

    let result = set1.join(&set2);
    assert_eq!(result.count(), 15); // Merged to [1, 15]
    let intervals = result.get();
    assert_eq!(intervals.len(), 1);
}

#[test]
fn join_identical_sets() {
    let mut set1 = IntervalSet::new();
    set1.add(Interval::from_boundaries(1, 10));

    let set2 = set1.clone();
    let result = set1.join(&set2);

    assert_eq!(result.count(), 10);
    let intervals = result.get();
    assert_eq!(intervals.len(), 1);
}

// Tests for subsumes_set()

#[test]
fn subsumes_set_empty_sets() {
    let set1: IntervalSet<i32> = IntervalSet::new();
    let set2: IntervalSet<i32> = IntervalSet::new();

    assert!(set1.subsumes_set(&set2));
    assert!(set2.subsumes_set(&set1));
}

#[test]
fn subsumes_set_empty_subset() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(1, 10));
    let empty_set: IntervalSet<i32> = IntervalSet::new();

    assert!(set.subsumes_set(&empty_set));
    assert!(!empty_set.subsumes_set(&set));
}

#[test]
fn subsumes_set_true() {
    let mut superset = IntervalSet::new();
    superset.add(Interval::from_boundaries(1, 20));

    let mut subset = IntervalSet::new();
    subset.add(Interval::from_boundaries(5, 10));
    subset.add(Interval::from_boundaries(15, 18));

    assert!(superset.subsumes_set(&subset));
    assert!(!subset.subsumes_set(&superset));
}

#[test]
fn subsumes_set_false() {
    let mut set1 = IntervalSet::new();
    set1.add(Interval::from_boundaries(1, 10));

    let mut set2 = IntervalSet::new();
    set2.add(Interval::from_boundaries(5, 15));

    assert!(!set1.subsumes_set(&set2));
    assert!(!set2.subsumes_set(&set1));
}

#[test]
fn subsumes_set_identical() {
    let mut set1 = IntervalSet::new();
    set1.add(Interval::from_boundaries(1, 10));

    let set2 = set1.clone();

    assert!(set1.subsumes_set(&set2));
    assert!(set2.subsumes_set(&set1));
}

// Tests for subsumes_interval()

#[test]
fn subsumes_interval_empty_set() {
    let set: IntervalSet<i32> = IntervalSet::new();
    let interval = Interval::from_boundaries(5, 10);

    assert!(!set.subsumes_interval(&interval));
}

#[test]
fn subsumes_interval_true() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(1, 20));
    let interval = Interval::from_boundaries(5, 15);

    assert!(set.subsumes_interval(&interval));
}

#[test]
fn subsumes_interval_false() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(1, 10));
    let interval = Interval::from_boundaries(5, 15);

    assert!(!set.subsumes_interval(&interval));
}

#[test]
fn subsumes_interval_multiple_intervals() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(1, 5));
    set.add(Interval::from_boundaries(10, 15));
    let interval = Interval::from_boundaries(2, 4);

    assert!(set.subsumes_interval(&interval));
}

// Tests for overlaps_set()

#[test]
fn overlaps_set_empty_sets() {
    let set1: IntervalSet<i32> = IntervalSet::new();
    let set2: IntervalSet<i32> = IntervalSet::new();

    assert!(!set1.overlaps_set(&set2));
}

#[test]
fn overlaps_set_one_empty() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(1, 10));
    let empty_set: IntervalSet<i32> = IntervalSet::new();

    assert!(!set.overlaps_set(&empty_set));
    assert!(!empty_set.overlaps_set(&set));
}

#[test]
fn overlaps_set_true() {
    let mut set1 = IntervalSet::new();
    set1.add(Interval::from_boundaries(1, 10));

    let mut set2 = IntervalSet::new();
    set2.add(Interval::from_boundaries(5, 15));

    assert!(set1.overlaps_set(&set2));
    assert!(set2.overlaps_set(&set1));
}

#[test]
fn overlaps_set_false() {
    let mut set1 = IntervalSet::new();
    set1.add(Interval::from_boundaries(1, 5));

    let mut set2 = IntervalSet::new();
    set2.add(Interval::from_boundaries(10, 15));

    assert!(!set1.overlaps_set(&set2));
    assert!(!set2.overlaps_set(&set1));
}

#[test]
fn overlaps_set_touching() {
    let mut set1 = IntervalSet::new();
    set1.add(Interval::from_boundaries(1, 5));

    let mut set2 = IntervalSet::new();
    set2.add(Interval::from_boundaries(5, 10));

    assert!(set1.overlaps_set(&set2));
    assert!(set2.overlaps_set(&set1));
}

// Tests for overlaps_interval()

#[test]
fn overlaps_interval_empty_set() {
    let set: IntervalSet<i32> = IntervalSet::new();
    let interval = Interval::from_boundaries(5, 10);

    assert!(!set.overlaps_interval(&interval));
}

#[test]
fn overlaps_interval_true() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(1, 10));
    let interval = Interval::from_boundaries(5, 15);

    assert!(set.overlaps_interval(&interval));
}

#[test]
fn overlaps_interval_false() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(1, 5));
    let interval = Interval::from_boundaries(10, 15);

    assert!(!set.overlaps_interval(&interval));
}

// Tests for contains()

#[test]
fn contains_empty_set() {
    let set: IntervalSet<i32> = IntervalSet::new();
    assert!(!set.contains(5));
}

#[test]
fn contains_true() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(1, 10));
    set.add(Interval::from_boundaries(20, 30));

    assert!(set.contains(5));
    assert!(set.contains(1));
    assert!(set.contains(10));
    assert!(set.contains(25));
}

#[test]
fn contains_false() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(1, 10));
    set.add(Interval::from_boundaries(20, 30));

    assert!(!set.contains(15));
    assert!(!set.contains(0));
    assert!(!set.contains(35));
}

// Tests for count()

#[test]
fn count_empty_set() {
    let set: IntervalSet<i32> = IntervalSet::new();
    assert_eq!(set.count(), 0);
}

#[test]
fn count_single_interval() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(5, 10));
    assert_eq!(set.count(), 6);
}

#[test]
fn count_multiple_intervals() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(1, 5));
    set.add(Interval::from_boundaries(10, 15));
    set.add(Interval::from_boundaries(20, 25));

    assert_eq!(set.count(), 5 + 6 + 6); // 17 total values
}

#[test]
fn count_after_merge() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(1, 5));
    set.add(Interval::from_boundaries(6, 10)); // Contiguous, should merge

    assert_eq!(set.count(), 10);
}

// Tests for extract()

#[test]
fn extract_empty_set() {
    let set: IntervalSet<i32> = IntervalSet::new();
    let result = set.extract(5, 15);
    assert!(result.is_empty());
}

#[test]
fn extract_no_overlap() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(1, 5));
    set.add(Interval::from_boundaries(20, 25));

    let result = set.extract(10, 15);
    assert!(result.is_empty());
}

#[test]
fn extract_partial_overlap() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(1, 20));

    let result = set.extract(5, 15);
    assert_eq!(result.count(), 11); // [5, 15]
    assert!(result.contains(5));
    assert!(result.contains(15));
    assert!(!result.contains(4));
    assert!(!result.contains(16));
}

#[test]
fn extract_multiple_intervals() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(1, 10));
    set.add(Interval::from_boundaries(15, 25));
    set.add(Interval::from_boundaries(30, 40));

    let result = set.extract(5, 35);
    assert_eq!(result.count(), 6 + 11 + 6); // [5, 10], [15, 25], [30, 35]
    let intervals = result.get();
    assert_eq!(intervals.len(), 3);
}

#[test]
fn extract_exact_boundaries() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(5, 15));

    let result = set.extract(5, 15);
    assert_eq!(result.count(), 11);
    assert!(result.contains(5));
    assert!(result.contains(15));
}

// Tests for get_interval_for()

#[test]
fn get_interval_for_empty_set() {
    let set: IntervalSet<i32> = IntervalSet::new();
    assert!(set.get_interval_for(5).is_none());
}

#[test]
fn get_interval_for_not_contained() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(1, 10));
    set.add(Interval::from_boundaries(20, 30));

    assert!(set.get_interval_for(15).is_none());
}

#[test]
fn get_interval_for_contained() {
    let mut set = IntervalSet::new();
    let interval1 = Interval::from_boundaries(1, 10);
    let interval2 = Interval::from_boundaries(20, 30);
    set.add(interval1);
    set.add(interval2);

    assert_eq!(set.get_interval_for(5), Some(interval1));
    assert_eq!(set.get_interval_for(25), Some(interval2));
    assert_eq!(set.get_interval_for(1), Some(interval1));
    assert_eq!(set.get_interval_for(30), Some(interval2));
}

// Tests for intersect()

#[test]
fn intersect_empty_set() {
    let set: IntervalSet<i32> = IntervalSet::new();
    let interval = Interval::from_boundaries(5, 15);
    let result = set.intersect(&interval);

    assert!(result.is_empty());
}

#[test]
fn intersect_no_overlap() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(1, 5));
    let interval = Interval::from_boundaries(10, 15);
    let result = set.intersect(&interval);

    assert!(result.is_empty());
}

#[test]
fn intersect_partial_overlap() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(1, 10));
    set.add(Interval::from_boundaries(20, 30));
    let interval = Interval::from_boundaries(5, 25);
    let result = set.intersect(&interval);

    assert_eq!(result.count(), 6 + 6); // [5, 10] and [20, 25]
    let intervals = result.get();
    assert_eq!(intervals.len(), 2);
}

#[test]
fn intersect_complete_overlap() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(5, 15));
    let interval = Interval::from_boundaries(1, 20);
    let result = set.intersect(&interval);

    assert_eq!(result.count(), 11); // [5, 15]
    let intervals = result.get();
    assert_eq!(intervals.len(), 1);
}

// Tests for is_empty()

#[test]
fn is_empty_new_set() {
    let set: IntervalSet<i32> = IntervalSet::new();
    assert!(set.is_empty());
}

#[test]
fn is_empty_after_add() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(5, 10));
    assert!(!set.is_empty());
}

#[test]
fn is_empty_after_remove_all() {
    let mut set = IntervalSet::new();
    let interval = Interval::from_boundaries(5, 10);
    set.add(interval);
    set.remove(&interval);
    assert!(set.is_empty());
}

// Tests for iter()

#[test]
fn iter_empty_set() {
    let set: IntervalSet<i32> = IntervalSet::new();
    let count = set.iter().count();
    assert_eq!(count, 0);
}

#[test]
fn iter_single_interval() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(5, 10));
    let count = set.iter().count();
    assert_eq!(count, 1);
}

#[test]
fn iter_multiple_intervals() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(1, 5));
    set.add(Interval::from_boundaries(10, 15));
    set.add(Interval::from_boundaries(20, 25));

    let count = set.iter().count();
    assert_eq!(count, 3);

    let intervals: Vec<_> = set.iter().collect();
    assert_eq!(intervals.len(), 3);
}

// Tests with different numeric types

#[test]
fn test_i64_type() {
    let mut set: IntervalSet<i64> = IntervalSet::new();
    set.add(Interval::from_boundaries(-1000000i64, 1000000i64));

    assert_eq!(set.count(), 2000001);
    assert!(set.contains(0i64));
    assert!(set.contains(-500000i64));
    assert!(set.contains(500000i64));
}

// Edge cases

#[test]
fn edge_case_single_value_intervals() {
    let mut set = IntervalSet::new();
    set.add_value(5);
    set.add_value(7);
    set.add_value(6); // Should merge all three

    assert_eq!(set.count(), 3); // [5, 7]
    let intervals = set.get();
    assert_eq!(intervals.len(), 1);
}

#[test]
fn edge_case_large_gaps() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(1, 2));
    set.add(Interval::from_boundaries(1000000, 1000001));

    assert_eq!(set.count(), 4);
    let intervals = set.get();
    assert_eq!(intervals.len(), 2);
}

#[test]
fn edge_case_maximum_boundaries() {
    let mut set: IntervalSet<i8> = IntervalSet::new();
    set.add(Interval::from_boundaries(i8::MIN + 1i8, i8::MAX));

    assert_eq!(set.count(), 255);
    assert!(set.contains(0i8));
    assert!(set.contains(i8::MIN + 1i8));
    assert!(set.contains(i8::MAX));
}

#[test]
fn edge_case_zero_boundaries() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(-5, 5));
    set.remove_value(0);

    assert_eq!(set.count(), 10); // [-5, -1] and [1, 5]
    let intervals = set.get();
    assert_eq!(intervals.len(), 2);
    assert!(!set.contains(0));
    assert!(set.contains(-1));
    assert!(set.contains(1));
}

// Tests for Default trait

#[test]
fn default_creates_empty_set() {
    let set: IntervalSet<i32> = IntervalSet::default();
    assert!(set.is_empty());
    assert_eq!(set.count(), 0);
}

// Tests for Display trait

#[test]
fn display_empty_set() {
    let set: IntervalSet<i32> = IntervalSet::new();
    let display_str = format!("{}", set);
    assert_eq!(display_str, "");
}

#[test]
fn display_single_interval() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(5, 10));
    let display_str = format!("{}", set);
    assert_eq!(display_str, "[5, 10]");
}

#[test]
fn display_multiple_intervals() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(1, 3));
    set.add(Interval::from_boundaries(7, 9));
    let display_str = format!("{}", set);
    // The order depends on the hash set iteration, but should contain both intervals
    assert!(display_str.contains("[1, 3]"));
    assert!(display_str.contains("[7, 9]"));
    assert!(display_str.contains(","));
}

// Tests for Clone and PartialEq traits

#[test]
fn clone_and_equality() {
    let mut set1 = IntervalSet::new();
    set1.add(Interval::from_boundaries(1, 10));
    set1.add(Interval::from_boundaries(20, 30));

    let set2 = set1.clone();
    assert_eq!(set1, set2);

    let mut set3 = IntervalSet::new();
    set3.add(Interval::from_boundaries(1, 10));
    assert_ne!(set1, set3);
}

// Stress tests

#[test]
fn stress_test_many_additions() {
    let mut set = IntervalSet::new();
    for i in 0..100 {
        set.add_value(i * 10); // Add isolated values
    }
    assert_eq!(set.count(), 100);
    let intervals = set.get();
    assert_eq!(intervals.len(), 100);
}

#[test]
fn stress_test_merging() {
    let mut set = IntervalSet::new();
    // Add overlapping intervals that should all merge
    for i in 0..50 {
        set.add(Interval::from_boundaries(i, i + 10));
    }
    // Should merge into one large interval
    let intervals = set.get();
    assert_eq!(intervals.len(), 1);
    assert_eq!(set.count(), 60); // [0, 59]
}

// Tests for fmt

#[test]
fn fmt_empty_set() {
    let set: IntervalSet<i32> = IntervalSet::new();
    let formatted = format!("{}", set);
    assert_eq!(formatted, "");
}

#[test]
fn fmt_single_interval() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(5, 10));
    let formatted = format!("{}", set);
    assert_eq!(formatted, "[5, 10]");
}

#[test]
fn fmt_multiple_intervals() {
    let mut set = IntervalSet::new();
    set.add(Interval::from_boundaries(1, 3));
    set.add(Interval::from_boundaries(7, 9));
    let formatted = format!("{}", set);
    assert_eq!(formatted, "[1, 3],[7, 9]");
}
