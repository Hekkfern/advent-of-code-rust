mod pipe_type;

use crate::pipe_type::PipeType;
use aoc_geometry::grid2d::Grid2D;
use aoc_geometry::point::Point;
use aoc_geometry::vector::Vector;
use std::collections::HashMap;

type Field = Grid2D<PipeType>;

fn parse_input(input: &str) -> (Field, Point<i32, 2>) {
    let mut grid_data = Vec::new();
    let mut start: Point<i32, 2> = Point::new([0, 0]);
    input.trim().lines().enumerate().for_each(|(y, line)| {
        let line = line.trim();
        let mut row: Vec<PipeType> = Vec::with_capacity(line.len());
        line.chars().enumerate().for_each(|(x, c)| {
            row.insert(x, PipeType::from(c));
            if c == 'S' {
                start = Point::new([x as i32, y as i32]);
            }
        });
        grid_data.push(row);
    });
    let mut field = Grid2D::from_double_vec(grid_data);
    field.flip_vertical();

    (field, start)
}

fn get_pipe_translation(pipe: PipeType) -> (Vector<i32, 2>, Vector<i32, 2>) {
    match pipe {
        PipeType::SouthEast => (Vector::new([0, -1]), Vector::new([1, 0])),
        PipeType::Horizontal => (Vector::new([-1, 0]), Vector::new([1, 0])),
        PipeType::NorthWest => (Vector::new([0, 1]), Vector::new([-1, 0])),
        PipeType::Vertical => (Vector::new([0, 1]), Vector::new([0, -1])),
        PipeType::NorthEast => (Vector::new([0, 1]), Vector::new([1, 0])),
        PipeType::SouthWest => (Vector::new([0, -1]), Vector::new([-1, 0])),
        _ => unreachable!(),
    }
}

fn move_across_field(
    field: &Field,
    pipe_position: &Point<i32, 2>,
    previous_position: &Point<i32, 2>,
) -> Point<i32, 2> {
    let translation = get_pipe_translation(
        field
            .get(&[
                pipe_position.get(0).clone() as usize,
                pipe_position.get(1).clone() as usize,
            ])
            .unwrap()
            .clone(),
    );
    if *pipe_position + translation.0 == *previous_position {
        *pipe_position + translation.1
    } else {
        *pipe_position + translation.0
    }
}

fn get_starting_neighbor(field: &Field, start: &Point<i32, 2>) -> Point<i32, 2> {
    for neighbor in start.get_neighbours() {
        let translation = get_pipe_translation(
            field
                .get(&[
                    neighbor.get(0).clone() as usize,
                    neighbor.get(1).clone() as usize,
                ])
                .unwrap()
                .clone(),
        );
        if (neighbor + translation.0 == *start) || (neighbor + translation.1 == *start) {
            return neighbor;
        }
    }
    unreachable!()
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
