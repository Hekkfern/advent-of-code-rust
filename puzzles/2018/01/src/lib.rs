use std::collections::HashSet;

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
    let freq: i64 = params
        .input_data
        .trim()
        .lines()
        .map(|line| line.trim().parse::<i64>().unwrap())
        .sum();
    freq.to_string()
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
    let mut current_freq: i64 = 0;
    let mut freqs_seen = HashSet::<i64>::new();
    freqs_seen.insert(current_freq);
    for value in params.input_data.trim().lines().cycle() {
        let change: i64 = value.trim().parse().unwrap();
        current_freq += change;
        if freqs_seen.contains(&current_freq) {
            break;
        }
        freqs_seen.insert(current_freq);
    }
    current_freq.to_string()
}
