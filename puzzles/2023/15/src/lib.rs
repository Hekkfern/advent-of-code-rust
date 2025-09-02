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
        let (label, op, arg) = {
            let mut chars = s.chars();
            let pos = s.find(|c| c == '=' || c == '-').unwrap();
            let label = &s[..pos];
            let op = chars.nth(pos).unwrap();
            let arg = s.get(pos + 1..);
            (label, op, arg)
        };
        let lens_box = &mut book[calculate_hash(label) as usize];
        match op {
            '=' => {
                let focal_length = arg.unwrap().parse::<u8>().unwrap();
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
