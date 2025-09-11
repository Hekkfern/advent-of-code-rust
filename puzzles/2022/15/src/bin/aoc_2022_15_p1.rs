use aoc_utils::cli_utils::solve_part;

fn main() {
    solve_part(1, || {
        aoc_2022_15::solve_part1(aoc_2022_15::Part1Parameters {
            input_data: include_str!("../../data/input/input.txt"),
            target_row: 2_000_000,
        })
    });
}
