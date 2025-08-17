use crate::part::Part;
use crate::symbol::Symbol;
use aoc_geometry::orthogonal_line::OrthogonalLine;
use aoc_geometry::point::Point;
use aoc_geometry::vector::Vector;

mod part;
mod schematic;
mod symbol;

fn parse_input_line(idx: usize, line: &str) -> schematic::Schematic {
    let mut parts = Vec::<Part>::new();
    let mut symbols = Vec::<Symbol>::new();

    let mut chars = line.chars().enumerate().peekable();
    while let Some((i, c)) = chars.peek().cloned() {
        if c.is_ascii_digit() {
            // Start of a number
            let start = i;
            let mut end = i;
            let mut num_str = String::new();
            while let Some((j, d)) = chars.peek().cloned() {
                if d.is_ascii_digit() {
                    num_str.push(d);
                    end = j;
                    chars.next();
                } else {
                    break;
                }
            }
            if let Ok(num) = num_str.parse::<u32>() {
                let p1 = Point::new([start as i32, idx as i32]);
                let p2 = Point::new([end as i32, idx as i32]);
                parts.push(Part::new(num, [p1, p2]));
            }
        } else {
            // Symbol (not a digit)
            if c != '.' {
                let pos = Point::new([i as i32, idx as i32]);
                symbols.push(Symbol::new(c, pos));
            }
            chars.next();
        }
    }

    schematic::Schematic::new(parts, symbols)
}

fn parse_input(input: &str) -> schematic::Schematic {
    input
        .trim()
        .lines()
        .enumerate()
        .map(|(i, line)| parse_input_line(i, line.trim()))
        .fold(schematic::Schematic::default(), |mut acc, schematic| {
            acc.merge(schematic);
            acc
        })
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
    let schematic = parse_input(params.input_data);
    let result: u64 = schematic.parts().iter().fold(0, |acc, part| {
        if schematic.symbols().iter().any(|symbol| {
            part.vertexes().iter().any(|vertex| {
                Vector::from_points(vertex, symbol.position()).chebyshev_distance() <= 1
            })
        }) {
            acc + part.number() as u64
        } else {
            acc
        }
    });
    result.to_string()
}

/// Parameters for solving Part 2 of the puzzle.
pub struct Part2Parameters {
    pub input_data: &'static str,
}

const GEAR_SYMBOL: char = '*';

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
    let schematic = parse_input(params.input_data);
    let result: u32 = schematic
        .symbols()
        .iter()
        .filter(|symbol| symbol.character() == GEAR_SYMBOL)
        .filter_map(|symbol| {
            let close_parts: Vec<&Part> = schematic
                .parts()
                .iter()
                .filter(|part| {
                    part.vertexes().iter().any(|vertex| {
                        Vector::from_points(vertex, symbol.position()).chebyshev_distance() <= 1
                    })
                })
                .collect();
            if close_parts.len() == 2 {
                Some(close_parts[0].number() * close_parts[1].number())
            } else {
                None
            }
        })
        .sum();

    result.to_string()
}
