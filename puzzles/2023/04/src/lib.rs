use crate::card::CardId;
use std::collections::HashMap;

mod card;

fn parse_input_line(line: &str) -> card::Card {
    let (card_id_part, numbers_part) = line.trim().split_once(':').unwrap();
    let id: u8 = card_id_part
        .trim()
        .strip_prefix("Card")
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    let (winning_str, candidate_str) = numbers_part.trim().split_once('|').unwrap();
    let winning_numbers = winning_str
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let candidate_numbers = candidate_str
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    card::Card::new(id, winning_numbers, candidate_numbers)
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
    let accum: u32 = params
        .input_data
        .trim()
        .lines()
        .map(|line| parse_input_line(line.trim()))
        .map(|card| card.calculate_points())
        .sum::<u32>();
    accum.to_string()
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
    let mut copies = HashMap::<CardId, u32>::new();
    let cards: Vec<_> = params
        .input_data
        .trim()
        .lines()
        .map(|line| parse_input_line(line.trim()))
        .collect();
    for card in &cards {
        copies
            .entry(card.card_id())
            .and_modify(|count| *count += 1)
            .or_insert(1);
        let num_matching_numbers = card.calculate_matching_numbers() as u8;
        for i in 1..=num_matching_numbers {
            let copies_to_add = copies[&card.card_id()];
            copies
                .entry(&card.card_id() + i)
                .and_modify(|count| *count += copies_to_add)
                .or_insert(copies_to_add);
        }
    }
    copies.values().sum::<u32>().to_string()
}
