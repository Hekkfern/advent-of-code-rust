/*use super::*;
use crate::coordinate_value::{maximum_coordinate_value, minimum_coordinate_value};
use crate::point::Point;
use pretty_assertions::assert_eq;
// Tests for zero

#[test]
fn vector_zero_3d() {
    let zero: Vector<i32, 3> = Vector::zero();
    assert_eq!(zero.get(0), &0);
    assert_eq!(zero.get(1), &0);
    assert_eq!(zero.get(2), &0);
    assert_eq!(zero.get_coordinates(), &[0, 0, 0]);
}

#[test]
fn vector_zero_different_types_3d() {
    let zero_i16: Vector<i16, 3> = Vector::zero();
    assert_eq!(zero_i16.get_coordinates(), &[0i16, 0i16, 0i16]);
}

// Tests for new

#[test]
fn vector_new_3d() {
    let vector = Vector::new(&[1, 2, 3]);
    assert_eq!(vector.get(0), &1);
    assert_eq!(vector.get(1), &2);
    assert_eq!(vector.get(2), &3);
    assert_eq!(vector.get_coordinates(), &[1, 2, 3]);
}

#[test]
fn vector_new_different_types_3d() {
    let vector_i16 = Vector::new(&[10i16, 20i16, 30i16]);
    assert_eq!(vector_i16.get(0), &10i16);
}

#[test]
fn vector_new_with_zeros_3d() {
    let vector_3d = Vector::new(&[0, 0, 0]);
    assert_eq!(vector_3d.get_coordinates(), &[0, 0, 0]);
}

#[test]
fn vector_new_with_negative_values_3d() {
    let vector_3d = Vector::new(&[-1, -2, -3]);
    assert_eq!(vector_3d.get_coordinates(), &[-1, -2, -3]);
}

#[test]
fn vector_new_with_mixed_values_3d() {
    let vector_3d = Vector::new(&[0, -5, 3]);
    assert_eq!(vector_3d.get_coordinates(), &[0, -5, 3]);
}

#[test]
fn vector_new_with_extreme_values_3d() {
    let vector_3d_extreme = Vector::new(&[i32::MIN, 0, i32::MAX]);
    assert_eq!(
        vector_3d_extreme.get_coordinates(),
        &[i32::MIN, 0, i32::MAX]
    );
}

// Tests for from_point

#[test]
fn vector_from_point_zero_point_3d() {
    let point = Point::new(&[0, 0, 0]);
    let vector = Vector::from_point(&point);
    assert_eq!(vector.get_coordinates(), &[0, 0, 0]);
    assert!(vector.is_zero());
}

#[test]
fn vector_from_point_mixed_values_3d() {
    let point = Point::new(&[-3, 0, 7]);
    let vector = Vector::from_point(&point);
    assert_eq!(vector.get_coordinates(), &[-3, 0, 7]);
}

// Tests for absolute_coordinates

#[test]
fn vector_absolute_coordinates_3d() {
    let vector = Vector::new(&[-3, 4, -5]);
    let abs_coords = vector.absolute_coordinates();
    assert_eq!(abs_coords, [3, 4, 5]);
}

#[test]
fn vector_absolute_coordinates_all_positive_3d() {
    let vector = Vector::new(&[3, 4, 5]);
    let abs_coords = vector.absolute_coordinates();
    assert_eq!(abs_coords, [3, 4, 5]);
}

#[test]
fn vector_absolute_coordinates_all_negative_3d() {
    let vector = Vector::new(&[-3, -4, -5]);
    let abs_coords = vector.absolute_coordinates();
    assert_eq!(abs_coords, [3, 4, 5]);
}

#[test]
fn vector_absolute_coordinates_with_zeros_3d() {
    let vector = Vector::new(&[0, -4, 0]);
    let abs_coords = vector.absolute_coordinates();
    assert_eq!(abs_coords, [0, 4, 0]);
}

// Tests for chebyshev_distance

#[test]
fn vector_chebyshev_distance_3d() {
    let vector = Vector::new(&[-3, 7, -2]);
    let max_coord = vector.chebyshev_distance();
    assert_eq!(max_coord, 7);
}

#[test]
fn vector_chebyshev_distance_all_negative_3d() {
    let vector = Vector::new(&[-3, -7, -2]);
    let max_coord = vector.chebyshev_distance();
    assert_eq!(max_coord, 7);
}

#[test]
fn vector_chebyshev_distance_with_zeros_3d() {
    let vector = Vector::new(&[0, -5, 0]);
    assert_eq!(vector.chebyshev_distance(), 5);
}

#[test]
fn vector_chebyshev_distance_zero_vector_3d() {
    let vector_zero = Vector::new(&[0, 0, 0]);
    assert_eq!(vector_zero.chebyshev_distance(), 0);
}

#[test]
fn vector_chebyshev_distance_single_element() {
    let vector = Vector::new(&[-10]);
    assert_eq!(vector.chebyshev_distance(), 10);
}

// Tests for manhattan_distance

#[test]
fn vector_manhattan_distance_3d() {
    let vector = Vector::new(&[3, -4, 5]);
    let distance = vector.manhattan_distance();
    assert_eq!(distance, 12); // |3| + |-4| + |5| = 3 + 4 + 5 = 12
}

#[test]
fn vector_manhattan_distance_all_negative_3d() {
    let vector = Vector::new(&[-3, -4, -5]);
    let distance = vector.manhattan_distance();
    assert_eq!(distance, 12); // |-3| + |-4| + |-5| = 3 + 4 + 5 = 12
}

#[test]
fn vector_manhattan_distance_with_zeros_3d() {
    let vector = Vector::new(&[0, -4, 0]);
    assert_eq!(vector.manhattan_distance(), 4);
}

#[test]
fn vector_manhattan_distance_zero_vector_3d() {
    let vector_zero = Vector::new(&[0, 0, 0]);
    assert_eq!(vector_zero.manhattan_distance(), 0);
}

#[test]
fn vector_manhattan_distance_single_coordinate_3d() {
    let vector = Vector::new(&[0, 0, -7]);
    assert_eq!(vector.manhattan_distance(), 7);
}

// Tests for normalize

#[test]
fn vector_normalize_3d() {
    let vector = Vector::new(&[5, -3, 0]);
    let normalized = vector.normalize();

    assert_eq!(normalized.get(0), &1); // 5 clamped to 1
    assert_eq!(normalized.get(1), &-1); // -3 clamped to -1
    assert_eq!(normalized.get(2), &0); // 0 remains 0
}

#[test]
fn vector_normalize_already_normalized_3d() {
    let vector = Vector::new(&[1, -1, 0]);
    let normalized = vector.normalize();

    assert_eq!(normalized.get_coordinates(), &[1, -1, 0]);
}

#[test]
fn vector_normalize_zero_vector_3d() {
    let vector = Vector::new(&[0, 0, 0]);
    let normalized = vector.normalize();

    assert_eq!(normalized.get_coordinates(), &[0, 0, 0]);
}

#[test]
fn vector_normalize_extreme_values_3d() {
    let vector = Vector::new(&[i32::MAX, i32::MIN, 0]);
    let normalized = vector.normalize();

    assert_eq!(normalized.get_coordinates(), &[1, -1, 0]);
}

// Tests for is_zero

#[test]
fn vector_is_zero_3d() {
    let zero = Vector::new(&[0, 0, 0]);
    let non_zero = Vector::new(&[1, 0, 0]);

    assert!(zero.is_zero());
    assert!(!non_zero.is_zero());
}

#[test]
fn vector_is_zero_negative_values_3d() {
    let non_zero_neg = Vector::new(&[-1, 0, 0]);
    assert!(!non_zero_neg.is_zero());
}

#[test]
fn vector_is_zero_negative_values_mixed_3d() {
    let non_zero_mixed = Vector::new(&[0, -1, 0]);
    assert!(!non_zero_mixed.is_zero());
}

// Tests for is_axis

#[test]
fn vector_is_axis_3d() {
    let axis_x = Vector::new(&[5, 0, 0]);
    let axis_y = Vector::new(&[0, 3, 0]);
    let axis_z = Vector::new(&[0, 0, -2]);
    let diagonal = Vector::new(&[3, 4, 0]);
    let zero = Vector::new(&[0, 0, 0]);

    assert!(axis_x.is_axis());
    assert!(axis_y.is_axis());
    assert!(axis_z.is_axis());
    assert!(!diagonal.is_axis());
    assert!(!zero.is_axis());
}

#[test]
fn vector_is_axis_negative_values_3d() {
    let axis_neg_x = Vector::new(&[-5, 0, 0]);
    let axis_neg_y = Vector::new(&[0, -3, 0]);

    assert!(axis_neg_x.is_axis());
    assert!(axis_neg_y.is_axis());
}

#[test]
fn vector_is_axis_multiple_non_zero_3d() {
    let non_axis = Vector::new(&[-1, 2, 0]);
    assert!(!non_axis.is_axis());

    let non_axis_all = Vector::new(&[-1, -2, -3]);
    assert!(!non_axis_all.is_axis());
}

// Tests for is (vector type)

#[test]
fn vector_type_3d() {
    let zero_3d = Vector::new(&[0, 0, 0]);
    let axis_3d = Vector::new(&[0, 0, 1]);
    let arbitrary_3d = Vector::new(&[1, 2, 3]);

    assert_eq!(zero_3d.is(), VectorType::Zero);
    assert_eq!(axis_3d.is(), VectorType::Axis);
    assert_eq!(arbitrary_3d.is(), VectorType::Arbitrary);
}

#[test]
fn vector_type_negative_values_3d() {
    let arbitrary_mixed = Vector::new(&[-1, 2, 0]);
    assert_eq!(arbitrary_mixed.is(), VectorType::Arbitrary);
}

// Tests for Index trait

#[test]
fn vector_index_operator_3d() {
    let vector = Vector::new(&[10, 20, 30]);
    assert_eq!(vector[0], 10);
    assert_eq!(vector[1], 20);
    assert_eq!(vector[2], 30);
}

#[test]
fn vector_index_operator_negative_values_3d() {
    let vector = Vector::new(&[-10, -20, 0]);
    assert_eq!(vector[0], -10);
    assert_eq!(vector[1], -20);
    assert_eq!(vector[2], 0);
}

// Tests for Display trait

#[test]
fn vector_display_3d() {
    let vector_3d = Vector::new(&[1, 2, 3]);
    assert_eq!(format!("{}", vector_3d), "(1,2,3)");
}

#[test]
fn vector_display_negative_values_3d() {
    let vector_mixed = Vector::new(&[-1, 0, 3]);
    assert_eq!(format!("{}", vector_mixed), "(-1,0,3)");
}

#[test]
fn vector_display_zeros_3d() {
    let vector_zero_3d = Vector::new(&[0, 0, 0]);
    assert_eq!(format!("{}", vector_zero_3d), "(0,0,0)");
}

// Tests for Add trait (vector + vector)

#[test]
fn vector_addition_3d() {
    let v1_3d = Vector::new(&[1, 2, 3]);
    let v2_3d = Vector::new(&[4, 5, 6]);
    let result_3d = v1_3d + v2_3d;

    assert_eq!(result_3d.get(0), &5);
    assert_eq!(result_3d.get(1), &7);
    assert_eq!(result_3d.get(2), &9);
}

// Tests for Sub trait (vector - vector)

#[test]
fn vector_subtraction_3d() {
    let v1_3d = Vector::new(&[10, 8, 6]);
    let v2_3d = Vector::new(&[3, 2, 1]);
    let result_3d = v1_3d - v2_3d;

    assert_eq!(result_3d.get(0), &7);
    assert_eq!(result_3d.get(1), &6);
    assert_eq!(result_3d.get(2), &5);
}

// Tests for Neg trait

#[test]
fn vector_negation_3d() {
    let vector = Vector::new(&[3, -4, 5]);
    let negated = -vector;

    assert_eq!(negated.get(0), &-3);
    assert_eq!(negated.get(1), &4);
    assert_eq!(negated.get(2), &-5);
}

#[test]
fn vector_negation_zeros_3d() {
    let vector_zero_3d = Vector::new(&[0, 0, 0]);
    let negated_3d = -vector_zero_3d;
    assert_eq!(negated_3d.get_coordinates(), &[0, 0, 0]);
}

// Tests for Mul trait (vector * scalar)

#[test]
fn vector_scalar_multiplication_vector_first_3d() {
    let vector_3d = Vector::new(&[1, -2, 3]);
    let scaled_3d = vector_3d * -2;

    assert_eq!(scaled_3d.get(0), &-2);
    assert_eq!(scaled_3d.get(1), &4);
    assert_eq!(scaled_3d.get(2), &-6);
}

#[test]
fn vector_scalar_multiplication_with_zero_3d() {
    let vector = Vector::new(&[5, -3, 7]);
    let scaled = vector * 0;

    assert_eq!(scaled.get_coordinates(), &[0, 0, 0]);
    assert!(scaled.is_zero());
}

#[test]
fn vector_scalar_multiplication_zero_vector_3d() {
    let vector = Vector::new(&[0, 0, 0]);
    let scaled = vector * 5;

    assert_eq!(scaled.get_coordinates(), &[0, 0, 0]);
    assert!(scaled.is_zero());
}

// Tests for Mul trait (scalar * vector)

#[test]
fn vector_scalar_multiplication_scalar_first_3d() {
    let vector_3d = Vector::new(&[1, -2, 3]);
    let scaled_3d = -2 * vector_3d;

    assert_eq!(scaled_3d.get(0), &-2);
    assert_eq!(scaled_3d.get(1), &4);
    assert_eq!(scaled_3d.get(2), &-6);
}

// Tests for Add trait (vector + point)

#[test]
fn vector_add_point_3d() {
    let vector_3d = Vector::new(&[10, 20, 30]);
    let point_3d = Point::new(&[1, 2, 3]);
    let result_3d = vector_3d + point_3d;

    assert_eq!(result_3d.get(0), &11);
    assert_eq!(result_3d.get(1), &22);
    assert_eq!(result_3d.get(2), &33);
}

#[test]
fn vector_add_point_negative_values_mixed_3d() {
    let vector_mixed = Vector::new(&[-5, 3]);
    let point_mixed = Point::new(&[2, -7]);
    let result_mixed = vector_mixed + point_mixed;
    assert_eq!(result_mixed.get_coordinates(), &[-3, -4]);
}

// Tests for Clone and Copy traits

#[test]
fn vector_clone_copy_3d() {
    let vector = Vector::new(&[1, 2, 3]);
    let cloned = vector.clone();
    let copied = vector;

    assert_eq!(vector, cloned);
    assert_eq!(vector, copied);
}

#[test]
fn vector_clone_copy_negative_values_3d() {
    let vector = Vector::new(&[-1, -2, -3]);
    let cloned = vector.clone();
    let copied = vector;

    assert_eq!(vector, cloned);
    assert_eq!(vector, copied);
    assert_eq!(cloned.get_coordinates(), &[-1, -2, -3]);
}

// Edge case tests

#[test]
fn vector_edge_cases_single_element() {
    // Test single element operations
    let single = Vector::new(&[42]);
    assert_eq!(single.get(0), &42);
    assert_eq!(single.manhattan_distance(), 42);
    assert_eq!(single.chebyshev_distance(), 42);
}

// Tests for from_points

#[test]
fn from_points_positive_values_3d() {
    let origin = Point::new(&[1, 2, 3]);
    let destination = Point::new(&[4, 6, 9]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(*vector.get_coordinates(), [3, 4, 6]);
}

#[test]
fn from_points_negative_values_3d() {
    let origin = Point::new(&[-2, -5, -1]);
    let destination = Point::new(&[-8, -1, -7]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(*vector.get_coordinates(), [-6, 4, -6]);
}

#[test]
fn from_points_mixed_values_3d() {
    let origin = Point::new(&[-3, 2, -5]);
    let destination = Point::new(&[5, -4, 10]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(*vector.get_coordinates(), [8, -6, 15]);
}

#[test]
fn from_points_zero_vector_3d() {
    let origin = Point::new(&[10, 15, -20]);
    let destination = Point::new(&[10, 15, -20]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(*vector.get_coordinates(), [0, 0, 0]);
    assert!(vector.is_zero());
}

#[test]
fn from_points_from_origin_3d() {
    let origin = Point::new(&[0, 0, 0]);
    let destination = Point::new(&[7, -3, 12]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(*vector.get_coordinates(), [7, -3, 12]);
}

#[test]
fn from_points_to_origin_3d() {
    let origin = Point::new(&[5, -8, 15]);
    let destination = Point::new(&[0, 0, 0]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(*vector.get_coordinates(), [-5, 8, -15]);
}

#[test]
fn from_points_extreme_values_i32_3d() {
    let origin: Point<i32, 3> =
        Point::new(&[minimum_coordinate_value(), maximum_coordinate_value(), 0]);
    let destination: Point<i32, 3> = Point::new(&[0, 0, minimum_coordinate_value()]);
    let vector: Vector<i32, 3> = Vector::from_points(&origin, &destination);
    assert_eq!(
        *vector.get_coordinates(),
        [
            -minimum_coordinate_value::<i32>(),
            -maximum_coordinate_value::<i32>(),
            minimum_coordinate_value()
        ]
    );
}

#[test]
fn from_points_different_types_i8_3d() {
    let origin = Point::new(&[5i8, -10i8, 20i8]);
    let destination = Point::new(&[15i8, 20i8, -30i8]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(*vector.get_coordinates(), [10i8, 30i8, -50i8]);
}

#[test]
fn from_points_different_types_i16_3d() {
    let origin = Point::new(&[100i16, -200i16, 300i16]);
    let destination = Point::new(&[300i16, 400i16, -100i16]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(*vector.get_coordinates(), [200i16, 600i16, -400i16]);
}

#[test]
fn from_points_different_types_i64_3d() {
    let origin = Point::new(&[1000i64, -2000i64, 5000i64]);
    let destination = Point::new(&[5000i64, 3000i64, -1000i64]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(*vector.get_coordinates(), [4000i64, 5000i64, -6000i64]);
}

#[test]
fn from_points_extreme_i8_3d() {
    let origin: Point<i8, 3> =
        Point::new(&[minimum_coordinate_value(), maximum_coordinate_value(), 0i8]);
    let destination: Point<i8, 3> = Point::new(&[0i8, 0i8, maximum_coordinate_value()]);
    let vector: Vector<i8, 3> = Vector::from_points(&origin, &destination);
    assert_eq!(
        *vector.get_coordinates(),
        [
            -minimum_coordinate_value::<i8>(),
            -maximum_coordinate_value::<i8>(),
            maximum_coordinate_value()
        ]
    );
}

#[test]
fn from_points_extreme_i16_3d() {
    let origin: Point<i16, 3> =
        Point::new(&[minimum_coordinate_value(), maximum_coordinate_value(), 0i16]);
    let destination: Point<i16, 3> = Point::new(&[0i16, 0i16, maximum_coordinate_value()]);
    let vector: Vector<i16, 3> = Vector::from_points(&origin, &destination);
    assert_eq!(
        *vector.get_coordinates(),
        [
            -minimum_coordinate_value::<i16>(),
            -maximum_coordinate_value::<i16>(),
            maximum_coordinate_value()
        ]
    );
}

#[test]
fn from_points_extreme_i64_3d() {
    let origin: Point<i64, 3> =
        Point::new(&[minimum_coordinate_value(), maximum_coordinate_value(), 0i64]);
    let destination: Point<i64, 3> = Point::new(&[0i64, 0i64, maximum_coordinate_value()]);
    let vector: Vector<i64, 3> = Vector::from_points(&origin, &destination);
    assert_eq!(
        *vector.get_coordinates(),
        [
            -minimum_coordinate_value::<i64>(),
            -maximum_coordinate_value::<i64>(),
            maximum_coordinate_value()
        ]
    );
}

#[test]
fn from_points_axis_aligned_vectors_3d() {
    // X-axis aligned
    let origin = Point::new(&[2, 5, 8]);
    let destination = Point::new(&[7, 5, 8]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(*vector.get_coordinates(), [5, 0, 0]);
    assert!(vector.is_axis());

    // Y-axis aligned
    let origin = Point::new(&[3, 1, -2]);
    let destination = Point::new(&[3, 8, -2]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(*vector.get_coordinates(), [0, 7, 0]);
    assert!(vector.is_axis());

    // Z-axis aligned
    let origin = Point::new(&[10, -5, 3]);
    let destination = Point::new(&[10, -5, 15]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(*vector.get_coordinates(), [0, 0, 12]);
    assert!(vector.is_axis());
}

#[test]
fn from_points_unit_vectors_3d() {
    // Unit vector in X direction
    let origin = Point::new(&[0, 0, 0]);
    let destination = Point::new(&[1, 0, 0]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(*vector.get_coordinates(), [1, 0, 0]);
    assert!(vector.is_axis());

    // Unit vector in Y direction
    let origin = Point::new(&[5, 10, -3]);
    let destination = Point::new(&[5, 11, -3]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(*vector.get_coordinates(), [0, 1, 0]);
    assert!(vector.is_axis());

    // Unit vector in Z direction
    let origin = Point::new(&[-2, 7, 0]);
    let destination = Point::new(&[-2, 7, 1]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(*vector.get_coordinates(), [0, 0, 1]);
    assert!(vector.is_axis());
}

#[test]
fn from_points_negative_unit_vectors_3d() {
    // Negative unit vector in X direction
    let origin = Point::new(&[1, 0, 0]);
    let destination = Point::new(&[0, 0, 0]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(*vector.get_coordinates(), [-1, 0, 0]);
    assert!(vector.is_axis());

    // Negative unit vector in Y direction
    let origin = Point::new(&[5, 11, -3]);
    let destination = Point::new(&[5, 10, -3]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(*vector.get_coordinates(), [0, -1, 0]);
    assert!(vector.is_axis());

    // Negative unit vector in Z direction
    let origin = Point::new(&[-2, 7, 1]);
    let destination = Point::new(&[-2, 7, 0]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(*vector.get_coordinates(), [0, 0, -1]);
    assert!(vector.is_axis());
}

#[test]
fn from_points_diagonal_vectors_3d() {
    // Diagonal vector with equal components
    let origin = Point::new(&[0, 0, 0]);
    let destination = Point::new(&[3, 3, 3]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(*vector.get_coordinates(), [3, 3, 3]);
    assert_eq!(vector.is(), VectorType::Arbitrary);

    // Diagonal vector with negative components
    let origin = Point::new(&[5, 5, 5]);
    let destination = Point::new(&[2, 2, 2]);
    let vector = Vector::from_points(&origin, &destination);
    assert_eq!(*vector.get_coordinates(), [-3, -3, -3]);
    assert_eq!(vector.is(), VectorType::Arbitrary);
}
*/
