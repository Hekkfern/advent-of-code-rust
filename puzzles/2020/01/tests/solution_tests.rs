use aoc_2020_01::{Part1Parameters, Part2Parameters, solve_part1, solve_part2};
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
        .trim()
}

#[test]
fn test_part1_real() {
    let input_data = read_data_file("input/input.txt");
    let result = solve_part1(Part1Parameters { input_data });
    assert_not_empty!(&result);
    let expected_result = read_data_file("solutions/solution_part1.txt");
    assert_eq!(result, expected_result);
}

#[test]
fn test_part1_test1() {
    let input_data = read_data_file("input/input_part1_test1.txt");
    let result = solve_part1(Part1Parameters { input_data });
    assert_not_empty!(&result);
    let expected_result = read_data_file("solutions/solution_part1_test1.txt");
    assert_eq!(result, expected_result);
}

#[test]
fn test_part2_real() {
    let input_data = read_data_file("input/input.txt");
    let result = solve_part2(Part2Parameters { input_data });
    assert_not_empty!(&result);
    let expected_result = read_data_file("solutions/solution_part2.txt");
    assert_eq!(result, expected_result);
}

#[test]
fn test_part2_test1() {
    let input_data = read_data_file("input/input_part2_test1.txt");
    let result = solve_part2(Part2Parameters { input_data });
    assert_not_empty!(&result);
    let expected_result = read_data_file("solutions/solution_part2_test1.txt");
    assert_eq!(result, expected_result);
}
