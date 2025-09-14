mod race;

use crate::race::Race;

fn is_exact_double(value: f64) -> bool {
    let rem = value % 1.0;
    rem < f64::EPSILON && rem > -f64::EPSILON
}

fn count_race_wins(race: &Race) -> u64 {
    let race_time = race.time() as f64;
    let race_distance = race.distance() as f64;
    let discriminant = (race_time * race_time) - (4.0 * race_distance);
    if discriminant < 0.0 {
        return 0;
    }
    let sqrt_disc = discriminant.sqrt();
    let first_win_d = (race_time - sqrt_disc) / 2.0;
    let last_win_d = (race_time + sqrt_disc) / 2.0;

    let last_index = if is_exact_double(last_win_d) {
        (last_win_d - 1.0) as u64
    } else {
        last_win_d.floor() as u64
    };
    let first_index = if is_exact_double(last_win_d) {
        (first_win_d + 1.0) as u64
    } else {
        first_win_d.ceil() as u64
    };
    if last_index < first_index {
        0
    } else {
        last_index - first_index + 1
    }
}

/// Parameters for solving Part 1 of the puzzle.
pub struct Part1Parameters {
    pub input_data: &'static str,
}

fn parse_input_for_part1(input: &str) -> Vec<Race> {
    let mut lines = input.trim().lines();

    // Parse times
    let times_line = lines.next().unwrap();
    let times: Vec<u64> = times_line
        .split_whitespace()
        .skip(1) // skip the label
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    // Parse distances
    let distances_line = lines.next().unwrap();
    let distances: Vec<u64> = distances_line
        .split_whitespace()
        .skip(1) // skip the label
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Race::new(time, distance))
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
    let races = parse_input_for_part1(params.input_data);
    let result: u64 = races.iter().map(count_race_wins).product();
    result.to_string()
}

/// Parameters for solving Part 2 of the puzzle.
pub struct Part2Parameters {
    pub input_data: &'static str,
}

fn parse_input_for_part2(input: &str) -> Race {
    let mut lines = input.trim().lines();

    // Parse time
    let times_line = lines.next().unwrap();
    let time_str: String = times_line
        .split_whitespace()
        .skip(1) // skip the label
        .collect::<String>();
    let time = time_str.parse::<u64>().unwrap();

    // Parse distance
    let distances_line = lines.next().unwrap();
    let distance_str: String = distances_line
        .split_whitespace()
        .skip(1) // skip the label
        .collect::<String>();
    let distance = distance_str.parse::<u64>().unwrap();

    Race::new(time, distance)
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
    let race = parse_input_for_part2(params.input_data);
    let result = count_race_wins(&race);
    result.to_string()
}
