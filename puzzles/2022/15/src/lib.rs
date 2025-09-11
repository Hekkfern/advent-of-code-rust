mod sensor_info;

use crate::sensor_info::SensorInfo;
use aoc_geometry::Point;
use aoc_intervals::interval::Interval;
use aoc_intervals::interval_set::IntervalSet;
use regex::Regex;
use std::collections::HashSet;

// -----------------------------------------------------------
// ------------------------ Common ---------------------------
// -----------------------------------------------------------

fn parse_input_line(line: &str) -> SensorInfo {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    let caps = re.captures(line).expect("Regex failed in parsing the line");
    let sx: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let sy: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
    let bx: i32 = caps.get(3).unwrap().as_str().parse().unwrap();
    let by: i32 = caps.get(4).unwrap().as_str().parse().unwrap();
    let sensor_position = Point::<i32, 2>::new([sx, sy]);
    let beacon_position = Point::<i32, 2>::new([bx, by]);
    SensorInfo::new(sensor_position, beacon_position)
}

// -----------------------------------------------------------
// ------------------------ Part 1 ---------------------------
// -----------------------------------------------------------

/// Parameters for solving Part 1 of the puzzle.
pub struct Part1Parameters {
    pub input_data: &'static str,
    pub target_row: i32,
}

fn fill_no_beacon_interval_in_target_row(
    interval: &mut IntervalSet<i32>,
    sensor_info: &SensorInfo,
    target_row: i32,
) {
    let distance = sensor_info.distance() as i32;
    let diff_y_abs = (target_row - sensor_info.sensor_position()[1]).abs();
    if diff_y_abs == distance {
        // add just one value
        interval.add_value(sensor_info.sensor_position()[0]);
    } else if diff_y_abs < distance {
        // add all the matching nodes
        let diff_x = distance - diff_y_abs;
        interval.add_interval(Interval::from_boundaries(
            sensor_info.sensor_position()[0] - diff_x,
            sensor_info.sensor_position()[0] + diff_x,
        ));
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
    let mut interval = IntervalSet::new();
    let mut busy_positions = HashSet::<Point<i32, 2>>::new();
    for line in params.input_data.lines() {
        let sensor_info = parse_input_line(line);
        if sensor_info.sensor_position()[1] == params.target_row {
            busy_positions.insert(*sensor_info.sensor_position());
        }
        if sensor_info.beacon_position()[1] == params.target_row {
            busy_positions.insert(*sensor_info.beacon_position());
        }
        fill_no_beacon_interval_in_target_row(&mut interval, &sensor_info, params.target_row);
    }
    for p in busy_positions {
        interval.remove_value(*p.get(0));
    }
    interval.count().to_string()
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
    // TODO
    String::from("")
}
