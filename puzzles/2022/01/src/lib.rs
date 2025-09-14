use aoc_utils::string_utils::convert_to_groups_of_numbers;

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
    let groups = convert_to_groups_of_numbers::<u32>(params.input_data);
    let max_calories = groups
        .iter()
        .map(|group| group.iter().sum::<u32>())
        .max()
        .unwrap();
    max_calories.to_string()
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
    let groups = convert_to_groups_of_numbers::<u32>(params.input_data);
    let mut group_sums: Vec<u32> = groups
        .iter()
        .map(|group| group.iter().sum::<u32>())
        .collect();
    group_sums.sort_unstable_by(|a, b| b.cmp(a));
    let top_3_sum: u32 = group_sums.iter().take(3).sum();
    top_3_sum.to_string()
}
