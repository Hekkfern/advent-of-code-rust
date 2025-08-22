use super::*;
use assertables::assert_some;
use pretty_assertions::{assert_eq, assert_ne};

// Tests for from_default_value

#[test]
fn from_default_value_primitive_values() {
    let values_to_test = [4, 0, -3];
    for value_to_test in values_to_test {
        let grid: Grid2D<i32> = Grid2D::from_default_value(&[3, 2], &value_to_test);
        // Check get_size and get_sizes
        assert_eq!(grid.get_size(0), 3);
        assert_eq!(grid.get_size(1), 2);
        assert_eq!(grid.get_sizes(), [3, 2]);
        // Check get_number_of_elements
        assert_eq!(grid.get_number_of_elements(), 6);
        for x in 0..3 {
            for y in 0..2 {
                let value = grid.get(&[x, y]);
                assert_some!(value);
                assert_eq!(value.unwrap(), &value_to_test);
            }
        }
    }
}

#[test]
fn from_default_value_custom_struct() {
    #[derive(Debug, PartialEq, Clone, Eq)]
    struct MyStruct {
        a: i32,
        b: String,
    }

    let default_value = MyStruct {
        a: 1,
        b: "default".to_string(),
    };
    let grid: Grid2D<MyStruct> = Grid2D::from_default_value(&[2, 2], &default_value);
    // Check get_size and get_sizes
    assert_eq!(grid.get_size(0), 2);
    assert_eq!(grid.get_size(1), 2);
    assert_eq!(grid.get_sizes(), [2, 2]);
    // Check get_number_of_elements
    assert_eq!(grid.get_number_of_elements(), 4);
    for x in 0..2 {
        for y in 0..2 {
            let coords = [x, y];
            let v = grid.get(&coords);
            assert_some!(v);
            assert_eq!(v.unwrap(), &default_value);
        }
    }
}

// Tests for from_vec
#[test]
fn from_vec_primitive_values() {
    let grid: Grid2D<i32> = Grid2D::from_vec(&[2, 3], vec![1, 2, 3, 4, 5, 6]);
    // Check get_size and get_sizes
    assert_eq!(grid.get_size(0), 2);
    assert_eq!(grid.get_size(1), 3);
    assert_eq!(grid.get_sizes(), [2, 3]);
    // Check get_number_of_elements
    assert_eq!(grid.get_number_of_elements(), 6);
    // Check get only
    for x in 0..2 {
        for y in 0..3 {
            let coords = [x, y];
            let value = grid.get(&coords);
            assert_some!(value);
            assert_eq!(*value.unwrap(), (x * 3 + y + 1) as i32);
        }
    }
}

#[test]
fn from_vec_custom_struct() {
    #[derive(Debug, PartialEq, Clone, Eq)]
    struct MyStruct {
        a: i32,
        b: &'static str,
    }
    let v1 = MyStruct { a: 1, b: "a" };
    let v2 = MyStruct { a: 2, b: "b" };
    let v3 = MyStruct { a: 3, b: "c" };
    let v4 = MyStruct { a: 4, b: "d" };
    let grid: Grid2D<MyStruct> = Grid2D::from_vec(
        &[2, 2],
        vec![v1.clone(), v2.clone(), v3.clone(), v4.clone()],
    );
    // Check get_size and get_sizes
    assert_eq!(grid.get_size(0), 2);
    assert_eq!(grid.get_size(1), 2);
    assert_eq!(grid.get_sizes(), [2, 2]);
    // Check get_number_of_elements
    assert_eq!(grid.get_number_of_elements(), 4);
    // Check get only
    assert_eq!(grid.get(&[0, 0]), Some(&v1));
    assert_eq!(grid.get(&[0, 1]), Some(&v2));
    assert_eq!(grid.get(&[1, 0]), Some(&v3));
    assert_eq!(grid.get(&[1, 1]), Some(&v4));
}

#[test]
#[should_panic(expected = "All dimensions must be greater than zero")]
fn from_vec_zero_size() {
    let _grid: Grid2D<i32> = Grid2D::from_vec(&[0, 0], vec![]);
}

#[test]
#[should_panic(expected = "Grid data length does not match specified dimensions")]
fn from_vec_wrong_length() {
    let _grid: Grid2D<i32> = Grid2D::from_vec(&[2, 2], vec![1, 2, 3]);
}

// Tests for rotate_counter_clockwise

#[test]
fn rotate_counter_clockwise_2x2() {
    let mut grid = Grid2D::from_vec(&[2, 2], vec![1, 2, 3, 4]);
    grid.rotate_counter_clockwise();
    // After rotation, shape should be [2, 2]
    assert_eq!(grid.get_sizes(), [2, 2]);
    // Original: [[1, 2], [3, 4]]
    // Rotated:  [[2, 4], [1, 3]]
    assert_eq!(grid.get(&[0, 0]), Some(&2));
    assert_eq!(grid.get(&[0, 1]), Some(&4));
    assert_eq!(grid.get(&[1, 0]), Some(&1));
    assert_eq!(grid.get(&[1, 1]), Some(&3));
}

#[test]
fn rotate_counter_clockwise_3x2() {
    let mut grid = Grid2D::from_vec(&[3, 2], vec![1, 2, 3, 4, 5, 6]);
    grid.rotate_counter_clockwise();
    // After rotation, shape should be [2, 3]
    assert_eq!(grid.get_sizes(), [2, 3]);
    // Original: [[1, 2], [3, 4], [5, 6]]
    // Rotated:  [[2, 4, 6], [1, 3, 5]]
    assert_eq!(grid.get(&[0, 0]), Some(&2));
    assert_eq!(grid.get(&[0, 1]), Some(&4));
    assert_eq!(grid.get(&[0, 2]), Some(&6));
    assert_eq!(grid.get(&[1, 0]), Some(&1));
    assert_eq!(grid.get(&[1, 1]), Some(&3));
    assert_eq!(grid.get(&[1, 2]), Some(&5));
}

// Tests for rotate_clockwise

#[test]
fn rotate_clockwise_2x2() {
    let mut grid = Grid2D::from_vec(&[2, 2], vec![1, 2, 3, 4]);
    grid.rotate_clockwise();
    // After rotation, shape should be [2, 2]
    assert_eq!(grid.get_sizes(), [2, 2]);
    // Original: [[1, 2], [3, 4]]
    // Rotated:  [[3, 1], [4, 2]]
    assert_eq!(grid.get(&[0, 0]), Some(&3));
    assert_eq!(grid.get(&[0, 1]), Some(&1));
    assert_eq!(grid.get(&[1, 0]), Some(&4));
    assert_eq!(grid.get(&[1, 1]), Some(&2));
}

#[test]
fn rotate_clockwise_3x2() {
    let mut grid = Grid2D::from_vec(&[3, 2], vec![1, 2, 3, 4, 5, 6]);
    grid.rotate_clockwise();
    // After rotation, shape should be [2, 3]
    assert_eq!(grid.get_sizes(), [2, 3]);
    // Original: [[1, 2], [3, 4], [5, 6]]
    // Rotated:  [[5, 3, 1], [6, 4, 2]]
    assert_eq!(grid.get(&[0, 0]), Some(&5));
    assert_eq!(grid.get(&[0, 1]), Some(&3));
    assert_eq!(grid.get(&[0, 2]), Some(&1));
    assert_eq!(grid.get(&[1, 0]), Some(&6));
    assert_eq!(grid.get(&[1, 1]), Some(&4));
    assert_eq!(grid.get(&[1, 2]), Some(&2));
}

// Tests for flip_horizontal

#[test]
fn flip_horizontal_2x2() {
    let mut grid = Grid2D::from_vec(&[2, 2], vec![1, 2, 3, 4]);
    grid.flip_horizontal();
    // After flip, shape should be [2, 2]
    assert_eq!(grid.get_sizes(), [2, 2]);
    // Original: [[1, 2], [3, 4]]
    // Flipped:  [[2, 1], [4, 3]]
    assert_eq!(grid.get(&[0, 0]), Some(&2));
    assert_eq!(grid.get(&[0, 1]), Some(&1));
    assert_eq!(grid.get(&[1, 0]), Some(&4));
    assert_eq!(grid.get(&[1, 1]), Some(&3));
}

#[test]
fn flip_horizontal_3x2() {
    let mut grid = Grid2D::from_vec(&[3, 2], vec![1, 2, 3, 4, 5, 6]);
    grid.flip_horizontal();
    // After flip, shape should be [3, 2]
    assert_eq!(grid.get_sizes(), [3, 2]);
    // Original: [[1, 2], [3, 4], [5, 6]]
    // Flipped:  [[2, 1], [4, 3], [6, 5]]
    assert_eq!(grid.get(&[0, 0]), Some(&2));
    assert_eq!(grid.get(&[0, 1]), Some(&1));
    assert_eq!(grid.get(&[1, 0]), Some(&4));
    assert_eq!(grid.get(&[1, 1]), Some(&3));
    assert_eq!(grid.get(&[2, 0]), Some(&6));
    assert_eq!(grid.get(&[2, 1]), Some(&5));
}

// Tests for flip_vertical

#[test]
fn flip_vertical_2x2() {
    let mut grid = Grid2D::from_vec(&[2, 2], vec![1, 2, 3, 4]);
    grid.flip_vertical();
    // After flip, shape should be [2, 2]
    assert_eq!(grid.get_sizes(), [2, 2]);
    // Original: [[1, 2], [3, 4]]
    // Flipped:  [[3, 4], [1, 2]]
    assert_eq!(grid.get(&[0, 0]), Some(&3));
    assert_eq!(grid.get(&[0, 1]), Some(&4));
    assert_eq!(grid.get(&[1, 0]), Some(&1));
    assert_eq!(grid.get(&[1, 1]), Some(&2));
}

#[test]
fn flip_vertical_3x2() {
    let mut grid = Grid2D::from_vec(&[3, 2], vec![1, 2, 3, 4, 5, 6]);
    grid.flip_vertical();
    // After flip, shape should be [3, 2]
    assert_eq!(grid.get_sizes(), [3, 2]);
    // Original: [[1, 2], [3, 4], [5, 6]]
    // Flipped:  [[5, 6], [3, 4], [1, 2]]
    assert_eq!(grid.get(&[0, 0]), Some(&5));
    assert_eq!(grid.get(&[0, 1]), Some(&6));
    assert_eq!(grid.get(&[1, 0]), Some(&3));
    assert_eq!(grid.get(&[1, 1]), Some(&4));
    assert_eq!(grid.get(&[2, 0]), Some(&1));
    assert_eq!(grid.get(&[2, 1]), Some(&2));
}

// Tests for position_status

#[test]
fn test_position_status_inside_center() {
    let grid = Grid2D::from_vec(&[3, 3], vec![1,2,3,4,5,6,7,8,9]);
    assert_eq!(grid.position_status(&[1, 1]), PositionStatus::Inside);
}

#[test]
fn test_position_status_on_border_0_1() {
    let grid = Grid2D::from_vec(&[3, 3], vec![1,2,3,4,5,6,7,8,9]);
    assert_eq!(grid.position_status(&[0, 1]), PositionStatus::OnBorder);
}

#[test]
fn test_position_status_on_border_1_0() {
    let grid = Grid2D::from_vec(&[3, 3], vec![1,2,3,4,5,6,7,8,9]);
    assert_eq!(grid.position_status(&[1, 0]), PositionStatus::OnBorder);
}

#[test]
fn test_position_status_on_border_2_2() {
    let grid = Grid2D::from_vec(&[3, 3], vec![1,2,3,4,5,6,7,8,9]);
    assert_eq!(grid.position_status(&[2, 2]), PositionStatus::OnBorder);
}

#[test]
fn test_position_status_outside_3_0() {
    let grid = Grid2D::from_vec(&[3, 3], vec![1,2,3,4,5,6,7,8,9]);
    assert_eq!(grid.position_status(&[3, 0]), PositionStatus::Outside);
}

#[test]
fn test_position_status_outside_0_3() {
    let grid = Grid2D::from_vec(&[3, 3], vec![1,2,3,4,5,6,7,8,9]);
    assert_eq!(grid.position_status(&[0, 3]), PositionStatus::Outside);
}

// Tests for is_outside

#[test]
fn test_is_outside_inside() {
    let grid = Grid2D::from_vec(&[3, 3], vec![1,2,3,4,5,6,7,8,9]);
    assert!(!grid.is_outside(&[0, 0]));
    assert!(!grid.is_outside(&[1, 1]));
    assert!(!grid.is_outside(&[2, 2]));
}

#[test]
fn test_is_outside_outside() {
    let grid = Grid2D::from_vec(&[3, 3], vec![1,2,3,4,5,6,7,8,9]);
    assert!(grid.is_outside(&[3, 0]));
    assert!(grid.is_outside(&[0, 3]));
}

// Tests for contains

#[test]
fn test_contains_true() {
    let grid = Grid2D::from_vec(&[3, 3], vec![1,2,3,4,5,6,7,8,9]);
    assert!(grid.contains(&[0, 0]));
    assert!(grid.contains(&[1, 1]));
    assert!(grid.contains(&[2, 2]));
}

#[test]
fn test_contains_false() {
    let grid = Grid2D::from_vec(&[3, 3], vec![1,2,3,4,5,6,7,8,9]);
    assert!(!grid.contains(&[3, 0]));
    assert!(!grid.contains(&[0, 3]));
}
