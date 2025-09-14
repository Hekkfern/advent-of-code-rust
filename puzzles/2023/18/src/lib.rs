mod instruction;

use aoc_geometry::{CardinalDirection2D, OrthogonalPolygon2D, Point, Vector};
use instruction::Instruction;
use std::ops::Mul;

const DIMENSIONS: usize = 2;

fn parse_input<F>(input: &str, parse_instruction: F) -> Vec<Instruction>
where
    F: Fn(&str) -> Instruction,
{
    input.lines().map(parse_instruction).collect()
}

fn solve(instructions: &[Instruction]) -> u64 {
    let starting_point = Point::<i64, DIMENSIONS>::origin();
    /* convert instructions into vertexes of a polygon */
    let vertexes: Vec<Point<i64, 2>> = instructions
        .iter()
        .scan(starting_point, |current_point, instruction| {
            let movement: Vector<i64, DIMENSIONS> = instruction
                .direction()
                .to_vector()
                .mul(instruction.steps() as i32)
                .unwrap();
            *current_point = current_point.move_by(&movement).unwrap();
            Some(*current_point)
        })
        .collect();
    /* generate result */
    let polygon = OrthogonalPolygon2D::from_vertices(vertexes);
    polygon.number_of_intrinsic_points() + polygon.perimeter()
}

/// Parameters for solving Part 1 of the puzzle.
pub struct Part1Parameters {
    pub input_data: &'static str,
}

fn parse_instruction_for_part1(line: &str) -> Instruction {
    let parts = line.split_whitespace().collect::<Vec<_>>();
    let direction = match parts[0] {
        "R" => CardinalDirection2D::Right,
        "L" => CardinalDirection2D::Left,
        "U" => CardinalDirection2D::Up,
        "D" => CardinalDirection2D::Down,
        _ => unreachable!(),
    };
    let steps = parts[1].parse::<u64>().unwrap();
    Instruction::new(direction, steps)
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
    let instructions = parse_input(params.input_data, parse_instruction_for_part1);
    let result = solve(&instructions);
    result.to_string()
}

/// Parameters for solving Part 2 of the puzzle.
pub struct Part2Parameters {
    pub input_data: &'static str,
}

fn parse_instruction_for_part2(line: &str) -> Instruction {
    let parts = line.split_whitespace().collect::<Vec<_>>();
    let code = &parts[2][2..=7];
    let direction = match code.chars().last().unwrap() {
        '0' => CardinalDirection2D::Right,
        '1' => CardinalDirection2D::Down,
        '2' => CardinalDirection2D::Left,
        '3' => CardinalDirection2D::Up,
        _ => unreachable!(),
    };
    let steps = u64::from_str_radix(&code[0..=4], 16).unwrap();
    Instruction::new(direction, steps)
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
    let instructions = parse_input(params.input_data, parse_instruction_for_part2);
    let result = solve(&instructions);
    result.to_string()
}
