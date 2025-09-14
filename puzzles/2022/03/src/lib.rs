use std::collections::HashSet;

/// Parameters for solving Part 1 of the puzzle.
pub struct Part1Parameters {
    pub input_data: &'static str,
}

fn calculate_priority(item: char) -> u32 {
    const UPPERCASE_FIRST_PRIORITY_VALUE: u32 = 27;
    const LOWERCASE_FIRST_PRIORITY_VALUE: u32 = 1;
    match item {
        'a'..='z' => (item as u32) - ('a' as u32) + LOWERCASE_FIRST_PRIORITY_VALUE,
        'A'..='Z' => (item as u32) - ('A' as u32) + UPPERCASE_FIRST_PRIORITY_VALUE,
        _ => unreachable!(),
    }
}

fn search_common_item_in_rucksack(compartment1: &str, compartment2: &str) -> char {
    let item_types_1: HashSet<char> = compartment1.chars().collect();
    /* look for the repeated item in both compartments */
    compartment2
        .chars()
        .find(|c| item_types_1.contains(c))
        .unwrap()
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
    let total_priority: u32 = params
        .input_data
        .trim()
        .lines()
        .map(|line| {
            let (compartment1, compartment2) = line.split_at(line.len() / 2);
            let common_item = search_common_item_in_rucksack(compartment1, compartment2);
            calculate_priority(common_item)
        })
        .sum();
    total_priority.to_string()
}

/// Parameters for solving Part 2 of the puzzle.
pub struct Part2Parameters {
    pub input_data: &'static str,
}

fn search_common_item_in_group(rucksack1: &str, rucksack2: &str, rucksack3: &str) -> char {
    let item_types_1: HashSet<char> = rucksack1.chars().collect();
    let item_types_2: HashSet<char> = rucksack2.chars().collect();
    /* look for the repeated item in all the compartments */
    rucksack3
        .chars()
        .find(|c| item_types_1.contains(c) && item_types_2.contains(c))
        .unwrap()
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
    let total_priority: u32 = params
        .input_data
        .trim()
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| {
            let common_item = search_common_item_in_group(group[0], group[1], group[2]);
            calculate_priority(common_item)
        })
        .sum();
    total_priority.to_string()
}
