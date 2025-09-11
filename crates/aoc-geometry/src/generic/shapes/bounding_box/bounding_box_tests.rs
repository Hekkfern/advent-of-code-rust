use super::*;
use crate::AxisDirection;
use crate::Point;
use crate::PositionStatus;

#[test]
fn bounding_box_update_and_min_max() {
    let mut bbox = BoundingBox::<i32, 3>::new();
    bbox.update(&Point::new([1, 2, 3]));
    bbox.update(&Point::new([-5, 10, 0]));
    bbox.update(&Point::new([7, -2, 8]));
    assert_eq!(*bbox.get_minimum(0), -5);
    assert_eq!(*bbox.get_minimum(1), -2);
    assert_eq!(*bbox.get_minimum(2), 0);
    assert_eq!(*bbox.get_maximum(0), 7);
    assert_eq!(*bbox.get_maximum(1), 10);
    assert_eq!(*bbox.get_maximum(2), 8);
}

#[test]
fn bounding_box_containment() {
    let mut bbox = BoundingBox::<i32, 3>::new();
    bbox.update(&Point::new([0, 0, 0]));
    bbox.update(&Point::new([10, 10, 10]));
    let inside = Point::new([5, 5, 5]);
    let border = Point::new([0, 10, 5]);
    let outside = Point::new([11, 5, 5]);
    assert!(!bbox.is_outside(&inside));
    assert!(!bbox.is_on_border(&inside));
    assert_eq!(bbox.check_position(&inside), PositionStatus::Inside);
    assert!(bbox.is_on_border(&border));
    assert_eq!(bbox.check_position(&border), PositionStatus::OnBorder);
    assert!(bbox.is_outside(&outside));
    assert_eq!(bbox.check_position(&outside), PositionStatus::Outside);
}

#[test]
fn bounding_box_empty() {
    let bbox = BoundingBox::<i32, 3>::new();
    // Should be at initial extreme values
    assert_eq!(*bbox.get_minimum(0), i32::MAX);
    assert_eq!(*bbox.get_maximum(0), i32::MIN);
}

#[test]
fn bounding_box_is_outside_for_axis() {
    let mut bbox = BoundingBox::<i32, 3>::new();
    bbox.update(&Point::new([0, 0, 0]));
    bbox.update(&Point::new([10, 10, 10]));
    let inside = Point::new([5, 5, 5]);
    let below_min = Point::new([-1, 5, 5]);
    let above_max = Point::new([11, 5, 5]);
    assert!(!bbox.is_outside_for_axis(0, &inside));
    assert!(bbox.is_outside_for_axis(0, &below_min));
    assert!(bbox.is_outside_for_axis(0, &above_max));
    // Axis 1 and 2
    assert!(!bbox.is_outside_for_axis(1, &inside));
    assert!(!bbox.is_outside_for_axis(2, &inside));
}

#[test]
fn bounding_box_is_outside_for_axis_and_direction() {
    let mut bbox = BoundingBox::<i32, 3>::new();
    bbox.update(&Point::new([0, 0, 0]));
    bbox.update(&Point::new([10, 10, 10]));
    let below_min = Point::new([-1, 5, 5]);
    let above_max = Point::new([11, 5, 5]);
    let at_min = Point::new([0, 5, 5]);
    let at_max = Point::new([10, 5, 5]);
    // Negative direction: below min
    assert!(bbox.is_outside_for_axis_and_direction(0, AxisDirection::Negative, &below_min));
    assert!(!bbox.is_outside_for_axis_and_direction(0, AxisDirection::Negative, &at_min));
    assert!(!bbox.is_outside_for_axis_and_direction(0, AxisDirection::Positive, &below_min));
    assert!(!bbox.is_outside_for_axis_and_direction(0, AxisDirection::Positive, &at_min));
    // Positive direction: above max
    assert!(bbox.is_outside_for_axis_and_direction(0, AxisDirection::Positive, &above_max));
    assert!(!bbox.is_outside_for_axis_and_direction(0, AxisDirection::Positive, &at_max));
    assert!(!bbox.is_outside_for_axis_and_direction(0, AxisDirection::Negative, &above_max));
    assert!(!bbox.is_outside_for_axis_and_direction(0, AxisDirection::Negative, &at_max));
    // Inside
    let inside = Point::new([5, 5, 5]);
    assert!(!bbox.is_outside_for_axis_and_direction(0, AxisDirection::Negative, &inside));
    assert!(!bbox.is_outside_for_axis_and_direction(0, AxisDirection::Positive, &inside));
}
