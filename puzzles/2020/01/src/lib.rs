use std::collections::HashSet;

const TARGET_SUM: u16 = 2020;

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
    let mut encountered = HashSet::<u16>::new();
    for value in params
        .input_data
        .trim()
        .lines()
        .map(|line| line.trim().parse::<u16>().unwrap())
    {
        let other_value = TARGET_SUM - value;
        if encountered.contains(&other_value) {
            return (value as u32 * other_value as u32).to_string();
        }
        encountered.insert(value);
    }
    unreachable!();
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
    let numbers: HashSet<u16> = params
        .input_data
        .trim()
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();
    for &n1 in &numbers {
        for &n2 in &numbers {
            if n1 == n2 {
                continue;
            }
            if let Some(n3) = TARGET_SUM.checked_sub(n1 + n2) {
                if numbers.contains(&n3) {
                    return (n1 as u64 * n2 as u64 * n3 as u64).to_string();
                }
            }
        }
    }
    unreachable!();
}
