mod beam;
mod tile_type;

use crate::beam::Beam;
use crate::tile_type::TileType;
use aoc_geometry::{CardinalDirection2D, Vector};
use aoc_geometry::{Grid2D, GridCoordinate2D};
use std::collections::HashSet;
use rayon::prelude::*;

type TileGrid = Grid2D<TileType>;

fn parse_input(input: &str) -> TileGrid {
    let data = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().map(|s| TileType::from(s)).collect())
        .collect();
    let mut grid = TileGrid::from_double_vec(data);
    grid.flip_vertical();
    grid
}

fn process_beam_in_tile(
    tile_type: &TileType,
    beam_direction: &CardinalDirection2D,
) -> Vec<CardinalDirection2D> {
    match tile_type {
        TileType::EmptySpace => vec![beam_direction.clone()],
        TileType::MirrorSlash => match beam_direction {
            CardinalDirection2D::Up => vec![CardinalDirection2D::Right],
            CardinalDirection2D::Down => vec![CardinalDirection2D::Left],
            CardinalDirection2D::Left => vec![CardinalDirection2D::Down],
            CardinalDirection2D::Right => vec![CardinalDirection2D::Up],
        },
        TileType::MirrorBackslash => match beam_direction {
            CardinalDirection2D::Up => vec![CardinalDirection2D::Left],
            CardinalDirection2D::Down => vec![CardinalDirection2D::Right],
            CardinalDirection2D::Left => vec![CardinalDirection2D::Up],
            CardinalDirection2D::Right => vec![CardinalDirection2D::Down],
        },
        TileType::SplitterVertical => match beam_direction {
            CardinalDirection2D::Up => vec![CardinalDirection2D::Up],
            CardinalDirection2D::Down => vec![CardinalDirection2D::Down],
            CardinalDirection2D::Left => vec![CardinalDirection2D::Up, CardinalDirection2D::Down],
            CardinalDirection2D::Right => vec![CardinalDirection2D::Up, CardinalDirection2D::Down],
        },
        TileType::SplitterHorizontal => match beam_direction {
            CardinalDirection2D::Up => vec![CardinalDirection2D::Left, CardinalDirection2D::Right],
            CardinalDirection2D::Down => {
                vec![CardinalDirection2D::Left, CardinalDirection2D::Right]
            }
            CardinalDirection2D::Left => vec![CardinalDirection2D::Left],
            CardinalDirection2D::Right => vec![CardinalDirection2D::Right],
        },
    }
}

fn process_position(tile_grid: &TileGrid, beam: &Beam) -> Vec<CardinalDirection2D> {
    process_beam_in_tile(
        tile_grid.get(beam.get_coordinates()).unwrap(),
        beam.get_direction(),
    )
}

fn move_around_grid(
    tile_grid: &TileGrid,
    coords: &GridCoordinate2D,
    direction: &CardinalDirection2D,
) -> Option<GridCoordinate2D> {
    let v: Vector<i8, 2> = direction.clone().into();
    tile_grid.try_move(&coords, &v)
}

fn process_recursively(
    tile_grid: &TileGrid,
    beam: &Beam,
    energized_tiles: &mut HashSet<GridCoordinate2D>,
    analyzed_beams: &mut HashSet<Beam>,
) {
    let output_beams = process_position(tile_grid, beam);
    for output_direction in output_beams {
        if let Some(new_coords) =
            move_around_grid(tile_grid, beam.get_coordinates(), &output_direction)
        {
            let new_beam = Beam::new(new_coords, output_direction);
            if !analyzed_beams.contains(&new_beam) {
                analyzed_beams.insert(new_beam.clone());
                energized_tiles.insert(*new_beam.get_coordinates());
                process_recursively(tile_grid, &new_beam, energized_tiles, analyzed_beams);
            }
        }
    }
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
    let tile_grid = parse_input(params.input_data);
    let initial_beam = Beam::new(
        aoc_geometry::Point::new([0, tile_grid.get_height() - 1]),
        CardinalDirection2D::Right,
    );
    let mut analyzed_beams = HashSet::new();
    analyzed_beams.insert(initial_beam.clone());
    let mut energized_tiles = HashSet::new();
    energized_tiles.insert(*initial_beam.get_coordinates());
    process_recursively(
        &tile_grid,
        &initial_beam,
        &mut energized_tiles,
        &mut analyzed_beams,
    );
    energized_tiles.len().to_string()
}

/// Parameters for solving Part 2 of the puzzle.
pub struct Part2Parameters {
    pub input_data: &'static str,
}

fn get_list_of_starting_beams(tile_grid: &TileGrid) -> Vec<Beam> {
    let mut beams = Vec::new();
    let width = tile_grid.get_width();
    let height = tile_grid.get_height();
    for x in 0..width {
        beams.push(Beam::new(
            aoc_geometry::Point::new([x, height - 1]),
            CardinalDirection2D::Down,
        ));
        beams.push(Beam::new(
            aoc_geometry::Point::new([x, 0]),
            CardinalDirection2D::Up,
        ));
    }
    for y in 0..height {
        beams.push(Beam::new(
            aoc_geometry::Point::new([0, y]),
            CardinalDirection2D::Right,
        ));
        beams.push(Beam::new(
            aoc_geometry::Point::new([width - 1, y]),
            CardinalDirection2D::Left,
        ));
    }
    beams
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
    let tile_grid = parse_input(params.input_data);
    get_list_of_starting_beams(&tile_grid)
        .par_iter()
        .map(|beam| -> u64 {
            let mut analyzed_beams = HashSet::new();
            analyzed_beams.insert(beam.clone());
            let mut energized_tiles = HashSet::new();
            energized_tiles.insert(*beam.get_coordinates());
            process_recursively(&tile_grid, beam, &mut energized_tiles, &mut analyzed_beams);
            energized_tiles.len() as u64
        })
        .max()
        .unwrap()
        .to_string()
}
