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
    let mut sum: u32 = 0;
    let digits: Vec<u32> = params
        .input_data
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    for (i, &digit) in digits.iter().enumerate() {
        let next_index = (i + 1) % digits.len();
        if digit == digits[next_index] {
            sum += digit;
        }
    }
    sum.to_string()
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
    let mut sum: u32 = 0;
    let digits: Vec<u32> = params
        .input_data
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    for (i, &digit) in digits.iter().enumerate() {
        let next_index = (i + (digits.len() / 2)) % digits.len();
        if digit == digits[next_index] {
            sum += digit;
        }
    }
    sum.to_string()
}
