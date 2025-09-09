use aoc_geometry::{Direction2D, Grid2D, GridCoordinate2D};
use aoc_utils::string_utils::convert_to_matrix_of_digits;

type Height = u8;
type TreeHeightMatrix = Grid2D<Height>;
type TreeVisibilityMatrix = Grid2D<bool>;

fn parse_input(input: &str) -> TreeHeightMatrix {
    let data = convert_to_matrix_of_digits(input);
    TreeHeightMatrix::from_double_vec(data)
}

/// Parameters for solving Part 1 of the puzzle.
pub struct Part1Parameters {
    pub input_data: &'static str,
}

const STARTING_HEIGHT: i32 = -1;

fn analyze_column_visibility<'a>(
    visibility: &mut TreeVisibilityMatrix,
    col_idx: usize,
    column: &mut impl DoubleEndedIterator<Item = (usize, &'a Height)>,
) -> u32 {
    let mut count = 0;
    let mut max_height = STARTING_HEIGHT;
    for (row_idx, &h) in column {
        let h = h as i32;
        if h > max_height {
            max_height = h;
            let coord = GridCoordinate2D::new([col_idx, row_idx]);
            if !visibility.get(&coord).unwrap() {
                visibility.set(&coord, &true);
                count += 1;
            }
        }
    }
    count
}

fn check_tree_visibility_vertically(
    forest: &TreeHeightMatrix,
    visibility: &mut TreeVisibilityMatrix,
) -> u32 {
    let mut count = 0;
    for col_idx in 1..forest.get_width() - 1 {
        count += analyze_column_visibility(visibility, col_idx, &mut forest.get_column(col_idx));
        count +=
            analyze_column_visibility(visibility, col_idx, &mut forest.get_column(col_idx).rev());
    }
    count
}

fn analyze_row_visibility<'a>(
    visibility: &mut TreeVisibilityMatrix,
    row_idx: usize,
    row: &mut impl DoubleEndedIterator<Item = (usize, &'a Height)>,
) -> u32 {
    let mut count = 0;
    let mut max_height = STARTING_HEIGHT;
    for (col_idx, &h) in row {
        let h = h as i32;
        if h > max_height {
            max_height = h;
            let coord = GridCoordinate2D::new([col_idx, row_idx]);
            if !visibility.get(&coord).unwrap() {
                visibility.set(&coord, &true);
                count += 1;
            }
        }
    }
    count
}

fn check_tree_visibility_horizontally(
    forest: &TreeHeightMatrix,
    visibility: &mut TreeVisibilityMatrix,
) -> u32 {
    let mut count = 0;
    for row_idx in 1..forest.get_height() - 1 {
        count += analyze_row_visibility(visibility, row_idx, &mut forest.get_row(row_idx));
        count += analyze_row_visibility(visibility, row_idx, &mut forest.get_row(row_idx).rev());
    }
    count
}

///Checks all the trees in the forest and counts how many trees are visible from outside the forest.
///
/// # Arguments
/// * `forest` - The 2D matrix of tree heights.
///
/// # Returns
/// Number of trees visible from outside the forest.
fn check_tree_visibility(forest: &TreeHeightMatrix) -> u32 {
    // initialize grid of visibility (true if it is visible, false otherwise)
    let mut visibility =
        TreeVisibilityMatrix::from_default_value(forest.get_width(), forest.get_height(), &false);
    // check visibilities. add 4 as the 4 corners are always visible
    check_tree_visibility_vertically(&forest, &mut visibility)
        + check_tree_visibility_horizontally(&forest, &mut visibility)
        + 4
}

/// Solves Part 1 of the puzzle
///
/// # Arguments
///
/// * `params` - Parameters for the solver
///
/// # Returns
///
/// The solution as a string
pub fn solve_part1(params: Part1Parameters) -> String {
    let forest = parse_input(params.input_data);
    check_tree_visibility(&forest).to_string()
}

/// Parameters for solving Part 2 of the puzzle.
pub struct Part2Parameters {
    pub input_data: &'static str,
}

fn calculate_viewing_distance(
    forest: &TreeHeightMatrix,
    coord: &GridCoordinate2D,
    direction: Direction2D,
) -> u32 {
    let height = *forest.get(coord).unwrap();
    let mut distance = 0;
    let mut current_coord = *coord;
    loop {
        if let Some(next_coord) = forest.try_move(&current_coord, &direction.to_vector()) {
            distance += 1;
            if *forest.get(&next_coord).unwrap() >= height {
                break;
            }
            current_coord = next_coord;
        } else {
            break;
        }
    }
    distance
}

/// Calculates the scenic score of the selected tree.
///
/// # Arguments
/// * `forest` - The 2D matrix of tree heights.
/// * `coord` - The coordinate of the tree to calculate the scenic score for.
///
/// # Returns
/// The scenic score of the selected tree.
fn calculate_scenic_score(forest: &TreeHeightMatrix, coord: &GridCoordinate2D) -> u32 {
    calculate_viewing_distance(forest, coord, Direction2D::Left)
        * calculate_viewing_distance(forest, coord, Direction2D::Right)
        * calculate_viewing_distance(forest, coord, Direction2D::Up)
        * calculate_viewing_distance(forest, coord, Direction2D::Down)
}

fn calculate_maximum_scenic_score(forest: &TreeHeightMatrix) -> u32 {
    forest
        .iter_all()
        .map(|(coord, _)| calculate_scenic_score(forest, &coord))
        .max()
        .unwrap_or(0)
}

/// Solves Part 2 of the puzzle
///
/// # Arguments
///
/// * `params` - Parameters for the solver
///
/// # Returns
///
/// The solution as a string
pub fn solve_part2(params: Part2Parameters) -> String {
    let forest = parse_input(params.input_data);
    calculate_maximum_scenic_score(&forest).to_string()
}
