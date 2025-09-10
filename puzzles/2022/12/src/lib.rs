mod map;
mod position;
mod types;

use crate::map::Map;
use crate::position::Position;
use crate::types::Height;
use aoc_geometry::{Grid2D, GridCoordinate2D, Vector};
use std::collections::{HashMap, VecDeque};

// -----------------------------------------------------------
// ------------------------ Common ---------------------------
// -----------------------------------------------------------

fn parse_height(c: char) -> Height {
    match c {
        'S' => 0,
        'E' => 'z' as u8 - 'a' as u8,
        _ => c as u8 - 'a' as u8,
    }
}

fn parse_input(input: &str) -> Map {
    let mut start: Position = Position::default();
    let mut end: Position = Position::default();
    let heights: Vec<Vec<Height>> = input
        .trim()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    let h = parse_height(c);
                    if c == 'S' {
                        start = Position::new(GridCoordinate2D::new([x, y]), h);
                    } else if c == 'E' {
                        end = Position::new(GridCoordinate2D::new([x, y]), h);
                    }
                    h
                })
                .collect()
        })
        .collect();
    let grid = Grid2D::from_double_vec(heights);
    Map::new(grid, start, end)
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum ClimbingDirection {
    Up,
    Down,
}

fn get_next_positions(
    map: &Map,
    pos: &Position,
    climbing_direction: ClimbingDirection,
) -> Vec<Position> {
    let mut next_positions = Vec::new();
    let deltas = vec![
        Vector::<i8, 2>::new([0, 1]),
        Vector::<i8, 2>::new([1, 0]),
        Vector::<i8, 2>::new([0, -1]),
        Vector::<i8, 2>::new([-1, 0]),
    ];
    for delta in deltas {
        if let Some(new_coord) = map.get_grid().try_move(pos.get_point(), &delta) {
            let new_coord_height = *map.get_grid().get(&new_coord).unwrap();
            if climbing_direction == ClimbingDirection::Up
                && new_coord_height <= pos.get_height() + 1
            {
                next_positions.push(Position::new(new_coord, new_coord_height));
            } else if climbing_direction == ClimbingDirection::Down
                && new_coord_height + 1 >= pos.get_height()
            {
                next_positions.push(Position::new(new_coord, new_coord_height));
            }
        }
    }
    next_positions
}

// -----------------------------------------------------------
// ------------------------ Part 1 ---------------------------
// -----------------------------------------------------------

/// Parameters for solving Part 1 of the puzzle.
pub struct Part1Parameters {
    pub input_data: &'static str,
}

fn climb_hill(map: &Map) -> u32 {
    let mut queue = VecDeque::<(Position, u32)>::new();
    queue.push_back((*map.get_origin(), 0));
    let mut costs = HashMap::<Position, u32>::new();
    costs.insert(*map.get_origin(), 0);

    while let Some((current_pos, current_cost)) = queue.pop_front() {
        if *current_pos.get_point() == *map.get_destination().get_point() {
            return current_cost;
        }
        let next_positions = get_next_positions(map, &current_pos, ClimbingDirection::Up);
        for next_pos in next_positions {
            let next_cost = current_cost + 1;
            // Check if the path, due to this movement, is not longer than the shortest known path.
            if !costs.contains_key(&next_pos) || next_cost < *costs.get(&next_pos).unwrap() {
                costs.insert(next_pos, next_cost);
                queue.push_back((next_pos, next_cost));
            }
        }
    }
    unreachable!()
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
    let map = parse_input(params.input_data);
    let steps = climb_hill(&map);
    steps.to_string()
}

// -----------------------------------------------------------
// ------------------------ Part 2 ---------------------------
// -----------------------------------------------------------

/// Parameters for solving Part 2 of the puzzle.
pub struct Part2Parameters {
    pub input_data: &'static str,
}

fn descend_hill(map: &Map) -> u32 {
    let mut best_cost = u32::MAX;
    let mut queue = VecDeque::<(Position, u32)>::new();
    queue.push_back((*map.get_destination(), 0));
    let mut costs = HashMap::<Position, u32>::new();
    costs.insert(*map.get_destination(), 0);

    while let Some((current_pos, current_cost)) = queue.pop_front() {
        if current_pos.get_height() == 0 {
            best_cost = std::cmp::min(current_cost, best_cost);
            // this is the end of this path
            continue;
        }
        let next_positions = get_next_positions(map, &current_pos, ClimbingDirection::Down);
        for next_pos in next_positions {
            let next_cost = current_cost + 1;
            // Check if the path, due to this movement, is not longer than the shortest known path.
            if !costs.contains_key(&next_pos) || next_cost < *costs.get(&next_pos).unwrap() {
                costs.insert(next_pos, next_cost);
                queue.push_back((next_pos, next_cost));
            }
        }
    }
    best_cost
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
    let map = parse_input(params.input_data);
    let steps = descend_hill(&map);
    steps.to_string()
}
