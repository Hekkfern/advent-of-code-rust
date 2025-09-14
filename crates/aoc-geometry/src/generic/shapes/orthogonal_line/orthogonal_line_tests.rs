use super::*;

// Tests for from_points

#[test]
fn from_points() {
    let p1 = Point::<i32, 2>::new([1, 2]);
    let p2 = Point::<i32, 2>::new([4, 2]);
    let line = OrthogonalLine::from_points(&p1, &p2);
    assert_eq!(line.length(), 3);
    assert_eq!(line.get_vertexes(), &[p1, p2]);
}

#[test]
#[should_panic(expected = "Points must be distinct to form a line.")]
fn from_points_same_point() {
    let p1 = Point::<i32, 2>::new([1, 2]);
    let _line = OrthogonalLine::from_points(&p1, &p1);
}

#[test]
fn from_points_reversed() {
    let p1 = Point::<i32, 2>::new([4, 2]);
    let p2 = Point::<i32, 2>::new([1, 2]);
    let line = OrthogonalLine::from_points(&p1, &p2);
    assert_eq!(line.length(), 3);
    assert_eq!(line.get_vertexes(), &[p1, p2]);
}

#[test]
#[should_panic(expected = "Orthogonal line must be axis-aligned.")]
fn from_points_no_orthogonal() {
    let p1 = Point::<i32, 2>::new([1, 7]);
    let p2 = Point::<i32, 2>::new([4, 2]);
    let _line = OrthogonalLine::from_points(&p1, &p2);
}

// Tests for from_point_and_vector

#[test]
fn from_point_and_vector() {
    let p1 = Point::<i32, 2>::new([1, 2]);
    let v = Vector::<i32, 2>::new([3, 0]);
    let line = OrthogonalLine::from_point_and_vector(&p1, &v);
    assert_eq!(line.get_vertexes()[0], p1);
    assert_eq!(line.get_vertexes()[1], Point::<i32, 2>::new([4, 2]));
}

#[test]
#[should_panic(expected = "Vector must be non-zero to form a line.")]
fn from_point_and_vector_zero_vector() {
    let p1 = Point::<i32, 2>::new([1, 2]);
    let v = Vector::<i32, 2>::zero();
    let _line = OrthogonalLine::from_point_and_vector(&p1, &v);
}

#[test]
#[should_panic(expected = "Orthogonal line must be axis-aligned.")]
fn from_point_and_vector_no_orthogonal() {
    let p1 = Point::<i32, 2>::new([1, 2]);
    let v = Vector::<i32, 2>::new([3, 3]);
    let _line = OrthogonalLine::from_point_and_vector(&p1, &v);
}

// Tests for get_axis

#[test]
fn get_axis() {
    let p1 = Point::<i32, 2>::new([1, 2]);
    let p2 = Point::<i32, 2>::new([4, 2]);
    let line = OrthogonalLine::from_points(&p1, &p2);
    assert_eq!(line.get_axis(), 0);
    let p3 = Point::<i32, 2>::new([1, 2]);
    let p4 = Point::<i32, 2>::new([1, 5]);
    let line2 = OrthogonalLine::from_points(&p3, &p4);
    assert_eq!(line2.get_axis(), 1);
}

// Tests for contains_point

#[test]
fn contains_point() {
    let p1 = Point::<i32, 2>::new([1, 2]);
    let p2 = Point::<i32, 2>::new([4, 2]);
    let line = OrthogonalLine::from_points(&p1, &p2);
    let inside = Point::<i32, 2>::new([2, 2]);
    let outside_not_aligned = Point::<i32, 2>::new([2, 3]);
    let outside_aligned = Point::<i32, 2>::new([7, 2]);
    assert!(line.contains_point(&inside));
    assert!(line.contains_point(&p1));
    assert!(line.contains_point(&p2));
    assert!(!line.contains_point(&outside_not_aligned));
    assert!(!line.contains_point(&outside_aligned));
}

// Tests for iter

#[test]
fn iter() {
    let p1 = Point::<i32, 2>::new([1, 2]);
    let p2 = Point::<i32, 2>::new([4, 2]);
    let line = OrthogonalLine::from_points(&p1, &p2);
    let points: Vec<_> = line.iter().collect();
    assert_eq!(
        points,
        vec![
            Point::<i32, 2>::new([1, 2]),
            Point::<i32, 2>::new([2, 2]),
            Point::<i32, 2>::new([3, 2]),
            Point::<i32, 2>::new([4, 2]),
        ]
    );
}

// Tests for overlaps

#[test]
fn overlaps_same_line() {
    let l1 =
        OrthogonalLine::from_points(&Point::<i32, 2>::new([1, 2]), &Point::<i32, 2>::new([4, 2]));
    assert!(l1.overlaps(&l1));
}

#[test]
fn overlaps_same_axis_overlapping() {
    let l1 =
        OrthogonalLine::from_points(&Point::<i32, 2>::new([1, 2]), &Point::<i32, 2>::new([4, 2]));
    let l2 =
        OrthogonalLine::from_points(&Point::<i32, 2>::new([3, 2]), &Point::<i32, 2>::new([5, 2]));
    assert!(l1.overlaps(&l2));
    assert!(l2.overlaps(&l1));
}

#[test]
fn overlaps_same_axis_no_overlapping_same_coord() {
    let l1 =
        OrthogonalLine::from_points(&Point::<i32, 2>::new([1, 2]), &Point::<i32, 2>::new([4, 2]));
    let l2 =
        OrthogonalLine::from_points(&Point::<i32, 2>::new([7, 2]), &Point::<i32, 2>::new([9, 2]));
    assert!(!l1.overlaps(&l2));
    assert!(!l2.overlaps(&l1));
}

#[test]
fn overlaps_same_axis_no_overlapping_diff_coord() {
    let l1 =
        OrthogonalLine::from_points(&Point::<i32, 2>::new([1, 2]), &Point::<i32, 2>::new([4, 2]));
    let l2 =
        OrthogonalLine::from_points(&Point::<i32, 2>::new([1, 3]), &Point::<i32, 2>::new([4, 3]));
    assert!(!l1.overlaps(&l2));
    assert!(!l2.overlaps(&l1));
}

#[test]
fn overlaps_other_axis_overlapping() {
    let l1 =
        OrthogonalLine::from_points(&Point::<i32, 2>::new([1, 2]), &Point::<i32, 2>::new([4, 2]));
    let l2 =
        OrthogonalLine::from_points(&Point::<i32, 2>::new([2, 1]), &Point::<i32, 2>::new([2, 5]));
    assert!(l1.overlaps(&l2));
    assert!(l2.overlaps(&l1));
}

#[test]
fn overlaps_other_axis_no_overlapping() {
    let l1 =
        OrthogonalLine::from_points(&Point::<i32, 2>::new([1, 2]), &Point::<i32, 2>::new([4, 2]));
    let l2 =
        OrthogonalLine::from_points(&Point::<i32, 2>::new([3, 7]), &Point::<i32, 2>::new([3, 9]));
    assert!(!l1.overlaps(&l2));
    assert!(!l2.overlaps(&l1));
}

// Tests for Display trait

#[test]
fn display() {
    let p1 = Point::<i32, 2>::new([1, 2]);
    let p2 = Point::<i32, 2>::new([4, 2]);
    let line = OrthogonalLine::from_points(&p1, &p2);
    assert_eq!(format!("{}", line), "[(1,2),(4,2)]");
}
