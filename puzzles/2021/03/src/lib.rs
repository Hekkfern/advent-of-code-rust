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
    frequencies: &Vec<i32>,
    rating: Rating,
) -> BitVec<u16, Msb0> {
    let mut indices: Interval<i32> = Interval::from_boundaries(0, codes.len() as i32 - 1);
    loop {
        for i in 0..frequencies.len() {
            let bit_criteria = match rating {
                Rating::Oxygen => frequencies[i] >= 0,
                Rating::CO2 => frequencies[i] < 0,
            };
            let first_one_idx = codes
                .iter()
                .enumerate()
                .filter(|(idx, _)| indices.contains(*idx as i32))
                .find(|(_, code)| code[i])
                .unwrap()
                .0;
            match bit_criteria {
                true => {
                    indices = Interval::from_boundaries(first_one_idx as i32, indices.get_max());
                }
                false => {
                    indices = Interval::from_boundaries(indices.get_min(), first_one_idx as i32 - 1);
                }
            }
            if indices.has_one_value() {
                return codes[indices.get_min() as usize].clone();
            }
        }
    }
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
    let mut frequencies: Vec<i32> = vec![0; params.binary_length];
    let mut codes: Vec<BitVec<u16, Msb0>> = Vec::new();
    params.input_data.trim().lines().for_each(|line| {
        let mut bv = bitvec![u16, Msb0; 0; params.binary_length];
        line.trim().chars().enumerate().for_each(|(i, c)| {
            if c == '1' {
                frequencies[i] += 1;
                bv.set(i, true);
            } else {
                frequencies[i] -= 1;
            }
        });
        codes.push(bv);
    });
    // sort codes
    codes.sort_unstable();
    // determine ratings codes
    let oxygen_code = filter(&codes, &frequencies, Rating::Oxygen);
    let co2_code = filter(&codes, &frequencies, Rating::CO2);
    // calculate life support rating
    let result = oxygen_code.load::<u32>() * co2_code.load::<u32>();
    result.to_string()
}
