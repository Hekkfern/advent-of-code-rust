use aoc_utils::cli_utils::solve_part;

fn main() {
    solve_part(2, || {
        aoc_2023_11::solve_part2(aoc_2023_11::Part2Parameters {
            input_data: include_str!("../../data/input/input.txt"),
            empty_multiplier: 1_000_000,
        })
    });
}
