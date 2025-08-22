mod pipe_type;

use crate::pipe_type::PipeType;
use aoc_geometry::point::Point;
use aoc_geometry::vector::Vector;
use std::collections::HashMap;

const PIPE_TRANSLATION: HashMap<PipeType, (Vector<i32, 2>, Vector<i32, 2>)> = HashMap::from([
    (
        PipeType::SouthEast,
        (Vector::new([0, 1]), Vector::new([1, 0])),
    ),
    (
        PipeType::Horizontal,
        (Vector::new([-1, 0]), Vector::new([1, 0])),
    ),
    (
        PipeType::NorthWest,
        (Vector::new([0, -1]), Vector::new([-1, 0])),
    ),
    (
        PipeType::Vertical,
        (Vector::new([0, 1]), Vector::new([0, -1])),
    ),
    (
        PipeType::NorthEast,
        (Vector::new([0, -1]), Vector::new([1, 0])),
    ),
    (
        PipeType::SouthWest,
        (Vector::new([0, 1]), Vector::new([-1, 0])),
    ),
]);

type Field = Vec<Vec<PipeType>>;

fn parse_input(input: &str) -> (Field, Point<i32, 2>) {
    let mut field: Field = Vec::new();
    let mut start: Point<i32, 2> = Point::new([0, 0]);
    input.trim().lines().enumerate().for_each(|(y, line)| {
        let line = line.trim();
        let mut row: Vec<PipeType> = Vec::with_capacity(line.len());
        line.chars()
            .enumerate()
            .for_each(|(x, c)| {
                row.insert(x, PipeType::from(c));
            });
    });

    (field, start)
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
    let (field, start) = parse_input(params.input_data);
    // TODO
    String::from("")
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
    // TODO
    String::from("")
}
