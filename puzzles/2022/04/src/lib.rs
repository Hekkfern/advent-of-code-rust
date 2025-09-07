use aoc_intervals::interval::Interval;

fn parse_input_line(line: &str) -> [Interval<i32>; 2] {
    line.split(',')
        .map(|part| {
            let bounds: Vec<i32> = part
                .split('-')
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            Interval::from_boundaries(bounds[0], bounds[1])
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
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
    let total_overlaps = params
        .input_data
        .trim()
        .lines()
        .map(parse_input_line)
        .filter(|sections| sections[0].subsumes(&sections[1]) || sections[1].subsumes(&sections[0]))
        .count();
    total_overlaps.to_string()
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
    let total_no_overlaps = params
        .input_data
        .trim()
        .lines()
        .map(parse_input_line)
        .filter(|sections| sections[0].overlaps(&sections[1]))
        .count();
    total_no_overlaps.to_string()
}
