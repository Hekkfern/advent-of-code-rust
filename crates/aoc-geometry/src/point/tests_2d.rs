/*
use super::*;
use crate::vector::Vector;
use pretty_assertions::{assert_eq, assert_ne};

// Tests for origin

#[test]
fn point_origin_2d() {
    let origin: Point<i32, 2> = Point::origin();
    assert_eq!(origin.get(0), &0);
    assert_eq!(origin.get(1), &0);
    assert_eq!(origin.get_coordinates(), &[0, 0]);
}

#[test]
fn point_origin_different_types_2d() {
    let origin_i8: Point<i8, 2> = Point::origin();
    assert_eq!(origin_i8.get_coordinates(), &[0i8, 0i8]);

    let origin_i64: Point<i64, 2> = Point::origin();
    assert_eq!(origin_i64.get_coordinates(), &[0i64, 0i64]);
}

// Tests for new

#[test]
fn point_new_2d() {
    let point = Point::new(&[3, 4]);
    assert_eq!(point.get(0), &3);
    assert_eq!(point.get(1), &4);
    assert_eq!(point.get_coordinates(), &[3, 4]);
}

#[test]
fn point_new_different_types_2d() {
    let point_i8 = Point::new(&[1i8, 2i8]);
    assert_eq!(point_i8.get(0), &1i8);

    let point_i16 = Point::new(&[10i16, 20i16]);
    assert_eq!(point_i16.get(0), &10i16);

    let point_i64 = Point::new(&[100i64, 200i64]);
    assert_eq!(point_i64.get(0), &100i64);
}

#[test]
fn point_new_with_zeros_2d() {
    let point_2d = Point::new(&[0, 0]);
    assert_eq!(point_2d.get(0), &0);
    assert_eq!(point_2d.get(1), &0);
    assert_eq!(point_2d.get_coordinates(), &[0, 0]);
}

#[test]
fn point_new_with_negative_values_2d() {
    let point_2d = Point::new(&[-5, -10]);
    assert_eq!(point_2d.get(0), &-5);
    assert_eq!(point_2d.get(1), &-10);
    assert_eq!(point_2d.get_coordinates(), &[-5, -10]);
}

#[test]
fn point_new_with_mixed_values_2d() {
    let point_2d = Point::new(&[-5, 10]);
    assert_eq!(point_2d.get(0), &-5);
    assert_eq!(point_2d.get(1), &10);
}

#[test]
fn point_new_with_extreme_values_2d() {
    let point_max = Point::new(&[i32::MAX, i32::MIN + 1]);
    assert_eq!(*point_max.get(0), i32::MAX);
    assert_eq!(*point_max.get(1), i32::MIN + 1);
}

// Tests for get_neighbours

#[test]
fn point_get_neighbours_2d() {
    let point = Point::new(&[0, 0]);
    let neighbours = point.get_neighbours();

    assert_eq!(neighbours.len(), 4);
    assert!(neighbours.contains(&Point::new(&[1, 0])));
    assert!(neighbours.contains(&Point::new(&[-1, 0])));
    assert!(neighbours.contains(&Point::new(&[0, 1])));
    assert!(neighbours.contains(&Point::new(&[0, -1])));
}

#[test]
fn point_get_neighbours_negative_coordinates_2d() {
    let point = Point::new(&[-5, -10]);
    let neighbours = point.get_neighbours();

    assert_eq!(neighbours.len(), 4);
    assert!(neighbours.contains(&Point::new(&[-4, -10])));
    assert!(neighbours.contains(&Point::new(&[-6, -10])));
    assert!(neighbours.contains(&Point::new(&[-5, -9])));
    assert!(neighbours.contains(&Point::new(&[-5, -11])));
}

#[test]
fn point_get_neighbours_zero_coordinates_2d() {
    let point = Point::new(&[0, 0]);
    let neighbours = point.get_neighbours();

    assert_eq!(neighbours.len(), 4);
    assert!(neighbours.contains(&Point::new(&[1, 0])));
    assert!(neighbours.contains(&Point::new(&[-1, 0])));
    assert!(neighbours.contains(&Point::new(&[0, 1])));
    assert!(neighbours.contains(&Point::new(&[0, -1])));
}

// Tests for Index trait

#[test]
fn point_index_operator_2d() {
    let point = Point::new(&[10, 20]);
    assert_eq!(point[0], 10);
    assert_eq!(point[1], 20);
}

#[test]
fn point_index_operator_negative_values_2d() {
    let point = Point::new(&[-10, -20]);
    assert_eq!(point[0], -10);
    assert_eq!(point[1], -20);
}

// Tests for Display trait

#[test]
fn point_display_2d() {
    let point_2d = Point::new(&[3, 4]);
    assert_eq!(format!("{}", point_2d), "(3,4)");
}

#[test]
fn point_display_negative_values_2d() {
    let point_neg = Point::new(&[-5, -10]);
    assert_eq!(format!("{}", point_neg), "(-5,-10)");
}

#[test]
fn point_display_zeros_2d() {
    let point_zero = Point::new(&[0, 0]);
    assert_eq!(format!("{}", point_zero), "(0,0)");
}

// Tests for Neg trait

#[test]
fn point_negation_2d() {
    let point_2d = Point::new(&[3, -4]);
    let negated = -point_2d;
    assert_eq!(negated.get(0), &-3);
    assert_eq!(negated.get(1), &4);
}

#[test]
fn point_negation_zeros_2d() {
    let point_zero = Point::new(&[0, 0]);
    let negated = -point_zero;
    assert_eq!(negated.get_coordinates(), &[0, 0]);
}

#[test]
fn point_negation_extreme_values_2d() {
    let point_max = Point::new(&[i32::MAX, i32::MIN + 1]); // MIN + 1 to avoid overflow
    let negated = -point_max;
    assert_eq!(negated.get(0), &(-i32::MAX));
    assert_eq!(negated.get(1), &(-(i32::MIN + 1)));
}

// Tests for Add trait (point + vector)

#[test]
fn point_add_vector_2d() {
    let point = Point::new(&[1, 2]);
    let vector = Vector::new(&[3, 4]);
    let result = point + vector;

    assert_eq!(result.get(0), &4);
    assert_eq!(result.get(1), &6);
}

#[test]
fn point_add_vector_negative_values_2d() {
    let point = Point::new(&[-5, -10]);
    let vector = Vector::new(&[-2, -3]);
    let result = point + vector;

    assert_eq!(result.get_coordinates(), &[-7, -13]);
}

#[test]
fn point_add_vector_zero_vector_2d() {
    let point = Point::new(&[5, 8]);
    let zero_vector = Vector::new(&[0, 0]);
    let result = point + zero_vector;

    assert_eq!(result.get_coordinates(), &[5, 8]);
}

// Tests for Sub trait (point - vector)

#[test]
fn point_sub_vector_2d() {
    let point = Point::new(&[5, 8]);
    let vector = Vector::new(&[2, 3]);
    let result = point - vector;

    assert_eq!(result.get(0), &3);
    assert_eq!(result.get(1), &5);
}

#[test]
fn point_sub_vector_negative_values_2d() {
    let point = Point::new(&[-5, -10]);
    let vector = Vector::new(&[-2, -3]);
    let result = point - vector;

    assert_eq!(result.get_coordinates(), &[-3, -7]);
}

#[test]
fn point_sub_vector_zero_vector_2d() {
    let point = Point::new(&[5, 8]);
    let zero_vector = Vector::new(&[0, 0]);
    let result = point - zero_vector;

    assert_eq!(result.get_coordinates(), &[5, 8]);
}

#[test]
fn point_sub_vector_resulting_in_zero_2d() {
    let point = Point::new(&[5, 8]);
    let vector = Vector::new(&[5, 8]);
    let result = point - vector;

    assert_eq!(result.get_coordinates(), &[0, 0]);
}

// Tests for PartialEq trait

#[test]
fn point_equality_2d() {
    let point1 = Point::new(&[1, 2]);
    let point2 = Point::new(&[1, 2]);
    let point3 = Point::new(&[2, 1]);

    assert_eq!(point1, point2);
    assert_ne!(point1, point3);
}

#[test]
fn point_equality_negative_values_2d() {
    let point1 = Point::new(&[-5, -10]);
    let point2 = Point::new(&[-5, -10]);
    let point3 = Point::new(&[-10, -5]);

    assert_eq!(point1, point2);
    assert_ne!(point1, point3);
}

#[test]
fn point_equality_zeros_2d() {
    let point1 = Point::new(&[0, 0]);
    let point2 = Point::new(&[0, 0]);
    let origin = Point::origin();

    assert_eq!(point1, point2);
    assert_eq!(point1, origin);
}

// Tests for Clone and Copy traits

#[test]
fn point_clone_copy_2d() {
    let point = Point::new(&[1, 2]);
    let cloned = point.clone();
    let copied = point;

    assert_eq!(point, cloned);
    assert_eq!(point, copied);
}

#[test]
fn point_clone_copy_negative_values_2d() {
    let point = Point::new(&[-1, -2]);
    let cloned = point.clone();
    let copied = point;

    assert_eq!(point, cloned);
    assert_eq!(point, copied);
    assert_eq!(cloned.get_coordinates(), &[-1, -2]);
}

// Tests with negative coordinates

#[test]
fn point_with_negative_coordinates_2d() {
    let point = Point::new(&[-5, -10]);
    assert_eq!(point.get(0), &-5);
    assert_eq!(point.get(1), &-10);

    let neighbours = point.get_neighbours();
    assert!(neighbours.contains(&Point::new(&[-4, -10])));
    assert!(neighbours.contains(&Point::new(&[-6, -10])));
    assert!(neighbours.contains(&Point::new(&[-5, -9])));
    assert!(neighbours.contains(&Point::new(&[-5, -11])));
}

// Tests for Hash trait

#[test]
fn point_hash_2d() {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    let point1 = Point::new(&[1, 2]);
    let point2 = Point::new(&[1, 2]);
    let point3 = Point::new(&[2, 1]);

    set.insert(point1);
    set.insert(point2);
    set.insert(point3);

    assert_eq!(set.len(), 2); // point1 and point2 are the same
}

#[test]
fn point_hash_negative_values_2d() {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    let point1 = Point::new(&[-1, -2]);
    let point2 = Point::new(&[-1, -2]);
    let point3 = Point::new(&[1, 2]);
    let point4 = Point::new(&[0, 0]);

    set.insert(point1);
    set.insert(point2);
    set.insert(point3);
    set.insert(point4);

    assert_eq!(set.len(), 3); // point1 and point2 are the same
}

// Tests for Debug trait

#[test]
fn point_debug_2d() {
    let point = Point::new(&[1, 2]);
    let debug_str = format!("{:?}", point);
    assert!(debug_str.contains("Point"));
    assert!(debug_str.contains("1"));
    assert!(debug_str.contains("2"));
}

#[test]
fn point_debug_negative_values_2d() {
    let point = Point::new(&[-1, -2]);
    let debug_str = format!("{:?}", point);
    assert!(debug_str.contains("Point"));
    assert!(debug_str.contains("-1"));
    assert!(debug_str.contains("-2"));
}
*/
