mod galaxy_map;

use aoc_geometry::Point;
use aoc_geometry::Vector;
use galaxy_map::GalaxyMap;
use itertools::Itertools;

const GALAXY_CHARACTER: char = '#';

fn parse_input(input: &str) -> GalaxyMap {
    let mut galaxies = Vec::new();
    let mut empty_rows = Vec::new();
    let lines: Vec<&str> = input.trim().lines().collect();
    let num_cols = lines[0].len();
    let mut galaxies_per_column = vec![0usize; num_cols];

    for (row_idx, line) in lines.iter().enumerate() {
        let mut galaxy_count = 0;
        line.trim()
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == GALAXY_CHARACTER)
            .for_each(|(col_idx, _)| {
                galaxies.push(Point::new([col_idx, row_idx]));
                galaxies_per_column[col_idx] += 1;
                galaxy_count += 1;
            });
        if galaxy_count == 0 {
            empty_rows.push(row_idx);
        }
    }

    let mut empty_columns = Vec::new();
    galaxies_per_column
        .iter()
        .enumerate()
        .filter(|(_, num_galaxies)| **num_galaxies == 0)
        .for_each(|(col_idx, _)| {
            empty_columns.push(col_idx);
        });
    empty_rows.sort_unstable();
    empty_columns.sort_unstable();
    GalaxyMap::new(galaxies, empty_rows, empty_columns)
}

fn count_empty_between(empty_indices: &[usize], a: usize, b: usize) -> usize {
    let (min, max) = if a < b { (a, b) } else { (b, a) };
    empty_indices
        .iter()
        .filter(|&&idx| min < idx && idx < max)
        .count()
}

fn calculate_accumulated_shortest_paths(galaxy_map: &GalaxyMap, empty_multiplier: u64) -> u64 {
    let mut accum_shortest_paths: u64 = 0;
    galaxy_map
        .get_galaxies()
        .iter()
        .combinations(2)
        .for_each(|galaxy_combination| {
            let (g1, g2) = (galaxy_combination[0], galaxy_combination[1]);
            let distance = Vector::<i64, 2>::from_points(g1, g2)
                .unwrap()
                .manhattan_distance();
            let num_empty_rows =
                count_empty_between(galaxy_map.get_empty_rows(), *g1.get(1), *g2.get(1));
            let num_empty_cols =
                count_empty_between(galaxy_map.get_empty_columns(), *g1.get(0), *g2.get(0));
            accum_shortest_paths += distance
                + (num_empty_rows as u64 * (empty_multiplier - 1))
                + (num_empty_cols as u64 * (empty_multiplier - 1));
        });
    accum_shortest_paths
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
    let galaxy_map = parse_input(params.input_data);
    let total_distance = calculate_accumulated_shortest_paths(&galaxy_map, 2);
    total_distance.to_string()
}

/// Parameters for solving Part 2 of the puzzle.
pub struct Part2Parameters {
    pub input_data: &'static str,
    pub empty_multiplier: u64,
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
    let galaxy_map = parse_input(params.input_data);
    let total_distance = calculate_accumulated_shortest_paths(&galaxy_map, params.empty_multiplier);
    total_distance.to_string()
}
