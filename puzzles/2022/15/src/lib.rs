mod sensor_info;

use crate::sensor_info::SensorInfo;
use aoc_geometry::{Point, SquareDiamond2D};
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
    pub max_grid_size: i32,
}

fn calculate_tuning_frequency(p: Point<i32, 2>) -> u64 {
    (p[0] as u64 * 4_000_000) + p[1] as u64
}

fn search_for_distress_beacon(
    sensor_info_list: &Vec<SensorInfo>,
    max_grid_size: i32,
) -> Point<i32, 2> {
    for sensor_info in sensor_info_list {
        let diamond = SquareDiamond2D::from_center_and_perimeter_point(
            sensor_info.sensor_position(),
            sensor_info.beacon_position(),
        );
        for position in diamond.step_around_outside_border() {
            if position[0] >= 0
                && position[0] <= max_grid_size
                && position[1] >= 0
                && position[1] <= max_grid_size
                && sensor_info_list.iter().all(|s| {
                    let v = aoc_geometry::Vector::<i64, 2>::from_points(
                        s.sensor_position(),
                        &position,
                    )
                    .unwrap();
                    v.manhattan_distance() > s.distance()
                })
            {
                return position;
            }
        }
    }
    unreachable!();
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
    let sensors: Vec<SensorInfo> = params.input_data.lines().map(parse_input_line).collect();
    let distress_beacon_position = search_for_distress_beacon(&sensors, params.max_grid_size);
    calculate_tuning_frequency(distress_beacon_position).to_string()
}
