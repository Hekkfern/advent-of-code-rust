mod packet;

use crate::packet::Packet;

// -----------------------------------------------------------
// ------------------------ Part 1 ---------------------------
// -----------------------------------------------------------

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
    let mut packets = params
        .input_data
        .trim()
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| serde_json::from_str::<Packet>(&l).unwrap());
    let mut sum: u32 = 0;
    for i in 1.. {
        let Some(left) = packets.next() else {
            break;
        };
        let right = packets.next().unwrap();
        if left < right {
            sum += i;
        }
    }
    sum.to_string()
}

// -----------------------------------------------------------
// ------------------------ Part 2 ---------------------------
// -----------------------------------------------------------

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
    let mut all: Vec<Packet> = params
        .input_data
        .trim()
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    let div1: Packet = serde_json::from_str("[[2]]").unwrap();
    let div2: Packet = serde_json::from_str("[[6]]").unwrap();
    all.push(div1.clone());
    all.push(div2.clone());
    all.sort_unstable();
    let idx1 = all.binary_search(&div1).unwrap() + 1;
    let idx2 = all.binary_search(&div2).unwrap() + 1;
    (idx1 * idx2).to_string()
}
