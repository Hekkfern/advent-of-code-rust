use super::*;
use assertables::assert_approx_eq;
use pretty_assertions::assert_eq;
use std::f64::consts::PI;

// Tests for calculate_number_of_intrinsic_points

#[test]
fn calculate_number_of_intrinsic_points_shape_1() {
    let vertices = vec![
        Point::<i32, 2>::new([0, 0]),
        Point::<i32, 2>::new([6, 0]),
        Point::<i32, 2>::new([6, -5]),
        Point::<i32, 2>::new([4, -5]),
        Point::<i32, 2>::new([4, -7]),
        Point::<i32, 2>::new([6, -7]),
        Point::<i32, 2>::new([6, -9]),
        Point::<i32, 2>::new([1, -9]),
        Point::<i32, 2>::new([1, -7]),
        Point::<i32, 2>::new([0, -7]),
        Point::<i32, 2>::new([0, -5]),
        Point::<i32, 2>::new([2, -5]),
        Point::<i32, 2>::new([2, -2]),
        Point::<i32, 2>::new([0, -2]),
    ];
    let polygon = OrthogonalPolygon2D::from_vertices(vertices);
    assert_eq!(polygon.number_of_intrinsic_points(), 24);
}

#[test]
fn calculate_number_of_intrinsic_points_shape_2() {
    let vertices = vec![
        Point::<i32, 2>::new([1, 0]),
        Point::<i32, 2>::new([3, 0]),
        Point::<i32, 2>::new([3, 2]),
        Point::<i32, 2>::new([1, 2]),
    ];
    let polygon = OrthogonalPolygon2D::from_vertices(vertices);
    assert_eq!(polygon.number_of_intrinsic_points(), 1);
}

// Tests for perimeter

#[test]
fn perimeter() {
    let vertices = vec![
        Point::<i32, 2>::new([0, 0]),
        Point::<i32, 2>::new([6, 0]),
        Point::<i32, 2>::new([6, -5]),
        Point::<i32, 2>::new([4, -5]),
        Point::<i32, 2>::new([4, -7]),
        Point::<i32, 2>::new([6, -7]),
        Point::<i32, 2>::new([6, -9]),
        Point::<i32, 2>::new([1, -9]),
        Point::<i32, 2>::new([1, -7]),
        Point::<i32, 2>::new([0, -7]),
        Point::<i32, 2>::new([0, -5]),
        Point::<i32, 2>::new([2, -5]),
        Point::<i32, 2>::new([2, -2]),
        Point::<i32, 2>::new([0, -2]),
    ];
    let polygon = OrthogonalPolygon2D::from_vertices(vertices);
    assert_eq!(polygon.perimeter(), 38);
}

// Tests for calculate_boundary_points

#[test]
fn calculate_boundary_points() {
    let vertices = vec![
        Point::<i32, 2>::new([0, 0]),
        Point::<i32, 2>::new([6, 0]),
        Point::<i32, 2>::new([6, -5]),
        Point::<i32, 2>::new([4, -5]),
        Point::<i32, 2>::new([4, -7]),
        Point::<i32, 2>::new([6, -7]),
        Point::<i32, 2>::new([6, -9]),
        Point::<i32, 2>::new([1, -9]),
        Point::<i32, 2>::new([1, -7]),
        Point::<i32, 2>::new([0, -7]),
        Point::<i32, 2>::new([0, -5]),
        Point::<i32, 2>::new([2, -5]),
        Point::<i32, 2>::new([2, -2]),
        Point::<i32, 2>::new([0, -2]),
    ];
    let polygon = OrthogonalPolygon2D::from_vertices(vertices);
    let boundary_points = polygon.get_boundary_points();
    assert_eq!(boundary_points.len(), 38);
}

// Tests for area

#[test]
fn area() {
    let vertices = vec![
        Point::<i32, 2>::new([0, 0]),
        Point::<i32, 2>::new([6, 0]),
        Point::<i32, 2>::new([6, -5]),
        Point::<i32, 2>::new([4, -5]),
        Point::<i32, 2>::new([4, -7]),
        Point::<i32, 2>::new([6, -7]),
        Point::<i32, 2>::new([6, -9]),
        Point::<i32, 2>::new([1, -9]),
        Point::<i32, 2>::new([1, -7]),
        Point::<i32, 2>::new([0, -7]),
        Point::<i32, 2>::new([0, -5]),
        Point::<i32, 2>::new([2, -5]),
        Point::<i32, 2>::new([2, -2]),
        Point::<i32, 2>::new([0, -2]),
    ];
    let polygon = OrthogonalPolygon2D::from_vertices(vertices);
    assert_approx_eq!(polygon.area(), 42.0);
}

// Tests for calculate_angle_between_vectors

#[test]
fn calculate_angle_between_vectors_same_vector() {
    let v1 = Vector::<i64, 2>::new([3, 4]);
    let v2 = v1.clone();
    let angle = calculate_angle_between_vectors(&v1, &v2);
    assert_approx_eq!(angle, 0.0);
}

#[test]
fn calculate_angle_between_vectors_90_degrees() {
    let v1 = Vector::<i64, 2>::new([1, 1]);
    let v2 = Vector::<i64, 2>::new([-1, 1]);
    {
        let angle = calculate_angle_between_vectors(&v1, &v2);
        assert_approx_eq!(angle, PI / 2.0);
    }
    {
        let angle = calculate_angle_between_vectors(&v2, &v1);
        assert_approx_eq!(angle, -PI / 2.0);
    }
}

#[test]
fn calculate_angle_between_vectors_180_degrees() {
    let v1 = Vector::<i64, 2>::new([1, 1]);
    let v2 = Vector::<i64, 2>::new([-1, -1]);
    {
        let angle = calculate_angle_between_vectors(&v1, &v2);
        assert_approx_eq!(angle, -PI);
    }
    {
        let angle = calculate_angle_between_vectors(&v2, &v1);
        assert_approx_eq!(angle, -PI);
    }
}

// Tests for is_outside, is_inside and is_on_edge

#[test]
fn is_outside() {
    let vertices = vec![
        Point::<i32, 2>::new([1, 0]),
        Point::<i32, 2>::new([3, 0]),
        Point::<i32, 2>::new([3, 2]),
        Point::<i32, 2>::new([1, 2]),
    ];
    let polygon = OrthogonalPolygon2D::from_vertices(vertices);
    let point = Point::<i32, 2>::new([10, 7]);
    assert!(polygon.is_outside(&point));
    assert!(!polygon.is_inside(&point));
    assert!(!polygon.is_on_edge(&point));
}

#[test]
fn is_inside() {
    let vertices = vec![
        Point::<i32, 2>::new([1, 0]),
        Point::<i32, 2>::new([3, 0]),
        Point::<i32, 2>::new([3, 2]),
        Point::<i32, 2>::new([1, 2]),
    ];
    let polygon = OrthogonalPolygon2D::from_vertices(vertices);
    let point = Point::<i32, 2>::new([2, 1]);
    assert!(!polygon.is_outside(&point));
    assert!(polygon.is_inside(&point));
    assert!(!polygon.is_on_edge(&point));
}

#[test]
fn is_on_edge() {
    let vertices = vec![
        Point::<i32, 2>::new([1, 0]),
        Point::<i32, 2>::new([3, 0]),
        Point::<i32, 2>::new([3, 2]),
        Point::<i32, 2>::new([1, 2]),
    ];
    let polygon = OrthogonalPolygon2D::from_vertices(vertices);
    let point = Point::<i32, 2>::new([1, 0]);
    assert!(!polygon.is_outside(&point));
    assert!(!polygon.is_inside(&point));
    assert!(polygon.is_on_edge(&point));
}
