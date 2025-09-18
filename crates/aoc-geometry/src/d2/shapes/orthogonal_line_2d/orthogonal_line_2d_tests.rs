use super::*;
use assertables::{assert_none, assert_some};
use pretty_assertions::assert_eq;

fn p(x: i32, y: i32) -> Point<i32, DIMENSIONS> {
    Point::new([x, y])
}

// Tests for from_points

#[test]
fn from_points() {
    let p1 = p(1, 2);
    let p2 = p(4, 2);
    let line = OrthogonalLine2D::from_points(&p1, &p2);
    assert_eq!(line.length(), 4);
    assert_eq!(line.get_vertexes(), &[p1, p2]);
}

#[test]
#[should_panic(expected = "Points must be distinct to form a line.")]
fn from_points_same_point() {
    let p1 = p(1, 2);
    let _line = OrthogonalLine2D::from_points(&p1, &p1);
}

#[test]
fn from_points_reversed() {
    let p1 = p(4, 2);
    let p2 = p(1, 2);
    let line = OrthogonalLine2D::from_points(&p1, &p2);
    assert_eq!(line.length(), 4);
    assert_eq!(line.get_vertexes(), &[p1, p2]);
}

#[test]
fn from_points_diagonal() {
    let p1 = p(1, 1);
    let p2 = p(3, 3);
    let line = OrthogonalLine2D::from_points(&p1, &p2);
    assert_eq!(line.length(), 3);
    assert_eq!(line.get_vertexes(), &[p1, p2]);
}

#[test]
#[should_panic(expected = "Orthogonal line must be axis-aligned or diagonal.")]
fn from_points_no_orthogonal() {
    let p1 = p(1, 7);
    let p2 = p(4, 2);
    let _line = OrthogonalLine2D::from_points(&p1, &p2);
}

// Tests for from_point_and_vector

#[test]
fn from_point_and_vector() {
    let p1 = p(1, 2);
    let v = Vector::<i32, DIMENSIONS>::new([3, 0]);
    let line = OrthogonalLine2D::from_point_and_vector(&p1, &v);
    assert_eq!(line.get_vertexes()[0], p1);
    assert_eq!(line.get_vertexes()[1], p(4, 2));
}

#[test]
#[should_panic(expected = "Vector must be non-zero to form a line.")]
fn from_point_and_vector_zero_vector() {
    let p1 = p(1, 2);
    let v = Vector::<i32, DIMENSIONS>::zero();
    let _line = OrthogonalLine2D::from_point_and_vector(&p1, &v);
}

#[test]
fn from_point_and_vector_diagonal() {
    let p1 = p(1, 2);
    let v = Vector::<i32, DIMENSIONS>::new([3, 3]);
    let line = OrthogonalLine2D::from_point_and_vector(&p1, &v);
    assert_eq!(line.get_vertexes()[0], p1);
    assert_eq!(line.get_vertexes()[1], p(4, 5));
}

#[test]
#[should_panic(expected = "Orthogonal line must be axis-aligned or diagonal.")]
fn from_point_and_vector_no_orthogonal() {
    let p1 = p(1, 2);
    let v = Vector::<i32, DIMENSIONS>::new([7, 3]);
    let _line = OrthogonalLine2D::from_point_and_vector(&p1, &v);
}

// Tests for get_axis

#[test]
fn get_axis_horitzontal() {
    let p1 = p(1, 2);
    let p2 = p(4, 2);
    let line = OrthogonalLine2D::from_points(&p1, &p2);
    let axis = line.get_axis();
    assert_some!(axis);
    let axis = axis.unwrap();
    assert_eq!(axis, 0);
}

#[test]
fn get_axis_vertical() {
    let p3 = p(1, 2);
    let p4 = p(1, 5);
    let line = OrthogonalLine2D::from_points(&p3, &p4);
    let axis = line.get_axis();
    assert_some!(axis);
    let axis = axis.unwrap();
    assert_eq!(axis, 1);
}

#[test]
fn get_axis_diagonal() {
    let p3 = p(1, 1);
    let p4 = p(3, 3);
    let line = OrthogonalLine2D::from_points(&p3, &p4);
    let axis = line.get_axis();
    assert_none!(axis);
}

// Tests for contains_point

#[test]
fn contains_point_horizontal() {
    let p1 = p(1, 2);
    let p2 = p(4, 2);
    let inside = p(2, 2);
    let outside_not_aligned = p(2, 3);
    let outside_aligned = p(7, 2);
    {
        let line = OrthogonalLine2D::from_points(&p1, &p2);
        assert!(line.contains_point(&inside));
        assert!(line.contains_point(&p1));
        assert!(line.contains_point(&p2));
        assert!(!line.contains_point(&outside_not_aligned));
        assert!(!line.contains_point(&outside_aligned));
    }
    {
        let line = OrthogonalLine2D::from_points(&p2, &p1);
        assert!(line.contains_point(&inside));
        assert!(line.contains_point(&p1));
        assert!(line.contains_point(&p2));
        assert!(!line.contains_point(&outside_not_aligned));
        assert!(!line.contains_point(&outside_aligned));
    }
}

#[test]
fn contains_point_vertical() {
    let p1 = p(2, 1);
    let p2 = p(2, 4);
    let inside = p(2, 2);
    let outside_not_aligned = p(3, 2);
    let outside_aligned = p(2, 7);
    {
        let line = OrthogonalLine2D::from_points(&p1, &p2);
        assert!(line.contains_point(&inside));
        assert!(line.contains_point(&p1));
        assert!(line.contains_point(&p2));
        assert!(!line.contains_point(&outside_not_aligned));
        assert!(!line.contains_point(&outside_aligned));
    }
    {
        let line = OrthogonalLine2D::from_points(&p2, &p1);
        assert!(line.contains_point(&inside));
        assert!(line.contains_point(&p1));
        assert!(line.contains_point(&p2));
        assert!(!line.contains_point(&outside_not_aligned));
        assert!(!line.contains_point(&outside_aligned));
    }
}

#[test]
fn contains_point_diagonal() {
    let p1 = p(0, 0);
    let p2 = p(4, -4);
    let inside = p(1, -1);
    let outside_not_aligned = p(3, 2);
    let outside_aligned = p(-2, 2);
    {
        let line = OrthogonalLine2D::from_points(&p1, &p2);
        assert!(line.contains_point(&inside));
        assert!(line.contains_point(&p1));
        assert!(line.contains_point(&p2));
        assert!(!line.contains_point(&outside_not_aligned));
        assert!(!line.contains_point(&outside_aligned));
    }
    {
        let line = OrthogonalLine2D::from_points(&p2, &p1);
        assert!(line.contains_point(&inside));
        assert!(line.contains_point(&p1));
        assert!(line.contains_point(&p2));
        assert!(!line.contains_point(&outside_not_aligned));
        assert!(!line.contains_point(&outside_aligned));
    }
}

// Tests for iter

#[test]
fn iter() {
    let p1 = p(1, 2);
    let p2 = p(4, 2);
    {
        let line = OrthogonalLine2D::from_points(&p1, &p2);
        let points: Vec<_> = line.iter().collect();
        assert_eq!(points, vec![p(1, 2), p(2, 2), p(3, 2), p(4, 2),]);
    }
    {
        let line = OrthogonalLine2D::from_points(&p2, &p1);
        let points: Vec<_> = line.iter().collect();
        assert_eq!(points, vec![p(4, 2), p(3, 2), p(2, 2), p(1, 2),]);
    }
}

// Tests for Display trait

#[test]
fn display() {
    let p1 = p(1, 2);
    let p2 = p(4, 2);
    let line = OrthogonalLine2D::from_points(&p1, &p2);
    assert_eq!(format!("{}", line), "[(1,2),(4,2)]");
}

// Tests for intersect, overlaps and is_collinear

// Horizontal vs horizontal

#[test]
fn intersect_horizontal_parallel() {
    let l1 = OrthogonalLine2D::from_points(&p(1, 2), &p(4, 2));
    let l2 = OrthogonalLine2D::from_points(&p(1, 3), &p(4, 3));
    assert!(!l1.overlaps(&l2));
    assert!(!l1.is_collinear(&l2));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

#[test]
fn intersect_horizontal_same_line() {
    let l1 = OrthogonalLine2D::from_points(&p(1, 2), &p(4, 2));
    assert!(l1.overlaps(&l1));
    assert!(l1.is_collinear(&l1));
    let intersect = l1.intersect(&l1);
    assert_eq!(intersect, l1.iter().collect::<Vec<_>>());
}

#[test]
fn intersect_horizontal_overlapping() {
    let l1 = OrthogonalLine2D::from_points(&p(1, 2), &p(4, 2));
    let l2 = OrthogonalLine2D::from_points(&p(3, 2), &p(5, 2));
    assert!(l1.overlaps(&l2));
    assert!(l1.is_collinear(&l2));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![p(3, 2), p(4, 2)]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

#[test]
fn intersect_horizontal_no_overlapping() {
    let l1 = OrthogonalLine2D::from_points(&p(1, 2), &p(4, 2));
    let l2 = OrthogonalLine2D::from_points(&p(7, 2), &p(9, 2));
    assert!(!l1.overlaps(&l2));
    assert!(l1.is_collinear(&l2));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

#[test]
fn intersect_horizontal_single_common_point() {
    let l1 = OrthogonalLine2D::from_points(&p(1, 2), &p(4, 2));
    let l2 = OrthogonalLine2D::from_points(&p(4, 2), &p(9, 2));
    assert!(l1.overlaps(&l2));
    assert!(l1.is_collinear(&l2));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![p(4, 2)]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

#[test]
fn intersect_horizontal_consecutive() {
    let l1 = OrthogonalLine2D::from_points(&p(1, 2), &p(4, 2));
    let l2 = OrthogonalLine2D::from_points(&p(5, 2), &p(9, 2));
    assert!(!l1.overlaps(&l2));
    assert!(l1.is_collinear(&l1));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

// Vertical vs Vertical

#[test]
fn intersect_vertical_parallel() {
    let l1 = OrthogonalLine2D::from_points(&p(2, 1), &p(2, 4));
    let l2 = OrthogonalLine2D::from_points(&p(3, 1), &p(3, 4));
    assert!(!l1.overlaps(&l2));
    assert!(!l1.is_collinear(&l2));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

#[test]
fn intersect_vertical_same_line() {
    let l1 = OrthogonalLine2D::from_points(&p(2, 1), &p(2, 4));
    assert!(l1.overlaps(&l1));
    assert!(l1.is_collinear(&l1));
    let intersect = l1.intersect(&l1);
    assert_eq!(intersect, l1.iter().collect::<Vec<_>>());
}

#[test]
fn intersect_vertical_overlapping() {
    let l1 = OrthogonalLine2D::from_points(&p(2, 1), &p(2, 4));
    let l2 = OrthogonalLine2D::from_points(&p(2, 3), &p(2, 5));
    assert!(l1.overlaps(&l2));
    assert!(l1.is_collinear(&l2));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![p(2, 3), p(2, 4)]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

#[test]
fn intersect_vertical_no_overlapping() {
    let l1 = OrthogonalLine2D::from_points(&p(2, 1), &p(2, 4));
    let l2 = OrthogonalLine2D::from_points(&p(2, 7), &p(2, 9));
    assert!(!l1.overlaps(&l2));
    assert!(l1.is_collinear(&l2));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

#[test]
fn intersect_vertical_single_common_point() {
    let l1 = OrthogonalLine2D::from_points(&p(2, 1), &p(2, 4));
    let l2 = OrthogonalLine2D::from_points(&p(2, 4), &p(2, 9));
    assert!(l1.overlaps(&l2));
    assert!(l1.is_collinear(&l2));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![p(2, 4)]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

#[test]
fn intersect_vertical_consecutive() {
    let l1 = OrthogonalLine2D::from_points(&p(2, 1), &p(2, 4));
    let l2 = OrthogonalLine2D::from_points(&p(2, 5), &p(2, 9));
    assert!(!l1.overlaps(&l2));
    assert!(l1.is_collinear(&l2));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

// Diagonal vs Diagonal

#[test]
fn intersect_diagonal_parallel() {
    let l1 = OrthogonalLine2D::from_points(&p(1, 1), &p(4, 4));
    let l2 = OrthogonalLine2D::from_points(&p(2, 1), &p(5, 4));
    assert!(!l1.overlaps(&l2));
    assert!(!l1.is_collinear(&l2));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

#[test]
fn intersect_diagonal_same_line() {
    let l1 = OrthogonalLine2D::from_points(&p(1, 1), &p(4, 4));
    assert!(l1.overlaps(&l1));
    assert!(l1.is_collinear(&l1));
    let intersect = l1.intersect(&l1);
    assert_eq!(intersect, l1.iter().collect::<Vec<_>>());
}

#[test]
fn intersect_diagonal_overlapping() {
    let l1 = OrthogonalLine2D::from_points(&p(1, 1), &p(4, 4));
    let l2 = OrthogonalLine2D::from_points(&p(3, 3), &p(6, 6));
    assert!(l1.overlaps(&l2));
    assert!(l1.is_collinear(&l2));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![p(3, 3), p(4, 4)]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

#[test]
fn intersect_diagonal_no_overlapping() {
    let l1 = OrthogonalLine2D::from_points(&p(1, 1), &p(4, 4));
    let l2 = OrthogonalLine2D::from_points(&p(7, 7), &p(9, 9));
    assert!(!l1.overlaps(&l2));
    assert!(l1.is_collinear(&l2));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

#[test]
fn intersect_diagonal_single_common_point() {
    let l1 = OrthogonalLine2D::from_points(&p(1, 1), &p(4, 4));
    let l2 = OrthogonalLine2D::from_points(&p(4, 4), &p(9, 9));
    assert!(l1.overlaps(&l2));
    assert!(l1.is_collinear(&l2));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![p(4, 4)]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

#[test]
fn intersect_diagonal_consecutive() {
    let l1 = OrthogonalLine2D::from_points(&p(1, 1), &p(4, 4));
    let l2 = OrthogonalLine2D::from_points(&p(5, 5), &p(9, 9));
    assert!(!l1.overlaps(&l2));
    assert!(l1.is_collinear(&l2));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

#[test]
fn intersect_diagonal_crossing_middle() {
    let l1 = OrthogonalLine2D::from_points(&p(8, 0), &p(0, 8));
    let l2 = OrthogonalLine2D::from_points(&p(6, 4), &p(2, 0));
    assert!(l1.overlaps(&l2));
    assert!(!l1.is_collinear(&l2));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![p(5, 3)]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

#[test]
fn intersect_diagonal_crossing_side() {
    let l1 = OrthogonalLine2D::from_points(&p(8, 0), &p(0, 8));
    let l2 = OrthogonalLine2D::from_points(&p(5, 3), &p(2, 0));
    assert!(l1.overlaps(&l2));
    assert!(!l1.is_collinear(&l2));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![p(5, 3)]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

#[test]
fn intersect_diagonal_crossing_vertexes() {
    let l1 = OrthogonalLine2D::from_points(&p(8, 0), &p(0, 8));
    let l2 = OrthogonalLine2D::from_points(&p(8, 0), &p(10, 2));
    assert!(l1.overlaps(&l2));
    assert!(!l1.is_collinear(&l2));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![p(8, 0)]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

// Horizontal vs Vertical

#[test]
fn intersect_h_vs_v_crossing_middle() {
    let l1 = OrthogonalLine2D::from_points(&p(1, 2), &p(4, 2));
    let l2 = OrthogonalLine2D::from_points(&p(2, 1), &p(2, 5));
    assert!(l1.overlaps(&l2));
    assert!(!l1.is_collinear(&l2));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![p(2, 2)]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

#[test]
fn intersect_h_vs_v_crossing_side() {
    let l1 = OrthogonalLine2D::from_points(&p(1, 2), &p(4, 2));
    let l2 = OrthogonalLine2D::from_points(&p(4, 1), &p(4, 5));
    assert!(l1.overlaps(&l2));
    assert!(!l1.is_collinear(&l2));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![p(4, 2)]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

#[test]
fn intersect_h_vs_v_crossing_vertexes() {
    let l1 = OrthogonalLine2D::from_points(&p(1, 2), &p(4, 2));
    let l2 = OrthogonalLine2D::from_points(&p(4, 2), &p(4, 6));
    assert!(l1.overlaps(&l2));
    assert!(!l1.is_collinear(&l2));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![p(4, 2)]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

#[test]
fn intersect_h_vs_v_no_crossing() {
    let l1 = OrthogonalLine2D::from_points(&p(1, 2), &p(4, 2));
    let l2 = OrthogonalLine2D::from_points(&p(7, 1), &p(7, 5));
    assert!(!l1.overlaps(&l2));
    assert!(!l1.is_collinear(&l2));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

// Horizontal vs Diagonal

#[test]
fn intersect_h_vs_d_crossing_middle() {
    let l1 = OrthogonalLine2D::from_points(&p(1, 2), &p(4, 2));
    let l2 = OrthogonalLine2D::from_points(&p(2, 1), &p(5, 4));
    assert!(l1.overlaps(&l2));
    assert!(!l1.is_collinear(&l2));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![p(3, 2)]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

#[test]
fn intersect_h_vs_d_crossing_side() {
    let l1 = OrthogonalLine2D::from_points(&p(1, 2), &p(4, 2));
    let l2 = OrthogonalLine2D::from_points(&p(3, 1), &p(6, 4));
    assert!(l1.overlaps(&l2));
    assert!(!l1.is_collinear(&l2));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![p(4, 2)]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

#[test]
fn intersect_h_vs_d_crossing_vertexes() {
    let l1 = OrthogonalLine2D::from_points(&p(1, 2), &p(4, 2));
    let l2 = OrthogonalLine2D::from_points(&p(4, 2), &p(6, 4));
    assert!(l1.overlaps(&l2));
    assert!(!l1.is_collinear(&l2));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![p(4, 2)]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

#[test]
fn intersect_h_vs_d_no_crossing() {
    let l1 = OrthogonalLine2D::from_points(&p(1, 2), &p(4, 2));
    let l2 = OrthogonalLine2D::from_points(&p(7, 1), &p(9, 3));
    assert!(!l1.overlaps(&l2));
    assert!(!l1.is_collinear(&l2));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

// Vertical vs Diagonal

#[test]
fn intersect_v_vs_d_crossing_middle() {
    let l1 = OrthogonalLine2D::from_points(&p(2, 1), &p(2, 4));
    let l2 = OrthogonalLine2D::from_points(&p(0, 4), &p(3, 1));
    assert!(l1.overlaps(&l2));
    assert!(!l1.is_collinear(&l2));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![p(2, 2)]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

#[test]
fn intersect_v_vs_d_crossing_side() {
    let l1 = OrthogonalLine2D::from_points(&p(2, 1), &p(2, 4));
    let l2 = OrthogonalLine2D::from_points(&p(0, 3), &p(3, 0));
    assert!(l1.overlaps(&l2));
    assert!(!l1.is_collinear(&l2));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![p(2, 1)]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

#[test]
fn intersect_v_vs_d_crossing_vertexes() {
    let l1 = OrthogonalLine2D::from_points(&p(2, 1), &p(2, 4));
    let l2 = OrthogonalLine2D::from_points(&p(-1, 4), &p(2, 1));
    assert!(l1.overlaps(&l2));
    assert!(!l1.is_collinear(&l2));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![p(2, 1)]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

#[test]
fn intersect_v_vs_d_no_crossing() {
    let l1 = OrthogonalLine2D::from_points(&p(2, 1), &p(2, 4));
    let l2 = OrthogonalLine2D::from_points(&p(7, 1), &p(9, 3));
    assert!(!l1.overlaps(&l2));
    assert!(!l1.is_collinear(&l2));
    let intersect1 = l1.intersect(&l2);
    assert_eq!(intersect1, vec![]);
    let intersect2 = l2.intersect(&l1);
    assert_eq!(intersect2, intersect1);
}

