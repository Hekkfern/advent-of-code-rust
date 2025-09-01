mod rocks;

use aoc_geometry::grid_2d::Grid2D;
use rocks::{GridCell, Rocks};

fn parse_input(input: &str) -> Rocks {
    let data = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().map(|s| GridCell::from(s)).collect())
        .collect();
    let mut grid = Grid2D::from_double_vec(data);
    grid.flip_vertical();
    Rocks::new(grid)
}

/// Parameters for solving Part 1 of the puzzle.
pub struct Part1Parameters {
    pub input_data: &'static str,
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
    let mut rocks = parse_input(params.input_data);
    rocks.shift_north();
    rocks.calculate_load().to_string()
}

/// Parameters for solving Part 2 of the puzzle.
pub struct Part2Parameters {
    pub input_data: &'static str,
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
    // TODO
    String::from("")
}
