mod constrained_sand_map;
mod infinite_sand_map;

use aoc_geometry::Point;

// -----------------------------------------------------------
// ------------------------ Common ---------------------------
// -----------------------------------------------------------

fn parse_input(input: &str) -> Vec<Vec<Point<i32, 2>>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|p| {
                    let coords: Vec<i32> = p.split(',').map(|c| c.parse().unwrap()).collect();
                    Point::new([coords[0], coords[1]])
                })
                .collect()
        })
        .collect()
}

// -----------------------------------------------------------
// ------------------------ Part 1 ---------------------------
// -----------------------------------------------------------

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
    let rock_vertices = parse_input(params.input_data);
    let mut sand_map = constrained_sand_map::ConstrainedSandMap::new(&rock_vertices);
    while sand_map.drop_sand() {}
    sand_map.get_number_of_sand_units().to_string()
}

// -----------------------------------------------------------
// ------------------------ Part 2 ---------------------------
// -----------------------------------------------------------

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
    let rock_vertices = parse_input(params.input_data);
    let mut sand_map = infinite_sand_map::InfiniteSandMap::new(&rock_vertices);
    while sand_map.drop_sand() {}
    sand_map.get_number_of_sand_units().to_string()
}
