use aoc_utils::string_utils::convert_to_list_of_numbers;

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
    // TODO: Use Iterator::map_windows (https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map_windows) when it becomes stable to avoid the allocation of a new vector.
    let input = convert_to_list_of_numbers::<u32>(params.input_data);
    let sum: u32 = input
        .windows(2)
        .map(|w| if w[0] < w[1] { 1 } else { 0 })
        .sum();
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
    let input = convert_to_list_of_numbers::<u32>(params.input_data);
    let windows1 = input.windows(3);
    let windows2 = input.windows(3).skip(1);
    let sum: u32 = windows1
        .zip(windows2)
        .map(|(w1, w2)| {
            let s1: u32 = w1.iter().sum();
            let s2: u32 = w2.iter().sum();
            if s1 < s2 { 1 } else { 0 }
        })
        .sum();
    sum.to_string()
}
