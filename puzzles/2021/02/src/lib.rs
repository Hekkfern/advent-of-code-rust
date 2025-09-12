use aoc_geometry::{Point, Vector};

// -----------------------------------------------------------
// ------------------------ Common ---------------------------
// -----------------------------------------------------------

// -----------------------------------------------------------
// ------------------------ Part 1 ---------------------------
// -----------------------------------------------------------

/// Parameters for solving Part 1 of the puzzle.
pub struct Part1Parameters {
    pub input_data: &'static str,
}

pub fn parse_instruction_for_part_1(line: &str) -> Vector<i8, 2> {
    let mut parts = line.split_whitespace();
    let command = parts.next().unwrap();
    let value: i8 = parts.next().unwrap().parse().unwrap();
    match command {
        "forward" => Vector::new([value, 0]),
        "down" => Vector::new([0, value]),
        "up" => Vector::new([0, -value]),
        _ => unreachable!(),
    }
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
    let mut current_position = Point::new([0, 0]);
    params.input_data.trim().lines().for_each(|line| {
        let movement = parse_instruction_for_part_1(line);
        current_position = current_position.move_by(&movement).unwrap();
    });
    let result = current_position[0] * current_position[1];
    result.to_string()
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
    let mut current_position = Point::new([0, 0]);
    let mut aim: i32 = 0;
    params.input_data.trim().lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        let command = parts.next().unwrap();
        let value: i32 = parts.next().unwrap().parse().unwrap();
        match command {
            "forward" => {
                let movement = Vector::new([value, aim * value]);
                current_position = current_position.move_by(&movement).unwrap();
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => unreachable!(),
        }
    });
    let result = current_position[0] * current_position[1];
    result.to_string()
}
