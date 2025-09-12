use aoc_2021_03::{Part1Parameters, Part2Parameters, solve_part1, solve_part2};
use assertables::assert_not_empty;
use include_dir::{Dir, include_dir};
use pretty_assertions::assert_eq;
use std::path::Path;

static DATA_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/data");
fn read_data_file<P: AsRef<Path>>(file_path: P) -> &'static str {
    DATA_DIR
        .get_file(file_path)
        .unwrap()
        .contents_utf8()
        .unwrap()
}

macro_rules! test_part1_case {
    ($params:expr, $expected_path:expr) => {{
        let result = solve_part1($params);
        assert_not_empty!(&result);
        let expected_result = read_data_file($expected_path);
        assert_eq!(result, expected_result);
    }};
}

macro_rules! test_part2_case {
    ($params:expr, $expected_path:expr) => {{
        let result = solve_part2($params);
        assert_not_empty!(&result);
        let expected_result = read_data_file($expected_path);
        assert_eq!(result, expected_result);
    }};
}

// -----------------------------------------------------------
// -------------------- Tests for Part 1  --------------------
// -----------------------------------------------------------

#[test]
fn test_part1_real() {
    let input_data = read_data_file("input/input.txt");
    test_part1_case!(
        Part1Parameters {
            input_data,
            binary_length: 12
        },
        "solutions/solution_part1.txt"
    );
}

#[test]
fn test_part1_test1() {
    let input_data = read_data_file("input/input_part1_test1.txt");
    test_part1_case!(
        Part1Parameters {
            input_data,
            binary_length: 5
        },
        "solutions/solution_part1_test1.txt"
    );
}

// -----------------------------------------------------------
// -------------------- Tests for Part 2  --------------------
// -----------------------------------------------------------

#[test]
fn test_part2_real() {
    let input_data = read_data_file("input/input.txt");
    test_part2_case!(
        Part2Parameters {
            input_data,
            binary_length: 12
        },
        "solutions/solution_part2.txt"
    );
}

#[test]
fn test_part2_test1() {
    let input_data = read_data_file("input/input_part2_test1.txt");
    test_part2_case!(
        Part2Parameters {
            input_data,
            binary_length: 5
        },
        "solutions/solution_part2_test1.txt"
    );
}
