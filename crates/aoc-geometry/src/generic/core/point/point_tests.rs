use super::*;
use crate::Vector;
use assertables::{assert_all, assert_none, assert_some};
use pretty_assertions::assert_eq;

const DIMENSIONS: usize = 3;

fn p(x: i32, y: i32, z: i32) -> Point<i32, DIMENSIONS> {
    Point::<i32, DIMENSIONS>::new([x, y, z])
}

// Tests for origin

#[test]
fn origin_i8() {
    let point = Point::<i8, DIMENSIONS>::origin();
    assert_eq!(point.get(0), &0);
    assert_eq!(point.get(1), &0);
    assert_eq!(point.get(2), &0);
    assert_eq!(point.get_coordinates(), &[0, 0, 0]);
    assert!(point.is_origin());
}

#[test]
fn origin_i16() {
    let point = Point::<i16, DIMENSIONS>::origin();
    assert_eq!(point.get(0), &0);
    assert_eq!(point.get(1), &0);
    assert_eq!(point.get(2), &0);
    assert_eq!(point.get_coordinates(), &[0, 0, 0]);
    assert!(point.is_origin());
}

#[test]
fn origin_i32() {
    let point = Point::<i32, DIMENSIONS>::origin();
    assert_eq!(point.get(0), &0);
    assert_eq!(point.get(1), &0);
    assert_eq!(point.get(2), &0);
    assert_eq!(point.get_coordinates(), &[0, 0, 0]);
    assert!(point.is_origin());
}

#[test]
fn origin_i64() {
    let point = Point::<i64, DIMENSIONS>::origin();
    assert_eq!(point.get(0), &0);
    assert_eq!(point.get(1), &0);
    assert_eq!(point.get(2), &0);
    assert_eq!(point.get_coordinates(), &[0, 0, 0]);
    assert!(point.is_origin());
}

// Tests for new

#[test]
fn new_i8() {
    let point = Point::<i8, DIMENSIONS>::new([1, 2, 3]);
    assert_eq!(point.get(0), &1);
    assert_eq!(point.get(1), &2);
    assert_eq!(point.get(2), &3);
    assert_eq!(point.get_coordinates(), &[1, 2, 3]);
    assert!(!point.is_origin());
}

#[test]
fn new_i16() {
    let point = Point::<i16, DIMENSIONS>::new([1, 2, 3]);
    assert_eq!(point.get(0), &1);
    assert_eq!(point.get(1), &2);
    assert_eq!(point.get(2), &3);
    assert_eq!(point.get_coordinates(), &[1, 2, 3]);
    assert!(!point.is_origin());
}

#[test]
fn new_i32() {
    let point = Point::<i32, DIMENSIONS>::new([1, 2, 3]);
    assert_eq!(point.get(0), &1);
    assert_eq!(point.get(1), &2);
    assert_eq!(point.get(2), &3);
    assert_eq!(point.get_coordinates(), &[1, 2, 3]);
    assert!(!point.is_origin());
}

#[test]
fn new_i64() {
    let point = Point::<i64, DIMENSIONS>::new([1, 2, 3]);
    assert_eq!(point.get(0), &1);
    assert_eq!(point.get(1), &2);
    assert_eq!(point.get(2), &3);
    assert_eq!(point.get_coordinates(), &[1, 2, 3]);
    assert!(!point.is_origin());
}

#[test]
fn new_with_zeros() {
    let point = Point::<i32, DIMENSIONS>::new([0, 0, 0]);
    assert_eq!(point.get(0), &0);
    assert_eq!(point.get(1), &0);
    assert_eq!(point.get(2), &0);
    assert_eq!(point.get_coordinates(), &[0, 0, 0]);
    assert!(point.is_origin());
}

#[test]
fn new_with_negative_values() {
    let point = Point::<i32, DIMENSIONS>::new([-1, -2, -3]);
    assert_eq!(point.get(0), &-1);
    assert_eq!(point.get(1), &-2);
    assert_eq!(point.get(2), &-3);
    assert_eq!(point.get_coordinates(), &[-1, -2, -3]);
    assert!(!point.is_origin());
}

#[test]
fn new_with_mixed_values() {
    let point = Point::<i32, DIMENSIONS>::new([0, -5, 3]);
    assert_eq!(point.get(0), &0);
    assert_eq!(point.get(1), &-5);
    assert_eq!(point.get(2), &3);
    assert_eq!(point.get_coordinates(), &[0, -5, 3]);
    assert!(!point.is_origin());
}

#[test]
fn new_with_extreme_values() {
    let point = Point::<i32, DIMENSIONS>::new([i32::MIN, 0, i32::MAX]);
    assert_eq!(*point.get_coordinates(), [i32::MIN, 0, i32::MAX,]);
    assert!(!point.is_origin());
}

// Tests for extremes

#[test]
fn extremes_all_positive() {
    let point = Point::<i32, DIMENSIONS>::extremes([AxisDirection::Positive; DIMENSIONS]);
    assert!(!point.is_origin());
    assert_all!(point.get_coordinates().iter(), |x| x == &i32::MAX);
}

#[test]
fn extremes_all_negative() {
    let point = Point::<i32, DIMENSIONS>::extremes([AxisDirection::Negative; DIMENSIONS]);
    assert!(!point.is_origin());
    assert_all!(point.get_coordinates().iter(), |x| x == &i32::MIN);
}

#[test]
fn extremes_mixed_values() {
    let point = Point::<i32, DIMENSIONS>::extremes([
        AxisDirection::Positive,
        AxisDirection::Negative,
        AxisDirection::Positive,
    ]);
    assert!(!point.is_origin());
    assert_eq!(point.get(0), &i32::MAX);
    assert_eq!(point.get(1), &i32::MIN);
    assert_eq!(point.get(2), &i32::MAX);
}

// Tests for get_neighbors

#[test]
fn get_neighbors_positive_coordinates() {
    let point = p(1, 1, 1);
    let neighbours = point.get_neighbors();

    assert_eq!(neighbours.len(), 6);
    assert!(neighbours.contains(&p(2, 1, 1)));
    assert!(neighbours.contains(&p(0, 1, 1)));
    assert!(neighbours.contains(&p(1, 2, 1)));
    assert!(neighbours.contains(&p(1, 0, 1)));
    assert!(neighbours.contains(&p(1, 1, 2)));
    assert!(neighbours.contains(&p(1, 1, 0)));
}

#[test]
fn get_neighbors_mixed_coordinates() {
    let point = p(-1, 0, 2);
    let neighbours = point.get_neighbors();

    assert_eq!(neighbours.len(), 6);
    assert!(neighbours.contains(&p(0, 0, 2)));
    assert!(neighbours.contains(&p(-2, 0, 2)));
    assert!(neighbours.contains(&p(-1, 1, 2)));
    assert!(neighbours.contains(&p(-1, -1, 2)));
    assert!(neighbours.contains(&p(-1, 0, 3)));
    assert!(neighbours.contains(&p(-1, 0, 1)));
}

#[test]
fn get_neighbors_space_boundaries() {
    let point = p(i32::MAX, 0, 2);
    let neighbours = point.get_neighbors();

    assert_eq!(neighbours.len(), 5);
    assert!(neighbours.contains(&p(i32::MAX - 1, 0, 2)));
    assert!(neighbours.contains(&p(i32::MAX, 1, 2)));
    assert!(neighbours.contains(&p(i32::MAX, -1, 2)));
    assert!(neighbours.contains(&p(i32::MAX, 0, 3)));
    assert!(neighbours.contains(&p(i32::MAX, 0, 1)));
}

// Tests for Index trait

#[test]
fn index_operator() {
    let point = p(10, 20, 30);
    assert_eq!(point[0], 10);
    assert_eq!(point[1], 20);
    assert_eq!(point[2], 30);
}

#[test]
fn index_operator_negative_values() {
    let point = p(-10, -20, 0);
    assert_eq!(point[0], -10);
    assert_eq!(point[1], -20);
    assert_eq!(point[2], 0);
}

// Tests for Display trait

#[test]
fn display_positive_values() {
    let point = p(1, 2, 3);
    assert_eq!(format!("{}", point), "(1,2,3)");
}

#[test]
fn display_negative_values() {
    let point_mixed = p(-1, 0, 3);
    assert_eq!(format!("{}", point_mixed), "(-1,0,3)");
}

#[test]
fn display_zeros() {
    let point_zero = p(0, 0, 0);
    assert_eq!(format!("{}", point_zero), "(0,0,0)");
}

// Tests for mirror

#[test]
fn mirror_point_respect_origin() {
    let point = p(1, -2, 3);
    let other = Point::<i32, DIMENSIONS>::origin();
    let mirrored_opt = point.mirror(&other);
    assert_some!(mirrored_opt);
    let mirrored = mirrored_opt.unwrap();
    assert_eq!(mirrored.get(0), &-1);
    assert_eq!(mirrored.get(1), &2);
    assert_eq!(mirrored.get(2), &-3);
}

#[test]
fn mirror_point_respect_other() {
    let point = p(3, 3, 3);
    let other = p(2, 2, 2);
    let mirrored_opt = point.mirror(&other);
    assert_some!(mirrored_opt);
    let mirrored = mirrored_opt.unwrap();
    assert_eq!(mirrored.get(0), &1);
    assert_eq!(mirrored.get(1), &1);
    assert_eq!(mirrored.get(2), &1);
}

#[test]
fn mirror_point_respect_itself() {
    let point = p(1, -2, 3);
    let other = point.clone();
    let mirrored_opt = point.mirror(&other);
    assert_some!(mirrored_opt);
    let mirrored = mirrored_opt.unwrap();
    assert_eq!(mirrored.get(0), point.get(0));
    assert_eq!(mirrored.get(1), point.get(1));
    assert_eq!(mirrored.get(2), point.get(2));
}

#[test]
fn mirror_origin_respect_itself() {
    let point = Point::<i32, DIMENSIONS>::origin();
    let other = point.clone();
    let mirrored_opt = point.mirror(&other);
    assert_some!(mirrored_opt);
    let mirrored = mirrored_opt.unwrap();
    assert_eq!(mirrored.get(0), point.get(0));
    assert_eq!(mirrored.get(1), point.get(1));
    assert_eq!(mirrored.get(2), point.get(2));
}

#[test]
fn mirror_out_of_bounds() {
    let point = p(i32::MAX, 0, 0);
    let other = p(-20, 10, -70);
    let mirrored_opt = point.mirror(&other);
    assert_none!(mirrored_opt);
}

// Tests for move_by

#[test]
fn move_by_positive_values() {
    let point = p(1, 2, 3);
    let vector = Vector::<i16, DIMENSIONS>::new([10, 20, 30]);
    let result_opt = point.move_by(&vector);
    assert_some!(result_opt);
    let result = result_opt.unwrap();
    assert_eq!(result.get(0), &11);
    assert_eq!(result.get(1), &22);
    assert_eq!(result.get(2), &33);
}

#[test]
fn move_by_negative_values() {
    let point = p(-1, 0, 2);
    let vector = Vector::<i16, DIMENSIONS>::new([1, -5, -2]);
    let result_opt = point.move_by(&vector);
    assert_some!(result_opt);
    let result = result_opt.unwrap();
    assert_eq!(result.get(0), &0);
    assert_eq!(result.get(1), &-5);
    assert_eq!(result.get(2), &0);
}

#[test]
fn move_by_zero() {
    let point = p(-1, 2, -3);
    let vector = Vector::<i16, DIMENSIONS>::zero();
    let result_opt = point.move_by(&vector);
    assert_some!(result_opt);
    let result = result_opt.unwrap();
    assert_eq!(result.get(0), &-1);
    assert_eq!(result.get(1), &2);
    assert_eq!(result.get(2), &-3);
}

#[test]
fn move_by_out_of_bounds() {
    let point = p(100, -200, -300);
    let vector = Vector::<i64, DIMENSIONS>::new([i64::MAX, i64::MAX, i64::MIN]);
    let result_opt = point.move_by(&vector);
    assert_none!(result_opt);
}

// Tests for is_in

#[test]
fn is_in_matching_axis_and_value() {
    let point = p(5, -3, 7);
    assert!(point.is_in(0, 5));
    assert!(point.is_in(1, -3));
    assert!(point.is_in(2, 7));
}

#[test]
fn is_in_non_matching_value() {
    let point = p(1, 2, 3);
    assert!(!point.is_in(0, 2));
    assert!(!point.is_in(1, 3));
    assert!(!point.is_in(2, 1));
}

#[test]
#[should_panic(expected = "Axis index out of bounds")]
fn is_in_out_of_bounds_axis_panics() {
    let point = p(1, 2, 3);
    let _result = point.is_in(3, 0);
}
