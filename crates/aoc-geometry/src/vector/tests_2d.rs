/*use super::*;
use crate::coordinate_value::{maximum_coordinate_value, minimum_coordinate_value};
use crate::point::Point;
use pretty_assertions::{assert_eq, assert_ne};
// Tests for zero

#[test]
fn vector_zero_2d() {
    let zero: Vector<i32, 2> = Vector::zero();
    assert_eq!(zero.get(0), &0);
    assert_eq!(zero.get(1), &0);
    assert_eq!(zero.get_coordinates(), &[0, 0]);
}

#[test]
fn vector_zero_different_types_2d() {
    let zero_i8: Vector<i8, 2> = Vector::zero();
    assert_eq!(zero_i8.get_coordinates(), &[0i8, 0i8]);

    let zero_i64: Vector<i64, 2> = Vector::zero();
    assert_eq!(zero_i64.get_coordinates(), &[0i64, 0i64]);
}

// Tests for new

#[test]
fn vector_new_2d() {
    let vector = Vector::new(&[3, 4]);
    assert_eq!(vector.get(0), &3);
    assert_eq!(vector.get(1), &4);
    assert_eq!(vector.get_coordinates(), &[3, 4]);
}

#[test]
fn vector_new_different_types_2d() {
    let vector_i8 = Vector::new(&[1i8, 2i8]);
    assert_eq!(vector_i8.get(0), &1i8);

    let vector_i16 = Vector::new(&[10i16, 20i16]);
    assert_eq!(vector_i16.get(0), &10i16);

    let vector_i64 = Vector::new(&[100i64, 200i64]);
    assert_eq!(vector_i64.get(0), &100i64);
}

#[test]
fn vector_new_with_zeros_2d() {
    let vector_2d = Vector::new(&[0, 0]);
    assert_eq!(vector_2d.get_coordinates(), &[0, 0]);
}

#[test]
fn vector_new_with_negative_values_2d() {
    let vector_2d = Vector::new(&[-5, -10]);
    assert_eq!(vector_2d.get_coordinates(), &[-5, -10]);
}

#[test]
fn vector_new_with_mixed_values_2d() {
    let vector_2d = Vector::new(&[-5, 10]);
    assert_eq!(vector_2d.get_coordinates(), &[-5, 10]);
}

#[test]
fn vector_new_with_extreme_values_2d() {
    let vector_max = Vector::new(&[i32::MAX, i32::MIN]);
    assert_eq!(vector_max.get_coordinates(), &[i32::MAX, i32::MIN]);
}

// Tests for from_point

#[test]
fn vector_from_point_2d() {
    let point = Point::new(&[3, 4]);
    let vector = Vector::from_point(&point);
    assert_eq!(vector.get(0), &3);
    assert_eq!(vector.get(1), &4);
}

#[test]
fn vector_from_point_negative_values_2d() {
    let point = Point::new(&[-5, -10]);
    let vector = Vector::from_point(&point);
    assert_eq!(vector.get_coordinates(), &[-5, -10]);
}

// Tests for from_points

#[test]
fn from_points_positive_values_2d() {
    let origin = Point::new(&[1, 2]);
    let destination = Point::new(&[4, 6]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(*vector.get_coordinates(), [3, 4]);
}

#[test]
fn from_points_negative_values_2d() {
    let origin = Point::new(&[-2, -5]);
    let destination = Point::new(&[-8, -1]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(*vector.get_coordinates(), [-6, 4]);
}

#[test]
fn from_points_mixed_values_2d() {
    let origin = Point::new(&[-3, 2]);
    let destination = Point::new(&[5, -4]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(*vector.get_coordinates(), [8, -6]);
}

#[test]
fn from_points_zero_vector_2d() {
    let origin = Point::new(&[10, 15]);
    let destination = Point::new(&[10, 15]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(*vector.get_coordinates(), [0, 0]);
    assert!(vector.is_zero());
}

#[test]
fn from_points_from_origin_2d() {
    let origin = Point::new(&[0, 0]);
    let destination = Point::new(&[7, -3]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(*vector.get_coordinates(), [7, -3]);
}

#[test]
fn from_points_to_origin_2d() {
    let origin = Point::new(&[5, -8]);
    let destination = Point::new(&[0, 0]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(*vector.get_coordinates(), [-5, 8]);
}

#[test]
fn from_points_extreme_values_i32_2d() {
    let origin: Point<i32, 2> =
        Point::new(&[minimum_coordinate_value(), maximum_coordinate_value()]);
    let destination: Point<i32, 2> = Point::new(&[0, 0]);
    let vector: Vector<i32, 2> = Vector::from_points(&origin, &destination);
    assert_eq!(
        *vector.get_coordinates(),
        [
            -minimum_coordinate_value::<i32>(),
            -maximum_coordinate_value::<i32>()
        ]
    );
}

#[test]
fn from_points_different_types_i8_2d() {
    let origin = Point::new(&[5i8, -10i8]);
    let destination = Point::new(&[15i8, 20i8]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(vector.get_coordinates(), &[10i8, 30i8]);
}

#[test]
fn from_points_different_types_i16_2d() {
    let origin = Point::new(&[100i16, -200i16]);
    let destination = Point::new(&[300i16, 400i16]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(vector.get_coordinates(), &[200i16, 600i16]);
}

#[test]
fn from_points_different_types_i64_2d() {
    let origin = Point::new(&[1000i64, -2000i64]);
    let destination = Point::new(&[5000i64, 3000i64]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(vector.get_coordinates(), &[4000i64, 5000i64]);
}

#[test]
fn from_points_extreme_i8_2d() {
    let origin: Point<i8, 2> =
        Point::new(&[minimum_coordinate_value(), maximum_coordinate_value()]);
    let destination: Point<i8, 2> = Point::new(&[0i8, 0i8]);
    let vector: Vector<i8, 2> = Vector::from_points(&origin, &destination);
    assert_eq!(
        *vector.get_coordinates(),
        [
            -minimum_coordinate_value::<i8>(),
            -maximum_coordinate_value::<i8>()
        ]
    );
}

#[test]
fn from_points_extreme_i16_2d() {
    let origin: Point<i16, 2> =
        Point::new(&[minimum_coordinate_value(), maximum_coordinate_value()]);
    let destination: Point<i16, 2> = Point::new(&[0i16, 0i16]);
    let vector: Vector<i16, 2> = Vector::from_points(&origin, &destination);
    assert_eq!(
        *vector.get_coordinates(),
        [
            -minimum_coordinate_value::<i16>(),
            -maximum_coordinate_value::<i16>()
        ]
    );
}

#[test]
fn from_points_extreme_i64_2d() {
    let origin: Point<i64, 2> =
        Point::new(&[minimum_coordinate_value(), maximum_coordinate_value()]);
    let destination: Point<i64, 2> = Point::new(&[0i64, 0i64]);
    let vector: Vector<i64, 2> = Vector::from_points(&origin, &destination);
    assert_eq!(
        *vector.get_coordinates(),
        [
            -minimum_coordinate_value::<i64>(),
            -maximum_coordinate_value::<i64>()
        ]
    );
}

#[test]
fn from_points_axis_aligned_vectors_2d() {
    // X-axis aligned
    let origin = Point::new(&[2, 5]);
    let destination = Point::new(&[7, 5]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(*vector.get_coordinates(), [5, 0]);
    assert!(vector.is_axis());

    // Y-axis aligned
    let origin = Point::new(&[3, 1]);
    let destination = Point::new(&[3, 8]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(*vector.get_coordinates(), [0, 7]);
    assert!(vector.is_axis());
}

// Tests for chebyshev_distance

#[test]
fn vector_chebyshev_distance_2d() {
    let vector_2d = Vector::new(&[4, -9]);
    assert_eq!(vector_2d.chebyshev_distance(), 9);
}

// Tests for manhattan_distance

#[test]
fn vector_manhattan_distance_2d() {
    let vector_2d = Vector::new(&[-2, 3]);
    assert_eq!(vector_2d.manhattan_distance(), 5); // |-2| + |3| = 2 + 3 = 5
}

// Tests for normalize

#[test]
fn vector_normalize_2d() {
    let vector_2d = Vector::new(&[0, 1]);
    let normalized_2d = vector_2d.normalize();
    assert_eq!(normalized_2d.get(0), &0);
    assert_eq!(normalized_2d.get(1), &1);
}

// Tests for is_zero

#[test]
fn vector_is_zero_2d() {
    let zero_2d = Vector::new(&[0, 0]);
    assert!(zero_2d.is_zero());
}

// Tests for is_axis

#[test]
fn vector_is_axis_2d() {
    let axis_2d = Vector::new(&[0, 7]);
    assert!(axis_2d.is_axis());
}

// Tests for is (vector type)

#[test]
fn vector_type_2d() {
    let zero = Vector::new(&[0, 0]);
    let axis = Vector::new(&[1, 0]);
    let arbitrary = Vector::new(&[1, 2]);

    assert_eq!(zero.is(), VectorType::Zero);
    assert_eq!(axis.is(), VectorType::Axis);
    assert_eq!(arbitrary.is(), VectorType::Arbitrary);
}

#[test]
fn vector_type_negative_values_2d() {
    let axis_neg = Vector::new(&[-1, 0]);
    assert_eq!(axis_neg.is(), VectorType::Axis);

    let arbitrary_neg = Vector::new(&[-1, -2]);
    assert_eq!(arbitrary_neg.is(), VectorType::Arbitrary);
}

// Tests for Index trait

#[test]
fn vector_index_operator_2d() {
    let vector = Vector::new(&[10, 20]);
    assert_eq!(vector[0], 10);
    assert_eq!(vector[1], 20);
}

#[test]
fn vector_index_operator_negative_values_2d() {
    let vector = Vector::new(&[-10, -20]);
    assert_eq!(vector[0], -10);
    assert_eq!(vector[1], -20);
}

// Tests for Display trait

#[test]
fn vector_display_2d() {
    let vector_2d = Vector::new(&[3, 4]);
    assert_eq!(format!("{}", vector_2d), "(3,4)");
}

#[test]
fn vector_display_negative_values_2d() {
    let vector_neg = Vector::new(&[-5, -10]);
    assert_eq!(format!("{}", vector_neg), "(-5,-10)");
}

#[test]
fn vector_display_zeros_2d() {
    let vector_zero = Vector::new(&[0, 0]);
    assert_eq!(format!("{}", vector_zero), "(0,0)");
}

// Tests for Add trait (vector + vector)

#[test]
fn vector_addition_2d() {
    let v1 = Vector::new(&[1, 2]);
    let v2 = Vector::new(&[3, 4]);
    let result = v1 + v2;

    assert_eq!(result.get(0), &4);
    assert_eq!(result.get(1), &6);
}

#[test]
fn vector_addition_negative_values_2d() {
    let v1 = Vector::new(&[-1, -2]);
    let v2 = Vector::new(&[-3, -4]);
    let result = v1 + v2;

    assert_eq!(result.get_coordinates(), &[-4, -6]);
}

#[test]
fn vector_addition_negative_values_mixed_2d() {
    let v1_mixed = Vector::new(&[-1, 2]);
    let v2_mixed = Vector::new(&[3, -4]);
    let result_mixed = v1_mixed + v2_mixed;
    assert_eq!(result_mixed.get_coordinates(), &[2, -2]);
}

#[test]
fn vector_addition_with_zeros_2d() {
    let v1 = Vector::new(&[5, -3]);
    let v2 = Vector::new(&[0, 0]);
    let result = v1 + v2;

    assert_eq!(result.get_coordinates(), &[5, -3]);
}

// Tests for Sub trait (vector - vector)

#[test]
fn vector_subtraction_2d() {
    let v1 = Vector::new(&[5, 7]);
    let v2 = Vector::new(&[2, 3]);
    let result = v1 - v2;

    assert_eq!(result.get(0), &3);
    assert_eq!(result.get(1), &4);
}

#[test]
fn vector_subtraction_negative_values_2d() {
    let v1 = Vector::new(&[-5, -2]);
    let v2 = Vector::new(&[-3, -4]);
    let result = v1 - v2;

    assert_eq!(result.get_coordinates(), &[-2, 2]);
}

#[test]
fn vector_subtraction_negative_values_mixed_2d() {
    let v1_mixed = Vector::new(&[5, -2]);
    let v2_mixed = Vector::new(&[-3, 4]);
    let result_mixed = v1_mixed - v2_mixed;
    assert_eq!(result_mixed.get_coordinates(), &[8, -6]);
}

#[test]
fn vector_subtraction_resulting_in_zero_2d() {
    let v1 = Vector::new(&[5, -3]);
    let v2 = Vector::new(&[5, -3]);
    let result = v1 - v2;

    assert_eq!(result.get_coordinates(), &[0, 0]);
    assert!(result.is_zero());
}

// Tests for Neg trait

#[test]
fn vector_negation_2d() {
    let vector_2d = Vector::new(&[1, -2]);
    let negated_2d = -vector_2d;
    assert_eq!(negated_2d.get(0), &-1);
    assert_eq!(negated_2d.get(1), &2);
}

#[test]
fn vector_negation_zeros_2d() {
    let vector_zero = Vector::new(&[0, 0]);
    let negated = -vector_zero;
    assert_eq!(negated.get_coordinates(), &[0, 0]);
}

#[test]
fn vector_negation_extreme_values_2d() {
    let vector_max = Vector::new(&[i32::MAX, i32::MIN + 1]); // MIN + 1 to avoid overflow
    let negated = -vector_max;
    assert_eq!(negated.get(0), &(-i32::MAX));
    assert_eq!(negated.get(1), &(-(i32::MIN + 1)));
}

// Tests for Mul trait (vector * scalar)

#[test]
fn vector_scalar_multiplication_vector_first_2d() {
    let vector = Vector::new(&[2, 3]);
    let scaled = vector * 4;

    assert_eq!(scaled.get(0), &8);
    assert_eq!(scaled.get(1), &12);
}

#[test]
fn vector_scalar_multiplication_negative_scalar_2d() {
    let vector = Vector::new(&[2, -3]);
    let scaled = vector * -4;

    assert_eq!(scaled.get_coordinates(), &[-8, 12]);
}

// Tests for Mul trait (scalar * vector)

#[test]
fn vector_scalar_multiplication_scalar_first_2d() {
    let vector = Vector::new(&[2, 3]);
    let scaled = 4 * vector;

    assert_eq!(scaled.get(0), &8);
    assert_eq!(scaled.get(1), &12);
}

#[test]
fn vector_scalar_multiplication_commutativity_2d() {
    let vector = Vector::new(&[3, 4]);
    let scalar = 5;

    let result1 = vector * scalar;
    let result2 = scalar * vector;

    assert_eq!(result1, result2);
}

#[test]
fn vector_scalar_multiplication_commutativity_negative_2d() {
    let vector = Vector::new(&[-3, 4]);
    let scalar = -5;

    let result1 = vector * scalar;
    let result2 = scalar * vector;

    assert_eq!(result1, result2);
    assert_eq!(result1.get_coordinates(), &[15, -20]);
}

#[test]
fn vector_scalar_multiplication_commutativity_zero_2d() {
    let vector = Vector::new(&[3, -4]);
    let scalar = 0;

    let result1 = vector * scalar;
    let result2 = scalar * vector;

    assert_eq!(result1, result2);
    assert!(result1.is_zero());
}

// Tests for Add trait (vector + point)

#[test]
fn vector_add_point_2d() {
    let vector = Vector::new(&[3, 4]);
    let point = Point::new(&[1, 2]);
    let result = vector + point;

    assert_eq!(result.get(0), &4);
    assert_eq!(result.get(1), &6);
}

#[test]
fn vector_add_point_negative_values_2d() {
    let vector = Vector::new(&[-3, -4]);
    let point = Point::new(&[-1, -2]);
    let result = vector + point;

    assert_eq!(result.get_coordinates(), &[-4, -6]);
}

#[test]
fn vector_add_point_zero_cases_2d() {
    let zero_vector = Vector::new(&[0, 0]);
    let point = Point::new(&[5, -3]);
    let result = zero_vector + point;

    assert_eq!(result.get_coordinates(), &[5, -3]);
}

#[test]
fn vector_add_point_zero_cases_origin_2d() {
    let vector = Vector::new(&[5, -3]);
    let origin = Point::new(&[0, 0]);
    let result2 = vector + origin;
    assert_eq!(result2.get_coordinates(), &[5, -3]);
}

// Tests for PartialEq trait

#[test]
fn vector_equality_2d() {
    let vector1 = Vector::new(&[1, 2]);
    let vector2 = Vector::new(&[1, 2]);
    let vector3 = Vector::new(&[2, 1]);

    assert_eq!(vector1, vector2);
    assert_ne!(vector1, vector3);
}

#[test]
fn vector_equality_negative_values_2d() {
    let vector1 = Vector::new(&[-5, -10]);
    let vector2 = Vector::new(&[-5, -10]);
    let vector3 = Vector::new(&[-10, -5]);

    assert_eq!(vector1, vector2);
    assert_ne!(vector1, vector3);
}

#[test]
fn vector_equality_zeros_2d() {
    let vector1 = Vector::new(&[0, 0]);
    let vector2 = Vector::new(&[0, 0]);
    let zero_vector = Vector::zero();

    assert_eq!(vector1, vector2);
    assert_eq!(vector1, zero_vector);
}

// Tests for Clone and Copy traits

#[test]
fn vector_clone_copy_2d() {
    let vector = Vector::new(&[1, 2]);
    let cloned = vector.clone();
    let copied = vector;

    assert_eq!(vector, cloned);
    assert_eq!(vector, copied);
}

#[test]
fn vector_clone_copy_negative_values_2d() {
    let vector = Vector::new(&[-1, -2]);
    let cloned = vector.clone();
    let copied = vector;

    assert_eq!(vector, cloned);
    assert_eq!(vector, copied);
    assert_eq!(cloned.get_coordinates(), &[-1, -2]);
}

// Tests with negative coordinates

#[test]
fn vector_with_negative_coordinates_2d() {
    let vector = Vector::new(&[-5, -10]);
    assert_eq!(vector.get(0), &-5);
    assert_eq!(vector.get(1), &-10);

    assert_eq!(vector.manhattan_distance(), 15);
    assert_eq!(vector.chebyshev_distance(), 10);
}

// Tests for Hash trait

#[test]
fn vector_hash_2d() {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    let vector1 = Vector::new(&[1, 2]);
    let vector2 = Vector::new(&[1, 2]);
    let vector3 = Vector::new(&[2, 1]);

    set.insert(vector1);
    set.insert(vector2);
    set.insert(vector3);

    assert_eq!(set.len(), 2); // vector1 and vector2 are the same
}

#[test]
fn vector_hash_negative_values_2d() {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    let vector1 = Vector::new(&[-1, -2]);
    let vector2 = Vector::new(&[-1, -2]);
    let vector3 = Vector::new(&[1, 2]);
    let vector4 = Vector::new(&[0, 0]);

    set.insert(vector1);
    set.insert(vector2);
    set.insert(vector3);
    set.insert(vector4);

    assert_eq!(set.len(), 3); // vector1 and vector2 are the same
}

// Tests for Debug trait

#[test]
fn vector_debug_2d() {
    let vector = Vector::new(&[1, 2]);
    let debug_str = format!("{:?}", vector);
    assert!(debug_str.contains("Vector"));
    assert!(debug_str.contains("1"));
    assert!(debug_str.contains("2"));
}

#[test]
fn vector_debug_negative_values_2d() {
    let vector = Vector::new(&[-1, -2]);
    let debug_str = format!("{:?}", vector);
    assert!(debug_str.contains("Vector"));
    assert!(debug_str.contains("-1"));
    assert!(debug_str.contains("-2"));
}

// Edge case tests

#[test]
fn vector_edge_cases_2d() {
    // Test with maximum values
    let vector_max = Vector::new(&[i32::MAX, i32::MIN]);
    assert_eq!(vector_max.get(0), &i32::MAX);
    assert_eq!(vector_max.get(1), &i32::MIN);
}

#[test]
fn vector_edge_cases_negative_extremes_2d() {
    let vector_extreme = Vector::new(&[i32::MIN + 1, i32::MAX]);
    assert_eq!(vector_extreme.chebyshev_distance(), i32::MAX);
    // Manhattan distance should handle the conversion to u64 properly
    let expected_manhattan = (i32::MAX as u64) + (((i32::MIN + 1) as i64).abs() as u64);
    assert_eq!(vector_extreme.manhattan_distance(), expected_manhattan);
}

// Tests with zero vector operations

#[test]
fn vector_operations_with_zero_2d() {
    let vector = Vector::new(&[3, 4]);
    let zero = Vector::zero();

    let add_result = vector + zero;
    assert_eq!(add_result, vector);

    let sub_result = vector - zero;
    assert_eq!(sub_result, vector);

    let mul_result = vector * 0;
    assert_eq!(mul_result, zero);
}

#[test]
fn vector_operations_zero_with_negative_2d() {
    let vector = Vector::new(&[-3, 4]);
    let zero = Vector::zero();

    let add_result = vector + zero;
    assert_eq!(add_result, vector);

    let sub_result = vector - zero;
    assert_eq!(sub_result, vector);

    let sub_from_zero = zero - vector;
    assert_eq!(sub_from_zero.get_coordinates(), &[3, -4]);
}
*/
