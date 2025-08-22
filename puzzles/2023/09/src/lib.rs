mod history;

use crate::history::History;

fn parse_input_line(line: &str) -> History {
    let numbers: Vec<i64> = line
        .split_whitespace()
        .map(|s| s.trim().parse().unwrap())
        .collect();
    History::new(numbers)
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
    let value: i64 = params
        .input_data
        .trim()
        .lines()
        .map(|line| parse_input_line(line.trim()))
        .map(|h| h.extrapolate_right())
        .sum();
    value.to_string()
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
    let value: i64 = params
        .input_data
        .trim()
        .lines()
        .map(|line| parse_input_line(line.trim()))
        .map(|h| h.extrapolate_left())
        .sum();
    value.to_string()
}
