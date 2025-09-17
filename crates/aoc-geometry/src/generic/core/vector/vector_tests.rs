use super::*;
use crate::Point;
use assertables::{assert_none, assert_some};
use pretty_assertions::assert_eq;
use std::ops::{Add, Mul, Neg, Sub};

const DIMENSIONS: usize = 3;

fn v(x: i32, y: i32, z: i32) -> Vector<i32, DIMENSIONS> {
    Vector::<i32, DIMENSIONS>::new([x, y, z])
}

// Tests for zero

#[test]
fn zero_i8() {
    let vector = Vector::<i8, DIMENSIONS>::zero();
    assert_eq!(vector.get(0), &0);
    assert_eq!(vector.get(1), &0);
    assert_eq!(vector.get(2), &0);
    assert_eq!(vector.get_coordinates(), &[0, 0, 0]);
}

#[test]
fn zero_i16() {
    let vector = Vector::<i16, DIMENSIONS>::zero();
    assert_eq!(vector.get(0), &0);
    assert_eq!(vector.get(1), &0);
    assert_eq!(vector.get(2), &0);
    assert_eq!(vector.get_coordinates(), &[0, 0, 0]);
}

#[test]
fn zero_i32() {
    let vector = Vector::<i32, DIMENSIONS>::zero();
    assert_eq!(vector.get(0), &0);
    assert_eq!(vector.get(1), &0);
    assert_eq!(vector.get(2), &0);
    assert_eq!(vector.get_coordinates(), &[0, 0, 0]);
}

#[test]
fn zero_i64() {
    let vector = Vector::<i64, DIMENSIONS>::zero();
    assert_eq!(vector.get(0), &0);
    assert_eq!(vector.get(1), &0);
    assert_eq!(vector.get(2), &0);
    assert_eq!(vector.get_coordinates(), &[0, 0, 0]);
}

// Tests for new

#[test]
fn new_i8() {
    let vector = Vector::<i8, DIMENSIONS>::new([1, 2, 3]);
    assert_eq!(vector.get(0), &1);
    assert_eq!(vector.get(1), &2);
    assert_eq!(vector.get(2), &3);
    assert_eq!(vector.get_coordinates(), &[1, 2, 3]);
}

#[test]
fn new_i16() {
    let vector = Vector::<i16, DIMENSIONS>::new([1, 2, 3]);
    assert_eq!(vector.get(0), &1);
    assert_eq!(vector.get(1), &2);
    assert_eq!(vector.get(2), &3);
    assert_eq!(vector.get_coordinates(), &[1, 2, 3]);
}

#[test]
fn new_i32() {
    let vector = Vector::<i32, DIMENSIONS>::new([1, 2, 3]);
    assert_eq!(vector.get(0), &1);
    assert_eq!(vector.get(1), &2);
    assert_eq!(vector.get(2), &3);
    assert_eq!(vector.get_coordinates(), &[1, 2, 3]);
}

#[test]
fn new_i64() {
    let vector = Vector::<i64, DIMENSIONS>::new([1, 2, 3]);
    assert_eq!(vector.get(0), &1);
    assert_eq!(vector.get(1), &2);
    assert_eq!(vector.get(2), &3);
    assert_eq!(vector.get_coordinates(), &[1, 2, 3]);
}

#[test]
fn new_with_zeros() {
    let vector = Vector::<i32, DIMENSIONS>::new([0, 0, 0]);
    assert_eq!(vector.get(0), &0);
    assert_eq!(vector.get(1), &0);
    assert_eq!(vector.get(2), &0);
    assert_eq!(vector.get_coordinates(), &[0, 0, 0]);
}

#[test]
fn new_with_negative_values() {
    let vector = Vector::<i32, DIMENSIONS>::new([-1, -2, -3]);
    assert_eq!(vector.get(0), &-1);
    assert_eq!(vector.get(1), &-2);
    assert_eq!(vector.get(2), &-3);
    assert_eq!(vector.get_coordinates(), &[-1, -2, -3]);
}

#[test]
fn new_with_mixed_values() {
    let vector = Vector::<i32, DIMENSIONS>::new([0, -5, 3]);
    assert_eq!(vector.get(0), &0);
    assert_eq!(vector.get(1), &-5);
    assert_eq!(vector.get(2), &3);
    assert_eq!(vector.get_coordinates(), &[0, -5, 3]);
}

#[test]
fn new_with_extreme_values() {
    let vector = Vector::<i32, DIMENSIONS>::new([i32::MIN, 0, i32::MAX]);
    assert_eq!(vector.get(0), &i32::MIN);
    assert_eq!(vector.get(1), &0);
    assert_eq!(vector.get(2), &i32::MAX);
    assert_eq!(vector.get_coordinates(), &[i32::MIN, 0, i32::MAX]);
}

// Tests for from_point

#[test]
fn from_point_with_zero_point() {
    let point = Point::<i32, DIMENSIONS>::new([0, 0, 0]);
    let vector = Vector::<i32, DIMENSIONS>::from_point(&point);
    assert!(vector.is_some());
    let vector = vector.unwrap();
    assert_eq!(vector.get(0), &0);
    assert_eq!(vector.get(1), &0);
    assert_eq!(vector.get(2), &0);
    assert_eq!(vector.get_coordinates(), &[0, 0, 0]);
    assert!(vector.is_zero());
}

#[test]
fn from_point_with_mixed_values() {
    let point = Point::<i32, DIMENSIONS>::new([-3, 0, 7]);
    let vector = Vector::<i32, DIMENSIONS>::from_point(&point);
    assert!(vector.is_some());
    let vector = vector.unwrap();
    assert_eq!(vector.get(0), &-3);
    assert_eq!(vector.get(1), &0);
    assert_eq!(vector.get(2), &7);
    assert_eq!(vector.get_coordinates(), &[-3, 0, 7]);
}

#[test]
fn from_point_with_extreme_values() {
    let point = Point::<i32, DIMENSIONS>::new([i32::MIN, 0, i32::MAX]);
    let vector = Vector::<i32, DIMENSIONS>::from_point(&point);
    assert!(vector.is_some());
    let vector = vector.unwrap();
    assert_eq!(vector.get(0), &i32::MIN);
    assert_eq!(vector.get(1), &0);
    assert_eq!(vector.get(2), &i32::MAX);
    assert_eq!(vector.get_coordinates(), &[i32::MIN, 0, i32::MAX]);
}

// Tests for from_points

#[test]
fn from_points_positive_values() {
    let origin = Point::<i32, DIMENSIONS>::new([1, 2, 3]);
    let destination = Point::<i32, DIMENSIONS>::new([4, 6, 9]);
    let vector = Vector::<i32, DIMENSIONS>::from_points(&origin, &destination);
    assert!(vector.is_some());
    let vector = vector.unwrap();
    assert_eq!(vector.get(0), &3);
    assert_eq!(vector.get(1), &4);
    assert_eq!(vector.get(2), &6);
    assert_eq!(*vector.get_coordinates(), [3, 4, 6]);
}

#[test]
fn from_points_negative_values() {
    let origin = Point::<i32, DIMENSIONS>::new([-2, -5, -1]);
    let destination = Point::<i32, DIMENSIONS>::new([-8, -1, -7]);
    let vector = Vector::<i32, DIMENSIONS>::from_points(&origin, &destination);
    assert!(vector.is_some());
    let vector = vector.unwrap();
    assert_eq!(vector.get(0), &-6);
    assert_eq!(vector.get(1), &4);
    assert_eq!(vector.get(2), &-6);
    assert_eq!(*vector.get_coordinates(), [-6, 4, -6]);
}

#[test]
fn from_points_mixed_values() {
    let origin = Point::<i32, DIMENSIONS>::new([-3, 2, -5]);
    let destination = Point::<i32, DIMENSIONS>::new([5, -4, 10]);
    let vector = Vector::<i32, DIMENSIONS>::from_points(&origin, &destination);
    assert!(vector.is_some());
    let vector = vector.unwrap();
    assert_eq!(vector.get(0), &8);
    assert_eq!(vector.get(1), &-6);
    assert_eq!(vector.get(2), &15);
    assert_eq!(*vector.get_coordinates(), [8, -6, 15]);
}

#[test]
fn from_points_same_points() {
    let origin = Point::<i32, DIMENSIONS>::new([10, 15, -20]);
    let destination = Point::<i32, DIMENSIONS>::new([10, 15, -20]);
    let vector = Vector::<i32, DIMENSIONS>::from_points(&origin, &destination);
    assert!(vector.is_some());
    let vector = vector.unwrap();
    assert_eq!(vector.get(0), &0);
    assert_eq!(vector.get(1), &0);
    assert_eq!(vector.get(2), &0);
    assert_eq!(*vector.get_coordinates(), [0, 0, 0]);
    assert!(vector.is_zero());
}

#[test]
fn from_points_from_origin() {
    let origin = Point::<i32, DIMENSIONS>::new([0, 0, 0]);
    let destination = Point::<i32, DIMENSIONS>::new([7, -3, 12]);
    let vector = Vector::<i32, DIMENSIONS>::from_points(&origin, &destination);
    assert!(vector.is_some());
    let vector = vector.unwrap();
    assert_eq!(vector.get(0), &7);
    assert_eq!(vector.get(1), &-3);
    assert_eq!(vector.get(2), &12);
    assert_eq!(*vector.get_coordinates(), [7, -3, 12]);
}

#[test]
fn from_points_to_origin() {
    let origin = Point::<i32, DIMENSIONS>::new([5, -8, 15]);
    let destination = Point::<i32, DIMENSIONS>::new([0, 0, 0]);
    let vector = Vector::<i32, DIMENSIONS>::from_points(&origin, &destination);
    assert!(vector.is_some());
    let vector = vector.unwrap();
    assert_eq!(vector.get(0), &-5);
    assert_eq!(vector.get(1), &8);
    assert_eq!(vector.get(2), &-15);
    assert_eq!(*vector.get_coordinates(), [-5, 8, -15]);
}

#[test]
fn from_points_extreme_values_i32() {
    let origin = Point::<i32, DIMENSIONS>::new([i32::MIN, i32::MAX, 0]);
    let destination = Point::<i32, DIMENSIONS>::new([0, 0, i32::MIN]);
    let vector = Vector::<i32, DIMENSIONS>::from_points(&origin, &destination);
    assert_none!(vector);
}

#[test]
fn from_points_out_of_bounds() {
    let origin = Point::<i32, DIMENSIONS>::new([500, -100, 200]);
    let destination = Point::<i32, DIMENSIONS>::new([150, 200, -300]);
    let vector = Vector::<i8, DIMENSIONS>::from_points(&origin, &destination);
    assert_none!(vector);
}

#[test]
fn from_points_x_axis_aligned() {
    // X-axis aligned
    let origin = Point::<i32, DIMENSIONS>::new([2, 5, 8]);
    let destination = Point::<i32, DIMENSIONS>::new([7, 5, 8]);
    let vector = Vector::<i32, DIMENSIONS>::from_points(&origin, &destination);
    assert!(vector.is_some());
    let vector = vector.unwrap();
    assert_eq!(vector.get_coordinates(), &[5, 0, 0]);
    assert!(vector.is_axis());
}

#[test]
fn from_points_y_axis_aligned() {
    // Y-axis aligned
    let origin = Point::<i32, DIMENSIONS>::new([3, 1, -2]);
    let destination = Point::<i32, DIMENSIONS>::new([3, 8, -2]);
    let vector = Vector::<i32, DIMENSIONS>::from_points(&origin, &destination);
    assert!(vector.is_some());
    let vector = vector.unwrap();
    assert_eq!(vector.get_coordinates(), &[0, 7, 0]);
    assert!(vector.is_axis());
}

#[test]
fn from_points_z_axis_aligned() {
    // Z-axis aligned
    let origin = Point::<i32, DIMENSIONS>::new([10, -5, 3]);
    let destination = Point::<i32, DIMENSIONS>::new([10, -5, 15]);
    let vector = Vector::<i32, DIMENSIONS>::from_points(&origin, &destination);
    assert!(vector.is_some());
    let vector = vector.unwrap();
    assert_eq!(vector.get_coordinates(), &[0, 0, 12]);
    assert!(vector.is_axis());
}

#[test]
fn from_points_positive_diagonal() {
    let origin = Point::<i32, DIMENSIONS>::new([1, 0, 1]);
    let destination = Point::<i32, DIMENSIONS>::new([9, 0, 9]);
    let vector = Vector::<i32, DIMENSIONS>::from_points(&origin, &destination);
    assert!(vector.is_some());
    let vector = vector.unwrap();
    assert_eq!(vector.get_coordinates(), &[8, 0, 8]);
    assert!(vector.is_diagonal());
}

#[test]
fn from_points_negative_diagonal() {
    let origin = Point::<i32, DIMENSIONS>::new([7, -1, 0]);
    let destination = Point::<i32, DIMENSIONS>::new([0, -8, 0]);
    let vector = Vector::<i32, DIMENSIONS>::from_points(&origin, &destination);
    assert!(vector.is_some());
    let vector = vector.unwrap();
    assert_eq!(vector.get_coordinates(), &[-7, -7, 0]);
    assert!(vector.is_diagonal());
}

// Tests for absolute_coordinates

#[test]
fn absolute_coordinates_mixed_values() {
    let vector = v(-3, 4, -5);
    assert_eq!(vector.absolute_coordinates(), [3, 4, 5]);
}

#[test]
fn absolute_coordinates_all_positive() {
    let vector = v(3, 4, 5);
    assert_eq!(vector.absolute_coordinates(), [3, 4, 5]);
}

#[test]
fn absolute_coordinates_all_negative() {
    let vector = v(-3, -4, -5);
    assert_eq!(vector.absolute_coordinates(), [3, 4, 5]);
}

#[test]
fn absolute_coordinates_with_zeros() {
    let vector = v(0, -4, 0);
    let abs_coords = vector.absolute_coordinates();
    assert_eq!(abs_coords, [0, 4, 0]);
}

// Tests for max_coordinate

#[test]
fn max_coordinate() {
    let vector = v(-3, 7, -2);
    assert_eq!(vector.max_coordinate(), 7);
}

#[test]
fn max_coordinate_all_negative() {
    let vector = v(-3, -7, -2);
    assert_eq!(vector.max_coordinate(), 7);
}

#[test]
fn max_coordinate_with_zeros() {
    let vector = v(0, -5, 0);
    assert_eq!(vector.max_coordinate(), 5);
}

#[test]
fn max_coordinate_zero_vector() {
    let vector = v(0, 0, 0);
    assert_eq!(vector.max_coordinate(), 0);
}

// Tests for manhattan_distance

#[test]
fn manhattan_distance() {
    let vector = v(3, -4, 5);
    assert_eq!(vector.manhattan_distance(), 12);
}

#[test]
fn manhattan_distance_all_negative() {
    let vector = v(-3, -4, -5);
    assert_eq!(vector.manhattan_distance(), 12);
}

#[test]
fn manhattan_distance_with_zeros() {
    let vector = v(0, -4, 0);
    assert_eq!(vector.manhattan_distance(), 4);
}

#[test]
fn manhattan_distance_zero_vector() {
    let vector_zero = v(0, 0, 0);
    assert_eq!(vector_zero.manhattan_distance(), 0);
}

#[test]
fn manhattan_distance_single_coordinate() {
    let vector = v(0, 0, -7);
    assert_eq!(vector.manhattan_distance(), 7);
}

// Tests for normalize

#[test]
fn normalize_one_dimension() {
    let vector = v(0, -3, 0);
    let normalized = vector.normalize();
    assert_eq!(normalized.get_coordinates(), &[0, -1, 0]);
}

#[test]
fn normalize_several_dimensions() {
    let vector = v(5, -3, 0);
    let normalized = vector.normalize();
    assert_eq!(normalized.get_coordinates(), &[1, -1, 0]);
}

#[test]
fn normalize_already_normalized() {
    let vector = v(1, -1, 0);
    let normalized = vector.normalize();
    assert_eq!(normalized.get_coordinates(), &[1, -1, 0]);
}

#[test]
fn normalize_zero_vector() {
    let vector = v(0, 0, 0);
    let normalized = vector.normalize();
    assert_eq!(normalized, vector);
}

#[test]
fn normalize_extreme_values() {
    let vector = v(i32::MAX, i32::MIN, 0);
    let normalized = vector.normalize();

    assert_eq!(normalized.get_coordinates(), &[1, -1, 0]);
}

// Tests for is_zero

#[test]
fn is_zero_zero() {
    let vector = v(0, 0, 0);
    assert!(vector.is_zero());
}

#[test]
fn is_zero_positive_values() {
    let vector = v(1, 5, 0);
    assert!(!vector.is_zero());
}

#[test]
fn is_zero_negative_values() {
    let vector = v(-1, 0, 0);
    assert!(!vector.is_zero());
}

// Tests for is_axis

#[test]
fn is_axis_positive_values() {
    let axis_x = v(5, 0, 0);
    let axis_y = v(0, 3, 0);
    let axis_z = v(0, 0, -2);
    let diagonal = v(3, 4, 0);
    let zero = v(0, 0, 0);

    assert!(axis_x.is_axis());
    assert!(axis_y.is_axis());
    assert!(axis_z.is_axis());
    assert!(!diagonal.is_axis());
    assert!(!zero.is_axis());
}

#[test]
fn is_axis_negative_values() {
    let axis_neg_x = v(-5, 0, 0);
    let axis_neg_y = v(0, -3, 0);

    assert!(axis_neg_x.is_axis());
    assert!(axis_neg_y.is_axis());
}

#[test]
fn is_axis_multiple_non_zero() {
    let non_axis = v(-1, 2, 0);
    assert!(!non_axis.is_axis());

    let non_axis_all = v(-1, -2, -3);
    assert!(!non_axis_all.is_axis());
}

// Tests for is_type

#[test]
fn is_type_positive_values() {
    let zero = v(0, 0, 0);
    let axis = v(0, 0, 1);
    let diagonal = v(0, 2, 2);
    let arbitrary = v(1, 2, 3);

    assert_eq!(zero.is_type(), VectorType::Zero);
    assert_eq!(axis.is_type(), VectorType::Axis);
    assert_eq!(diagonal.is_type(), VectorType::Diagonal);
    assert_eq!(arbitrary.is_type(), VectorType::Arbitrary);
}

#[test]
fn is_type_negative_values() {
    let arbitrary_mixed = v(-1, 2, 0);
    assert_eq!(arbitrary_mixed.is_type(), VectorType::Arbitrary);
}

// Tests for Index trait

#[test]
fn index_operator() {
    let vector = v(10, 20, 30);
    assert_eq!(vector[0], 10);
    assert_eq!(vector[1], 20);
    assert_eq!(vector[2], 30);
}

#[test]
fn index_operator_negative_values() {
    let vector = v(-10, -20, 0);
    assert_eq!(vector[0], -10);
    assert_eq!(vector[1], -20);
    assert_eq!(vector[2], 0);
}

// Tests for Display trait

#[test]
fn display() {
    let vector = v(1, 2, 3);
    assert_eq!(format!("{}", vector), "(1,2,3)");
}

#[test]
fn display_negative_values() {
    let vector_mixed = v(-1, 0, 3);
    assert_eq!(format!("{}", vector_mixed), "(-1,0,3)");
}

#[test]
fn display_zeros() {
    let vector_zero = v(0, 0, 0);
    assert_eq!(format!("{}", vector_zero), "(0,0,0)");
}

// Tests for add

#[test]
fn add_correct() {
    let v1 = v(1, 2, 3);
    let v2 = v(4, 5, 6);
    let result = v1.add(v2);
    assert_some!(result);
    let result = result.unwrap();
    assert_eq!(result.get_coordinates(), &[5, 7, 9]);
}

#[test]
fn add_out_of_bounds() {
    let v1 = v(i32::MAX, 2, 3);
    let v2 = v(4, 5, 6);
    let result = v1.add(v2);
    assert_none!(result);
}

// Tests for subtract

#[test]
fn subtract_correct() {
    let v1 = v(10, 8, 6);
    let v2 = v(3, 2, 1);
    let result = v1.sub(v2);
    assert_some!(result);
    let result = result.unwrap();
    assert_eq!(result.get_coordinates(), &[7, 6, 5]);
}

#[test]
fn subtract_out_of_bounds() {
    let v1 = v(i32::MAX, 2, 3);
    let v2 = v(4, 5, 6);
    let result = v1.add(v2);
    assert_none!(result);
}

// Tests for invert

#[test]
fn invert_correct() {
    let vector = v(3, -4, 5);
    let negated = vector.neg();
    assert_some!(negated);
    let negated = negated.unwrap();
    assert_eq!(negated.get_coordinates(), &[-3, 4, -5]);
}

#[test]
fn invert_zeros() {
    let vector = v(0, 0, 0);
    let negated = vector.neg();
    assert_some!(negated);
    let negated = negated.unwrap();
    assert_eq!(negated.get_coordinates(), &[0, 0, 0]);
}

#[test]
fn invert_out_of_bounds() {
    let vector = v(i32::MAX, -4, i32::MIN);
    let negated = vector.neg();
    assert_none!(negated);
}

// Tests for multiply_by_scalar

#[test]
fn multiply_by_scalar_with_positive_value() {
    let vector = v(1, -2, 3);
    let scaled = vector.mul(2);
    assert_some!(scaled);
    let scaled = scaled.unwrap();
    assert_eq!(scaled.get_coordinates(), &[2, -4, 6]);
}

#[test]
fn scalar_multiplication_with_zero() {
    let vector = v(5, -3, 7);
    let scaled = vector.mul(0);
    assert_some!(scaled);
    let scaled = scaled.unwrap();
    assert_eq!(scaled.get_coordinates(), &[0, 0, 0]);
    assert!(scaled.is_zero());
}

#[test]
fn multiply_by_scalar_with_negative_value() {
    let vector = v(1, -2, 3);
    let scaled = vector.mul(-2);
    assert_some!(scaled);
    let scaled = scaled.unwrap();
    assert_eq!(scaled.get_coordinates(), &[-2, 4, -6]);
}

#[test]
fn multiply_by_scalar_out_of_bounds() {
    let vector = v(i32::MIN, -2, 3);
    let scaled = vector.mul(2);
    assert_none!(scaled);
}

// Tests for is_collinear

#[test]
fn is_collinear_true_positive() {
    let v1 = v(2, 4, 6);
    let v2 = v(1, 2, 3);
    assert!(Vector::is_collinear(&v1, &v2));
}

#[test]
fn is_collinear_true_negative() {
    let v1 = v(2, 4, 6);
    let v2 = v(-1, -2, -3);
    assert!(Vector::is_collinear(&v1, &v2));
}

#[test]
fn is_collinear_false() {
    let v1 = v(2, 4, 6);
    let v2 = v(1, 2, 4);
    assert!(!Vector::is_collinear(&v1, &v2));
}

// Tests for convert

#[test]
fn convert_i32_to_i64_success() {
    let vector_i32: Vector<i32, DIMENSIONS> = v(1, -2, 3);
    let vector_i64 = vector_i32.convert::<i64>();
    assert_some!(vector_i64);
    let vector_i64 = vector_i64.unwrap();
    assert_eq!(vector_i64.get_coordinates(), &[1, -2, 3]);
}

#[test]
fn convert_i32_to_i8_failure() {
    let vector_i32 = v(300, -2, 3);
    let vector_i64 = vector_i32.convert::<i8>();
    assert_none!(vector_i64);
}
