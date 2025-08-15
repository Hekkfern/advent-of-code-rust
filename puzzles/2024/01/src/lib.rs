use rayon::prelude::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

/// Parameters for solving Part 1 of the puzzle.
pub struct Part1Parameters {
    pub input_data: &'static str,
}

fn parse_input_for_part1(input: &str) -> (BinaryHeap<Reverse<u32>>, BinaryHeap<Reverse<u32>>) {
    let mut list1 = BinaryHeap::new();
    let mut list2 = BinaryHeap::new();

    input.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        if let Some(num1) = parts.next() {
            if let Ok(n1) = num1.parse::<u32>() {
                list1.push(Reverse(n1));
            }
        }
        if let Some(num2) = parts.next() {
            if let Ok(n2) = num2.parse::<u32>() {
                list2.push(Reverse(n2));
            }
        }
    });

    (list1, list2)
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
    let (mut list1, mut list2) = parse_input_for_part1(params.input_data);
    let mut sum: u32 = 0;
    while let (Some(Reverse(a)), Some(Reverse(b))) = (list1.pop(), list2.pop()) {
        sum += (a as i64 - b as i64).abs() as u32;
    }
    sum.to_string()
}

/// Parameters for solving Part 2 of the puzzle.
pub struct Part2Parameters {
    pub input_data: &'static str,
}

fn parse_input_for_part2(input: &str) -> (Vec<u32>, HashMap<u32, u32>) {
    let mut list1 = Vec::new();
    let mut list2_freqs = HashMap::new();

    input.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        if let Some(num1) = parts.next() {
            if let Ok(n1) = num1.parse::<u32>() {
                list1.push(n1);
            }
        }
        if let Some(num2) = parts.next() {
            if let Ok(n2) = num2.parse::<u32>() {
                list2_freqs.entry(n2).and_modify(|c| *c += 1).or_insert(1);
            }
        }
    });

    (list1, list2_freqs)
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
    let (list1, list2_freqs) = parse_input_for_part2(params.input_data);
    let sum = list1
        .par_iter()
        .map(|&num| {
            list2_freqs
                .get(&num)
                .map_or(0, |&freq| freq as u64 * num as u64)
        })
        .sum::<u64>();
    sum.to_string()
}
