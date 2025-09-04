use aoc_utils::cli_utils::solve_part;

fn main() {
    solve_part(1, || {
        aoc_2023_16::solve_part2(aoc_2023_16::Part2Parameters {
            input_data: include_str!("../../data/input/input.txt"),
        })
    });
}
