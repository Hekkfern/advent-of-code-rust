/*use super::*;
use crate::vector::Vector;
use pretty_assertions::assert_eq;

// Tests for origin

#[test]
fn point_origin_3d() {
    let origin: Point<i32, 3> = Point::origin();
    assert_eq!(origin.get(0), &0);
    assert_eq!(origin.get(1), &0);
    assert_eq!(origin.get(2), &0);
    assert_eq!(origin.get_coordinates(), &[0, 0, 0]);
}

#[test]
fn point_origin_different_types_3d() {
    let origin_i16: Point<i16, 3> = Point::origin();
    assert_eq!(origin_i16.get_coordinates(), &[0i16, 0i16, 0i16]);
}

// Tests for new

#[test]
fn point_new_3d() {
    let point = Point::new(&[1, 2, 3]);
    assert_eq!(point.get(0), &1);
    assert_eq!(point.get(1), &2);
    assert_eq!(point.get(2), &3);
    assert_eq!(point.get_coordinates(), &[1, 2, 3]);
}

#[test]
fn point_new_different_types_3d() {
    let point_i16 = Point::new(&[10i16, 20i16, 30i16]);
    assert_eq!(point_i16.get(0), &10i16);
}

#[test]
fn point_new_with_zeros_3d() {
    let point_3d = Point::new(&[0, 0, 0]);
    assert_eq!(point_3d.get_coordinates(), &[0, 0, 0]);
}

#[test]
fn point_new_with_negative_values_3d() {
    let point_3d = Point::new(&[-1, -2, -3]);
    assert_eq!(point_3d.get_coordinates(), &[-1, -2, -3]);
}

#[test]
fn point_new_with_mixed_values_3d() {
    let point_3d = Point::new(&[0, -5, 3]);
    assert_eq!(point_3d.get_coordinates(), &[0, -5, 3]);
}

#[test]
fn point_new_with_extreme_values_3d() {
    let point_3d_extreme = Point::new(&[i32::MIN + 1, 0, i32::MAX]);
    assert_eq!(
        *point_3d_extreme.get_coordinates(),
        [i32::MIN + 1, 0, i32::MAX]
    );
}

// Tests for get_neighbours

#[test]
fn point_get_neighbours_3d() {
    let point = Point::new(&[1, 1, 1]);
    let neighbours = point.get_neighbours();

    assert_eq!(neighbours.len(), 6);
    assert!(neighbours.contains(&Point::new(&[2, 1, 1])));
    assert!(neighbours.contains(&Point::new(&[0, 1, 1])));
    assert!(neighbours.contains(&Point::new(&[1, 2, 1])));
    assert!(neighbours.contains(&Point::new(&[1, 0, 1])));
    assert!(neighbours.contains(&Point::new(&[1, 1, 2])));
    assert!(neighbours.contains(&Point::new(&[1, 1, 0])));
}

#[test]
fn point_get_neighbours_mixed_coordinates_3d() {
    let point_3d = Point::new(&[-1, 0, 2]);
    let neighbours = point_3d.get_neighbours();

    assert_eq!(neighbours.len(), 6);
    assert!(neighbours.contains(&Point::new(&[0, 0, 2])));
    assert!(neighbours.contains(&Point::new(&[-2, 0, 2])));
    assert!(neighbours.contains(&Point::new(&[-1, 1, 2])));
    assert!(neighbours.contains(&Point::new(&[-1, -1, 2])));
    assert!(neighbours.contains(&Point::new(&[-1, 0, 3])));
    assert!(neighbours.contains(&Point::new(&[-1, 0, 1])));
}

// Tests for Index trait

#[test]
fn point_index_operator_3d() {
    let point = Point::new(&[10, 20, 30]);
    assert_eq!(point[0], 10);
    assert_eq!(point[1], 20);
    assert_eq!(point[2], 30);
}

#[test]
fn point_index_operator_negative_values_3d() {
    let point = Point::new(&[-10, -20, 0]);
    assert_eq!(point[0], -10);
    assert_eq!(point[1], -20);
    assert_eq!(point[2], 0);
}

// Tests for Display trait

#[test]
fn point_display_3d() {
    let point_3d = Point::new(&[1, 2, 3]);
    assert_eq!(format!("{}", point_3d), "(1,2,3)");
}

#[test]
fn point_display_negative_values_3d() {
    let point_mixed = Point::new(&[-1, 0, 3]);
    assert_eq!(format!("{}", point_mixed), "(-1,0,3)");
}

#[test]
fn point_display_zeros_3d() {
    let point_zero_3d = Point::new(&[0, 0, 0]);
    assert_eq!(format!("{}", point_zero_3d), "(0,0,0)");
}

// Tests for Neg trait

#[test]
fn point_negation_3d() {
    let point_3d = Point::new(&[1, -2, 3]);
    let negated_3d = -point_3d;
    assert_eq!(negated_3d.get(0), &-1);
    assert_eq!(negated_3d.get(1), &2);
    assert_eq!(negated_3d.get(2), &-3);
}

#[test]
fn point_negation_zeros_3d() {
    let point_zero_3d = Point::new(&[0, 0, 0]);
    let negated_3d = -point_zero_3d;
    assert_eq!(negated_3d.get_coordinates(), &[0, 0, 0]);
}

// Tests for Add trait (point + vector)

#[test]
fn point_add_vector_3d() {
    let point = Point::new(&[1, 2, 3]);
    let vector = Vector::new(&[10, 20, 30]);
    let result = point + vector;

    assert_eq!(result.get(0), &11);
    assert_eq!(result.get(1), &22);
    assert_eq!(result.get(2), &33);
}

#[test]
fn point_add_vector_negative_values_3d() {
    let point_3d = Point::new(&[-1, 0, 2]);
    let vector_3d = Vector::new(&[1, -5, -2]);
    let result_3d = point_3d + vector_3d;
    assert_eq!(result_3d.get_coordinates(), &[0, -5, 0]);
}

#[test]
fn point_add_vector_zero_vector_3d() {
    let point_3d = Point::new(&[-1, 2, -3]);
    let zero_vector_3d = Vector::new(&[0, 0, 0]);
    let result_3d = point_3d + zero_vector_3d;
    assert_eq!(result_3d.get_coordinates(), &[-1, 2, -3]);
}

// Tests for Sub trait (point - vector)

#[test]
fn point_sub_vector_3d() {
    let point = Point::new(&[10, 15, 20]);
    let vector = Vector::new(&[3, 5, 7]);
    let result = point - vector;

    assert_eq!(result.get(0), &7);
    assert_eq!(result.get(1), &10);
    assert_eq!(result.get(2), &13);
}

#[test]
fn point_sub_vector_negative_values_3d() {
    let point_3d = Point::new(&[0, -5, 3]);
    let vector_3d = Vector::new(&[2, -1, -2]);
    let result_3d = point_3d - vector_3d;
    assert_eq!(result_3d.get_coordinates(), &[-2, -4, 5]);
}

#[test]
fn point_sub_vector_zero_vector_3d() {
    let point_3d = Point::new(&[-1, 2, -3]);
    let zero_vector_3d = Vector::new(&[0, 0, 0]);
    let result_3d = point_3d - zero_vector_3d;
    assert_eq!(result_3d.get_coordinates(), &[-1, 2, -3]);
}

// Tests for Clone and Copy traits

#[test]
fn point_clone_copy_3d() {
    let point = Point::new(&[1, 2, 3]);
    let cloned = point.clone();
    let copied = point;

    assert_eq!(point, cloned);
    assert_eq!(point, copied);
}

#[test]
fn point_clone_copy_negative_values_3d() {
    let point = Point::new(&[-1, -2, -3]);
    let cloned = point.clone();
    let copied = point;

    assert_eq!(point, cloned);
    assert_eq!(point, copied);
    assert_eq!(cloned.get_coordinates(), &[-1, -2, -3]);
}
*/
