mod card_type;
mod hand;
mod hand_type;

use crate::card_type::CardType;
use crate::hand::Hand;

fn parse_input(input: &str, with_jokers: bool) -> Vec<Hand> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let cards: Vec<_> = parts
                .next()
                .unwrap()
                .chars()
                .map(|s| CardType::from(s))
                .collect();
            let bid = parts.next().unwrap().parse::<u32>().unwrap();
            Hand::new(cards.try_into().unwrap(), bid, with_jokers)
        })
        .collect()
}

fn calculate_winnings(hands: &Vec<Hand>, with_jokers: bool) -> u64 {
    // Sort the hands in ascending order (weakest hand first)
    let mut sorted_hands = hands.clone();
    sorted_hands.sort_by(|a, b| a.cmp(b, with_jokers));
    // Calculate the total winnings
    sorted_hands
        .iter()
        .enumerate()
        .map(|(i, hand)| -> u64 { (i + 1) as u64 * hand.bid() as u64 })
        .sum()
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
    let hands = parse_input(params.input_data, false);
    let winnings = calculate_winnings(&hands, false);
    winnings.to_string()
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
    let hands = parse_input(params.input_data, true);
    let winnings = calculate_winnings(&hands, true);
    winnings.to_string()
}
