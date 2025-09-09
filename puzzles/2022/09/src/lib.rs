use crate::rope::{Coordinate, Rope};
use crate::rope_instruction::RopeInstruction;
use aoc_geometry::CardinalDirection2D;
use std::collections::HashSet;

mod rope;
mod rope_instruction;

fn parse_direction_char(direction: char) -> CardinalDirection2D {
    match direction {
        'U' => CardinalDirection2D::Up,
        'D' => CardinalDirection2D::Down,
        'L' => CardinalDirection2D::Left,
        'R' => CardinalDirection2D::Right,
        _ => unreachable!(),
    }
}

fn parse_input_line(line: &str) -> RopeInstruction {
    let mut parts = line.split_whitespace();
    let direction_char = parts.next().unwrap().chars().next().unwrap();
    let steps: u32 = parts.next().unwrap().parse().unwrap();
    let direction = parse_direction_char(direction_char);
    RopeInstruction::new(direction, steps)
}

fn solve<const ROPE_LENGTH: usize>(input_data: &str) -> u32 {
    let mut rope = Rope::<ROPE_LENGTH>::new();
    let mut visited_tail_positions = HashSet::<Coordinate>::new();
    input_data.lines().for_each(|line| {
        let instruction = parse_input_line(line);
        for _ in 0..instruction.steps() {
            rope.move_head(instruction.direction().clone());
            visited_tail_positions.insert(*rope.tail());
        }
    });
    visited_tail_positions.len() as u32
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
    const ROPE_LENGTH: usize = 2;
    solve::<ROPE_LENGTH>(params.input_data).to_string()
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
    const ROPE_LENGTH: usize = 10;
    solve::<ROPE_LENGTH>(params.input_data).to_string()
}
