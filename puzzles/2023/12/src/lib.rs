mod record;

use record::Record;

fn parse_input_line(line: &str) -> Record {
    let mut parts = line.split_whitespace();
    let springs = parts.next().unwrap().to_string();
    let contiguous_group_info = parts
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    Record::new(springs, contiguous_group_info)
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
    let value: u64 = params
        .input_data
        .trim()
        .lines()
        .map(|line| {
            let record = parse_input_line(line.trim());
            record.solve_original()
        })
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
    let value: u64 = params
        .input_data
        .trim()
        .lines()
        .map(|line| {
            let record = parse_input_line(line.trim());
            record.solve_unfolded()
        })
        .sum();
    value.to_string()
}
