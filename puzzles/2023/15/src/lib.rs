mod hasher;
mod lens;

use crate::lens::Lens;
use hasher::calculate_hash;

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
    params
        .input_data
        .trim()
        .split(',')
        .fold(0, |acc, s| -> u64 { acc + calculate_hash(s) })
        .to_string()
}

/// Parameters for solving Part 2 of the puzzle.
pub struct Part2Parameters {
    pub input_data: &'static str,
}

type LensBox = Vec<Lens>;
type LensBook = Vec<LensBox>;

const NUM_BOXES: usize = 256;

fn calculate_focusing_power(book: &LensBook) -> u64 {
    book.iter()
        .enumerate()
        .map(|(i, lens_box)| -> u64 {
            lens_box
                .iter()
                .enumerate()
                .map(|(j, lens)| -> u64 {
                    (i as u64 + 1) * (j as u64 + 1) * lens.get_focal_length() as u64
                })
                .sum()
        })
        .sum()
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
    // create book
    let mut book: LensBook = Vec::with_capacity(NUM_BOXES);
    book.resize_with(NUM_BOXES, Vec::new);
    // fill book based on input data
    params.input_data.trim().split(',').for_each(|s| {
        let symbol_position = s.find(|c| "=-".contains(c)).unwrap();
        let label = &s[..symbol_position];
        let lens_box = book.get_mut(calculate_hash(label) as usize).unwrap();
        match s.chars().nth(symbol_position).unwrap() {
            '=' => {
                let focal_length = s
                    .chars()
                    .nth(symbol_position + 1)
                    .unwrap()
                    .to_digit(10)
                    .unwrap() as u8;
                match lens_box.iter_mut().find(|lens| lens.get_label() == label) {
                    Some(lens) => lens.set_focal_length(focal_length),
                    None => lens_box.push(Lens::new(label, focal_length)),
                }
            }
            '-' => lens_box.retain(|lens| lens.get_label() != label),
            _ => unreachable!(),
        }
    });
    // calculate result
    calculate_focusing_power(&book).to_string()
}
