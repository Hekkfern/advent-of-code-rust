use aoc_geometry::OrthogonalLine2D;
use aoc_geometry::Point;
use itertools::Itertools;
use std::collections::HashSet;

// -----------------------------------------------------------
// ------------------------ Common ---------------------------
// -----------------------------------------------------------

fn parse_point(point_str: &str) -> Point<i32, 2> {
    let coords: Vec<i32> = point_str
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    Point::<i32, 2>::new([coords[0], coords[1]])
}

// -----------------------------------------------------------
// ------------------------ Part 1 ---------------------------
// -----------------------------------------------------------

/// Parameters for solving Part 1 of the puzzle.
pub struct Part1Parameters {
    pub input_data: &'static str,
}

fn parse_input_for_part_1(input: &str) -> Vec<OrthogonalLine2D<i32>> {
    input
        .trim()
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(" -> ");
            let start_point = parse_point(parts.next().unwrap());
            let end_point = parse_point(parts.next().unwrap());
            if start_point[0] == end_point[0] || start_point[1] == end_point[1] {
                Some((start_point, end_point))
            } else {
                None
            }
        })
        .map(|(start_point, end_point)| OrthogonalLine2D::from_points(&start_point, &end_point))
        .collect()
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
    let lines = parse_input_for_part_1(params.input_data);
    let mut points_map = HashSet::<Point<i32, 2>>::new();
    for line_pair in lines.iter().combinations(2) {
        let line1 = line_pair[0];
        let line2 = line_pair[1];
        let intersection = line1.intersect(line2);
        if !intersection.is_empty() {
            points_map.extend(intersection);
        }
    }
    points_map.len().to_string()
}

// -----------------------------------------------------------
// ------------------------ Part 2 ---------------------------
// -----------------------------------------------------------

/// Parameters for solving Part 2 of the puzzle.
pub struct Part2Parameters {
    pub input_data: &'static str,
}

fn parse_input_for_part_2(input: &str) -> Vec<OrthogonalLine2D<i32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split(" -> ");
            let start_point = parse_point(parts.next().unwrap());
            let end_point = parse_point(parts.next().unwrap());
            OrthogonalLine2D::from_points(&start_point, &end_point)
        })
        .collect()
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
    let lines = parse_input_for_part_2(params.input_data);
    let mut points_map = HashSet::<Point<i32, 2>>::new();
    for line_pair in lines.iter().combinations(2) {
        let line1 = line_pair[0];
        let line2 = line_pair[1];
        let intersection = line1.intersect(line2);
        if !intersection.is_empty() {
            points_map.extend(intersection);
        }
    }
    points_map.len().to_string()
}
