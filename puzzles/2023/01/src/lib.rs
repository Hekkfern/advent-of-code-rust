/// Parameters for solving Part 1 of the puzzle.
pub struct Part1Parameters {
    pub input_data: &'static str,
}

/// Parses a line to extract the first and last non-zero digit and combines them into a two-digit number.
///
/// # Arguments
///
/// * `line` - A string slice representing a line from the input.
///
/// # Returns
///
/// A two-digit number formed by the first and last non-zero digit in the line.
fn parse_simple_line(line: &str) -> u32 {
    let digits: Vec<_> = line.chars().filter(|c| c.is_ascii_digit()).collect();
    match (digits.first(), digits.last()) {
        (Some(&first), Some(&last)) => {
            (first.to_digit(10).unwrap() * 10) + last.to_digit(10).unwrap()
        }
        _ => 0,
    }
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
    let value: u32 = params
        .input_data
        .trim()
        .lines()
        .map(|line| parse_simple_line(line.trim()))
        .sum();
    value.to_string()
}

/// Parameters for solving Part 2 of the puzzle.
pub struct Part2Parameters {
    pub input_data: &'static str,
}

/// Mapping of spelled-out numbers to their digit values.
const NUMBER_TEXT_MAP: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

/// Analyzes a substring to determine if it starts with a digit or a spelled-out number.
///
/// # Arguments
///
/// * `substr` - A string slice to analyze.
///
/// # Returns
///
/// An option containing the digit value if found.
fn analyze_item_in_weird_line(substr: &str) -> Option<u32> {
    let first_char = substr.chars().next()?;
    if first_char.is_ascii_digit() && first_char != '0' {
        return first_char.to_digit(10);
    }
    NUMBER_TEXT_MAP.iter().find_map(|&(word, value)| {
        if substr.starts_with(word) {
            Some(value)
        } else {
            None
        }
    })
}

/// Parses a line to extract the first and last digit or spelled-out number and combines them into a two-digit number.
///
/// # Arguments
///
/// * `line` - A string slice representing a line from the input.
///
/// # Returns
///
/// A two-digit number formed by the first and last valid digit or spelled-out number in the line.
fn parse_weird_line(line: &str) -> u32 {
    let mut first = None;
    let mut last = None;
    for i in 0..line.len() {
        let substr = &line[i..];
        if let Some(value) = analyze_item_in_weird_line(substr) {
            if first.is_none() {
                first = Some(value);
            }
            last = Some(value);
        }
    }
    match (first, last) {
        (Some(f), Some(l)) => f * 10 + l,
        _ => 0,
    }
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
    let value: u32 = params
        .input_data
        .trim()
        .lines()
        .map(|line| parse_weird_line(line.trim()))
        .sum();
    value.to_string()
}
