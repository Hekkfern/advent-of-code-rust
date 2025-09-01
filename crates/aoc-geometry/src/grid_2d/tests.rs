use super::*;
use assertables::assert_some;
use pretty_assertions::assert_eq;
use std::collections::HashSet;

fn coord(x: usize, y: usize) -> Coordinate {
    Coordinate::new([x, y])
}

// Tests for from_default_value

#[test]
fn from_default_value_primitive_values() {
    let values_to_test = [4, 0, -3];
    for value_to_test in values_to_test {
        let grid: Grid2D<i32> = Grid2D::from_default_value(3, 2, &value_to_test);
        // Check get_size and get_sizes
        assert_eq!(grid.get_height(), 2);
        assert_eq!(grid.get_width(), 3);
        assert_eq!(grid.get_sizes(), (3, 2));
        // Check get_number_of_elements
        assert_eq!(grid.get_number_of_elements(), 6);
        for x in 0..3 {
            for y in 0..2 {
                let value = grid.get(&coord(x, y));
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
    let grid: Grid2D<MyStruct> = Grid2D::from_default_value(3, 2, &default_value);
    // Check get_size and get_sizes
    assert_eq!(grid.get_height(), 2);
    assert_eq!(grid.get_width(), 3);
    assert_eq!(grid.get_sizes(), (3, 2));
    // Check get_number_of_elements
    assert_eq!(grid.get_number_of_elements(), 6);
    for x in 0..3 {
        for y in 0..2 {
            let coords = coord(x, y);
            let v = grid.get(&coords);
            assert_some!(v);
            assert_eq!(v.unwrap(), &default_value);
        }
    }
}

// Tests for from_single_vec

#[test]
fn from_single_vec_primitive_values() {
    let grid: Grid2D<i32> = Grid2D::from_single_vec(3, 2, vec![1, 2, 3, 4, 5, 6]);
    // Check get_size and get_sizes
    assert_eq!(grid.get_height(), 2);
    assert_eq!(grid.get_width(), 3);
    assert_eq!(grid.get_sizes(), (3, 2));
    // Check get_number_of_elements
    assert_eq!(grid.get_number_of_elements(), 6);
    // Check get only
    assert_eq!(grid.get(&coord(0, 0)), Some(&1));
    assert_eq!(grid.get(&coord(1, 0)), Some(&2));
    assert_eq!(grid.get(&coord(2, 0)), Some(&3));
    assert_eq!(grid.get(&coord(0, 1)), Some(&4));
    assert_eq!(grid.get(&coord(1, 1)), Some(&5));
    assert_eq!(grid.get(&coord(2, 1)), Some(&6));
}

#[test]
fn from_single_vec_custom_struct() {
    #[derive(Debug, PartialEq, Clone, Eq)]
    struct MyStruct {
        a: i32,
        b: &'static str,
    }
    let v1 = MyStruct { a: 1, b: "a" };
    let v2 = MyStruct { a: 2, b: "b" };
    let v3 = MyStruct { a: 3, b: "c" };
    let v4 = MyStruct { a: 4, b: "d" };
    let v5 = MyStruct { a: 5, b: "e" };
    let v6 = MyStruct { a: 6, b: "f" };
    let grid: Grid2D<MyStruct> = Grid2D::from_single_vec(
        3,
        2,
        vec![
            v1.clone(),
            v2.clone(),
            v3.clone(),
            v4.clone(),
            v5.clone(),
            v6.clone(),
        ],
    );
    // Check get_size and get_sizes
    assert_eq!(grid.get_height(), 2);
    assert_eq!(grid.get_width(), 3);
    assert_eq!(grid.get_sizes(), (3, 2));
    // Check get_number_of_elements
    assert_eq!(grid.get_number_of_elements(), 6);
    // Check get only
    assert_eq!(grid.get(&coord(0, 0)), Some(&v1));
    assert_eq!(grid.get(&coord(1, 0)), Some(&v2));
    assert_eq!(grid.get(&coord(2, 0)), Some(&v3));
    assert_eq!(grid.get(&coord(0, 1)), Some(&v4));
    assert_eq!(grid.get(&coord(1, 1)), Some(&v5));
    assert_eq!(grid.get(&coord(2, 1)), Some(&v6));
}

#[test]
#[should_panic(expected = "All dimensions must be greater than zero")]
fn from_single_vec_zero_size() {
    let _grid: Grid2D<i32> = Grid2D::from_single_vec(0, 0, vec![]);
}

#[test]
#[should_panic(expected = "Grid data length does not match specified dimensions")]
fn from_single_vec_wrong_length() {
    let _grid: Grid2D<i32> = Grid2D::from_single_vec(2, 2, vec![1, 2, 3]);
}

// Tests for from_double_vec

#[test]
fn from_double_vec_primitive_values() {
    let grid: Grid2D<i32> = Grid2D::from_double_vec(vec![vec![1, 2, 3], vec![4, 5, 6]]);
    // Check get_size and get_sizes
    assert_eq!(grid.get_height(), 2);
    assert_eq!(grid.get_width(), 3);
    assert_eq!(grid.get_sizes(), (3, 2));
    // Check get_number_of_elements
    assert_eq!(grid.get_number_of_elements(), 6);
    // Check get only
    assert_eq!(grid.get(&coord(0, 0)), Some(&1));
    assert_eq!(grid.get(&coord(1, 0)), Some(&2));
    assert_eq!(grid.get(&coord(2, 0)), Some(&3));
    assert_eq!(grid.get(&coord(0, 1)), Some(&4));
    assert_eq!(grid.get(&coord(1, 1)), Some(&5));
    assert_eq!(grid.get(&coord(2, 1)), Some(&6));
}

#[test]
fn from_double_vec_custom_struct() {
    #[derive(Debug, PartialEq, Clone, Eq)]
    struct MyStruct {
        a: i32,
        b: &'static str,
    }
    let v1 = MyStruct { a: 1, b: "a" };
    let v2 = MyStruct { a: 2, b: "b" };
    let v3 = MyStruct { a: 3, b: "c" };
    let v4 = MyStruct { a: 4, b: "d" };
    let v5 = MyStruct { a: 5, b: "e" };
    let v6 = MyStruct { a: 6, b: "f" };
    let grid: Grid2D<MyStruct> = Grid2D::from_double_vec(vec![
        vec![v1.clone(), v2.clone(), v3.clone()],
        vec![v4.clone(), v5.clone(), v6.clone()],
    ]);
    // Check get_size and get_sizes
    assert_eq!(grid.get_height(), 2);
    assert_eq!(grid.get_width(), 3);
    assert_eq!(grid.get_sizes(), (3, 2));
    // Check get_number_of_elements
    assert_eq!(grid.get_number_of_elements(), 6);
    // Check get only
    assert_eq!(grid.get(&coord(0, 0)), Some(&v1));
    assert_eq!(grid.get(&coord(1, 0)), Some(&v2));
    assert_eq!(grid.get(&coord(2, 0)), Some(&v3));
    assert_eq!(grid.get(&coord(0, 1)), Some(&v4));
    assert_eq!(grid.get(&coord(1, 1)), Some(&v5));
    assert_eq!(grid.get(&coord(2, 1)), Some(&v6));
}

#[test]
#[should_panic(expected = "Grid data cannot be empty")]
fn from_double_vec_empty() {
    let _grid: Grid2D<i32> = Grid2D::from_double_vec(vec![]);
}

#[test]
#[should_panic(expected = "All rows must have the same number of columns")]
fn from_double_vec_irregular_rows() {
    let _grid: Grid2D<i32> = Grid2D::from_double_vec(vec![vec![1, 2], vec![3]]);
}

// Tests for rotate_counter_clockwise

#[test]
fn rotate_counter_clockwise_2x2() {
    // Original: [[1, 2], [3, 4]]
    let mut grid = Grid2D::from_single_vec(2, 2, vec![1, 2, 3, 4]);
    assert_eq!(grid.get(&coord(0, 0)), Some(&1));
    assert_eq!(grid.get(&coord(1, 0)), Some(&2));
    assert_eq!(grid.get(&coord(0, 1)), Some(&3));
    assert_eq!(grid.get(&coord(1, 1)), Some(&4));
    // Rotated:  [[3, 1], [4, 2]]
    grid.rotate_counter_clockwise();
    assert_eq!(grid.get_sizes(), (2, 2));
    assert_eq!(grid.get(&coord(0, 0)), Some(&3));
    assert_eq!(grid.get(&coord(1, 0)), Some(&1));
    assert_eq!(grid.get(&coord(0, 1)), Some(&4));
    assert_eq!(grid.get(&coord(1, 1)), Some(&2));
}

#[test]
fn rotate_counter_clockwise_3x2() {
    // Original: [[1, 2, 3], [4, 5, 6]]
    let mut grid = Grid2D::from_single_vec(3, 2, vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(grid.get(&coord(0, 0)), Some(&1));
    assert_eq!(grid.get(&coord(1, 0)), Some(&2));
    assert_eq!(grid.get(&coord(2, 0)), Some(&3));
    assert_eq!(grid.get(&coord(0, 1)), Some(&4));
    assert_eq!(grid.get(&coord(1, 1)), Some(&5));
    assert_eq!(grid.get(&coord(2, 1)), Some(&6));
    // Rotated:  [[4, 1], [5, 2], [6, 3]]
    grid.rotate_counter_clockwise();
    assert_eq!(grid.get_sizes(), (2, 3));
    assert_eq!(grid.get(&coord(0, 0)), Some(&4));
    assert_eq!(grid.get(&coord(1, 0)), Some(&1));
    assert_eq!(grid.get(&coord(0, 1)), Some(&5));
    assert_eq!(grid.get(&coord(1, 1)), Some(&2));
    assert_eq!(grid.get(&coord(0, 2)), Some(&6));
    assert_eq!(grid.get(&coord(1, 2)), Some(&3));
}

// Tests for rotate_clockwise

#[test]
fn rotate_clockwise_2x2() {
    // Original: [[1, 2], [3, 4]]
    let mut grid = Grid2D::from_single_vec(2, 2, vec![1, 2, 3, 4]);
    assert_eq!(grid.get(&coord(0, 0)), Some(&1));
    assert_eq!(grid.get(&coord(1, 0)), Some(&2));
    assert_eq!(grid.get(&coord(0, 1)), Some(&3));
    assert_eq!(grid.get(&coord(1, 1)), Some(&4));
    // Rotated:  [[2, 4], [1, 3]]
    grid.rotate_clockwise();
    assert_eq!(grid.get_sizes(), (2, 2));
    assert_eq!(grid.get(&coord(0, 0)), Some(&2));
    assert_eq!(grid.get(&coord(1, 0)), Some(&4));
    assert_eq!(grid.get(&coord(0, 1)), Some(&1));
    assert_eq!(grid.get(&coord(1, 1)), Some(&3));
}

#[test]
fn rotate_clockwise_3x2() {
    // Original: [[1, 2, 3], [4, 5, 6]]
    let mut grid = Grid2D::from_single_vec(3, 2, vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(grid.get(&coord(0, 0)), Some(&1));
    assert_eq!(grid.get(&coord(1, 0)), Some(&2));
    assert_eq!(grid.get(&coord(2, 0)), Some(&3));
    assert_eq!(grid.get(&coord(0, 1)), Some(&4));
    assert_eq!(grid.get(&coord(1, 1)), Some(&5));
    assert_eq!(grid.get(&coord(2, 1)), Some(&6));
    // Rotated:  [[5, 3, 1], [6, 4, 2]]
    grid.rotate_clockwise();
    // After rotation, shape should be [2, 3]
    assert_eq!(grid.get_sizes(), (2, 3));
    assert_eq!(grid.get(&coord(0, 0)), Some(&3));
    assert_eq!(grid.get(&coord(1, 0)), Some(&6));
    assert_eq!(grid.get(&coord(0, 1)), Some(&2));
    assert_eq!(grid.get(&coord(1, 1)), Some(&5));
    assert_eq!(grid.get(&coord(0, 2)), Some(&1));
    assert_eq!(grid.get(&coord(1, 2)), Some(&4));
}

// Tests for flip_horizontal

#[test]
fn flip_horizontal_2x2() {
    // Original: [[1, 2], [3, 4]]
    let mut grid = Grid2D::from_single_vec(2, 2, vec![1, 2, 3, 4]);
    assert_eq!(grid.get(&coord(0, 0)), Some(&1));
    assert_eq!(grid.get(&coord(1, 0)), Some(&2));
    assert_eq!(grid.get(&coord(0, 1)), Some(&3));
    assert_eq!(grid.get(&coord(1, 1)), Some(&4));
    // Flipped:  [[2, 1], [4, 3]]
    grid.flip_horizontal();
    assert_eq!(grid.get_sizes(), (2, 2));
    assert_eq!(grid.get(&coord(0, 0)), Some(&2));
    assert_eq!(grid.get(&coord(1, 0)), Some(&1));
    assert_eq!(grid.get(&coord(0, 1)), Some(&4));
    assert_eq!(grid.get(&coord(1, 1)), Some(&3));
}

#[test]
fn flip_horizontal_3x2() {
    // Original: [[1, 2, 3], [4, 5, 6]]
    let mut grid = Grid2D::from_single_vec(3, 2, vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(grid.get(&coord(0, 0)), Some(&1));
    assert_eq!(grid.get(&coord(1, 0)), Some(&2));
    assert_eq!(grid.get(&coord(2, 0)), Some(&3));
    assert_eq!(grid.get(&coord(0, 1)), Some(&4));
    assert_eq!(grid.get(&coord(1, 1)), Some(&5));
    assert_eq!(grid.get(&coord(2, 1)), Some(&6));
    // Flipped:  [[3, 2, 1], [6, 5, 4]]
    grid.flip_horizontal();
    assert_eq!(grid.get_sizes(), (3, 2));
    assert_eq!(grid.get(&coord(0, 0)), Some(&3));
    assert_eq!(grid.get(&coord(1, 0)), Some(&2));
    assert_eq!(grid.get(&coord(2, 0)), Some(&1));
    assert_eq!(grid.get(&coord(0, 1)), Some(&6));
    assert_eq!(grid.get(&coord(1, 1)), Some(&5));
    assert_eq!(grid.get(&coord(2, 1)), Some(&4));
}

// Tests for flip_vertical

#[test]
fn flip_vertical_2x2() {
    let mut grid = Grid2D::from_single_vec(2, 2, vec![1, 2, 3, 4]);
    // Original: [[1, 2], [3, 4]]
    assert_eq!(grid.get(&coord(0, 0)), Some(&1));
    assert_eq!(grid.get(&coord(1, 0)), Some(&2));
    assert_eq!(grid.get(&coord(0, 1)), Some(&3));
    assert_eq!(grid.get(&coord(1, 1)), Some(&4));
    // Flipped:  [[3, 4], [1, 2]]
    grid.flip_vertical();
    assert_eq!(grid.get_sizes(), (2, 2));
    assert_eq!(grid.get(&coord(0, 0)), Some(&3));
    assert_eq!(grid.get(&coord(1, 0)), Some(&4));
    assert_eq!(grid.get(&coord(0, 1)), Some(&1));
    assert_eq!(grid.get(&coord(1, 1)), Some(&2));
}

#[test]
fn flip_vertical_3x2() {
    // Original: [[1, 2, 3], [4, 5, 6]]
    let mut grid = Grid2D::from_single_vec(3, 2, vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(grid.get(&coord(0, 0)), Some(&1));
    assert_eq!(grid.get(&coord(1, 0)), Some(&2));
    assert_eq!(grid.get(&coord(2, 0)), Some(&3));
    assert_eq!(grid.get(&coord(0, 1)), Some(&4));
    assert_eq!(grid.get(&coord(1, 1)), Some(&5));
    assert_eq!(grid.get(&coord(2, 1)), Some(&6));
    // Flipped: [[4, 5, 6], [1, 2, 3]]
    grid.flip_vertical();
    assert_eq!(grid.get_sizes(), (3, 2));
    assert_eq!(grid.get(&coord(0, 0)), Some(&4));
    assert_eq!(grid.get(&coord(1, 0)), Some(&5));
    assert_eq!(grid.get(&coord(2, 0)), Some(&6));
    assert_eq!(grid.get(&coord(0, 1)), Some(&1));
    assert_eq!(grid.get(&coord(1, 1)), Some(&2));
    assert_eq!(grid.get(&coord(2, 1)), Some(&3));
}

// Tests for position_status

#[test]
fn position_status_inside_center() {
    let grid = Grid2D::from_single_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(grid.position_status(&coord(1, 1)), PositionStatus::Inside);
}

#[test]
fn position_status_on_border_0_1() {
    let grid = Grid2D::from_single_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(grid.position_status(&coord(0, 1)), PositionStatus::OnBorder);
}

#[test]
fn position_status_on_border_1_0() {
    let grid = Grid2D::from_single_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(grid.position_status(&coord(1, 0)), PositionStatus::OnBorder);
}

#[test]
fn position_status_on_border_2_2() {
    let grid = Grid2D::from_single_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(grid.position_status(&coord(2, 2)), PositionStatus::OnBorder);
}

#[test]
fn position_status_outside_3_0() {
    let grid = Grid2D::from_single_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(grid.position_status(&coord(3, 0)), PositionStatus::Outside);
}

#[test]
fn position_status_outside_0_3() {
    let grid = Grid2D::from_single_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(grid.position_status(&coord(0, 3)), PositionStatus::Outside);
}

// Tests for is_outside

#[test]
fn is_outside_inside() {
    let grid = Grid2D::from_single_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert!(!grid.is_outside(&coord(0, 0)));
    assert!(!grid.is_outside(&coord(1, 1)));
    assert!(!grid.is_outside(&coord(2, 2)));
}

#[test]
fn is_outside_outside() {
    let grid = Grid2D::from_single_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert!(grid.is_outside(&coord(3, 0)));
    assert!(grid.is_outside(&coord(0, 3)));
}

// Tests for contains

#[test]
fn contains_true() {
    let grid = Grid2D::from_single_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert!(grid.contains(&coord(0, 0)));
    assert!(grid.contains(&coord(1, 1)));
    assert!(grid.contains(&coord(2, 2)));
}

#[test]
fn contains_false() {
    let grid = Grid2D::from_single_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert!(!grid.contains(&coord(3, 0)));
    assert!(!grid.contains(&coord(0, 3)));
}

// Tests for get_neighbors

#[test]
fn get_neighbors_center() {
    let grid = Grid2D::from_single_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let neighbors = grid.get_neighbors(&coord(1, 1));
    let expected: HashSet<_> = [coord(1, 2), coord(1, 0), coord(2, 1), coord(0, 1)]
        .into_iter()
        .collect();
    assert_eq!(expected.len(), 4);
    assert_eq!(neighbors, expected);
}

#[test]
fn get_neighbors_corner() {
    let grid = Grid2D::from_single_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let neighbors = grid.get_neighbors(&coord(0, 0));
    let expected: HashSet<_> = [coord(0, 1), coord(1, 0)].into_iter().collect();
    assert_eq!(expected.len(), 2);
    assert_eq!(neighbors, expected);
}

#[test]
fn get_neighbors_bottom_edge() {
    let grid = Grid2D::from_single_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let neighbors = grid.get_neighbors(&coord(1, 0));
    let expected: HashSet<_> = [coord(1, 1), coord(2, 0), coord(0, 0)]
        .into_iter()
        .collect();
    assert_eq!(expected.len(), 3);
    assert_eq!(neighbors, expected);
}

#[test]
fn get_neighbors_side_edge() {
    let grid = Grid2D::from_single_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let neighbors = grid.get_neighbors(&coord(0, 1));
    let expected: HashSet<_> = [coord(0, 2), coord(0, 0), coord(1, 1)]
        .into_iter()
        .collect();
    assert_eq!(neighbors, expected);
}

// Tests for set

#[test]
fn set_value() {
    let mut grid: Grid2D<i32> = Grid2D::from_double_vec(vec![vec![1, 2, 3], vec![4, 5, 6]]);
    {
        assert!(grid.set(&coord(0, 0), &10));
        assert_eq!(grid.get(&coord(0, 0)), Some(&10));
        assert_eq!(grid[&coord(0, 0)], 10);
    }
    {
        grid[&coord(1, 0)] = 20;
        assert_eq!(grid.get(&coord(1, 0)), Some(&20));
        assert_eq!(grid[&coord(1, 0)], 20);
    }
}

#[test]
fn set_out_of_bounds() {
    let mut grid: Grid2D<i32> = Grid2D::from_double_vec(vec![vec![1, 2, 3], vec![4, 5, 6]]);
    assert!(!grid.set(&coord(17, 3), &10));
}

// Tests for iter_all

#[test]
fn iter_all() {
    let grid: Grid2D<i32> = Grid2D::from_double_vec(vec![vec![1, 2, 3], vec![4, 5, 6]]);
    let elements: Vec<(Coordinate, i32)> =
        grid.iter_all().map(|(coord, &val)| (coord, val)).collect();
    assert_eq!(elements.len(), 6);
    assert!(elements.contains(&(coord(0, 0), 1)));
    assert!(elements.contains(&(coord(1, 0), 2)));
    assert!(elements.contains(&(coord(2, 0), 3)));
    assert!(elements.contains(&(coord(0, 1), 4)));
    assert!(elements.contains(&(coord(1, 1), 5)));
    assert!(elements.contains(&(coord(2, 1), 6)));
}

// Tests for get_row

#[test]
fn get_row() {
    let grid: Grid2D<i32> = Grid2D::from_double_vec(vec![vec![1, 2, 3], vec![4, 5, 6]]);
    let row_0: Vec<&i32> = grid.get_row(0).collect();
    assert_eq!(row_0, vec![&1, &2, &3]);
}

#[test]
fn get_row_mut() {
    let mut grid: Grid2D<i32> = Grid2D::from_double_vec(vec![vec![1, 2, 3], vec![4, 5, 6]]);
    for val in grid.get_row_mut(0) {
        *val += 10;
    }
    assert_eq!(grid.get(&coord(0, 0)), Some(&11));
    assert_eq!(grid.get(&coord(1, 0)), Some(&12));
    assert_eq!(grid.get(&coord(2, 0)), Some(&13));
}

// Tests for get_row

#[test]
fn get_column() {
    let grid: Grid2D<i32> = Grid2D::from_double_vec(vec![vec![1, 2, 3], vec![4, 5, 6]]);
    let row_0: Vec<&i32> = grid.get_column(1).collect();
    assert_eq!(row_0, vec![&2, &5]);
}

#[test]
fn get_column_mut() {
    let mut grid: Grid2D<i32> = Grid2D::from_double_vec(vec![vec![1, 2, 3], vec![4, 5, 6]]);
    for val in grid.get_column_mut(1) {
        *val += 10;
    }
    assert_eq!(grid.get(&coord(1, 0)), Some(&12));
    assert_eq!(grid.get(&coord(1, 1)), Some(&15));
}
