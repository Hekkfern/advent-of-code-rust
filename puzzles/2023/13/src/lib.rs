mod pattern;

use pattern::Pattern;

const ROCK_CHARACTER: char = '#';

fn parse_pattern(block: &str) -> Pattern {
    let mut rows: Vec<u64> = Vec::new();
    let mut cols: Vec<u64> = Vec::new();
    for line in block.lines() {
        let mut row_value: u64 = 0;
        for (col_idx, c) in line.trim().chars().enumerate() {
            row_value = (row_value << 1) | if c == ROCK_CHARACTER { 1 } else { 0 };
            if cols.len() == col_idx {
                cols.push(if c == ROCK_CHARACTER { 1 } else { 0 });
            } else {
                cols[col_idx] = (cols[col_idx] << 1) | if c == ROCK_CHARACTER { 1 } else { 0 };
            }
        }
        rows.push(row_value);
    }
    Pattern::new(rows, cols)
}

fn parse_input(input: &str) -> Vec<Pattern> {
    input
        .trim()
        .split("\n\n")
        .map(|block| parse_pattern(block))
        .collect()
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
    let patterns = parse_input(params.input_data);
    let result: u64 = patterns
        .iter()
        .map(|p| {
            let mut value: u64 = 0;
            if let Some((idx1, _)) = p.search_vertical_reflection_line() {
                value += (idx1 + 1) as u64;
            }
            if let Some((idx1, _)) = p.search_horizontal_reflection_line() {
                value += 100_u64 * (idx1 + 1) as u64;
            }
            value
        })
        .sum();
    result.to_string()
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
    let patterns = parse_input(params.input_data);
    let result: u64 = patterns
        .iter()
        .map(|p| {
            let mut value: u64 = 0;
            if let Some((idx1, _)) = p.search_vertical_reflection_line_with_single_fix() {
                value += (idx1 + 1) as u64;
            }
            if let Some((idx1, _)) = p.search_horizontal_reflection_line_with_single_fix() {
                value += 100_u64 * (idx1 + 1) as u64;
            }
            value
        })
        .sum();
    result.to_string()
}
