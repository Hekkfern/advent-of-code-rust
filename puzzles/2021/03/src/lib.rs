use aoc_intervals::interval::Interval;
use bitvec::prelude::*;

// -----------------------------------------------------------
// ------------------------ Common ---------------------------
// -----------------------------------------------------------

// -----------------------------------------------------------
// ------------------------ Part 1 ---------------------------
// -----------------------------------------------------------

/// Parameters for solving Part 1 of the puzzle.
pub struct Part1Parameters {
    pub input_data: &'static str,
    pub binary_length: usize,
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
    // extract frequencies from input data
    let mut frequencies: Vec<i32> = vec![0; params.binary_length];
    params.input_data.trim().lines().for_each(|line| {
        line.trim().chars().enumerate().for_each(|(i, c)| {
            if c == '1' {
                frequencies[i] += 1;
            } else {
                frequencies[i] -= 1;
            }
        })
    });
    // calculate gamma and epsilon rates
    let mut gamma = bitvec![u16, Msb0; 0; params.binary_length];
    for (i, freq) in frequencies.iter().enumerate() {
        if *freq >= 0 {
            gamma.set(i, true);
        }
    }
    let epsilon = !gamma.clone();
    // calculate power consumption
    let result = gamma.load::<u32>() * epsilon.load::<u32>();
    result.to_string()
}

// -----------------------------------------------------------
// ------------------------ Part 2 ---------------------------
// -----------------------------------------------------------

/// Parameters for solving Part 2 of the puzzle.
pub struct Part2Parameters {
    pub input_data: &'static str,
    pub binary_length: usize,
}

enum Rating {
    Oxygen,
    CO2,
}

fn filter(
    codes: &Vec<BitVec<u16, Msb0>>,
    rating: Rating,
    binary_length: usize,
) -> BitVec<u16, Msb0> {
    let mut indices: Interval<i32> = Interval::from_boundaries(0, codes.len() as i32 - 1);
    for i in 0..binary_length {
        let first_one_idx = codes
            .iter()
            .enumerate()
            .filter(|(idx, _)| indices.contains(*idx as i32))
            .find(|(_, code)| code[i])
            .unwrap()
            .0 as i32;
        let mut bit_criteria = first_one_idx <= indices.get_min() + (indices.count() as i32 / 2);
        if matches!(rating, Rating::CO2) {
            // invert for CO2
            bit_criteria = !bit_criteria;
        }
        match bit_criteria {
            true => {
                indices = Interval::from_boundaries(first_one_idx, indices.get_max());
            }
            false => {
                indices = Interval::from_boundaries(indices.get_min(), first_one_idx - 1);
            }
        }
        if indices.has_one_value() {
            break;
        }
    }
    codes[indices.get_min() as usize].clone()
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
    // parse input data

    let mut codes: Vec<_> = params
        .input_data
        .trim()
        .lines()
        .map(|line| {
            let mut bv = bitvec![u16, Msb0; 0; params.binary_length];
            line.trim().chars().enumerate().for_each(|(i, c)| {
                if c == '1' {
                    bv.set(i, true);
                }
            });
            bv
        })
        .collect();
    // sort codes
    codes.sort_unstable();
    // determine ratings codes
    let oxygen_code = filter(&codes, Rating::Oxygen, params.binary_length).load::<u32>();
    let co2_code = filter(&codes, Rating::CO2, params.binary_length).load::<u32>();
    // calculate life support rating
    let result = oxygen_code * co2_code;
    result.to_string()
}
