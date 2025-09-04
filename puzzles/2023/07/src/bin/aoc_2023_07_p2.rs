use aoc_utils::cli_utils::solve_part;

fn main() {
    solve_part(1, || {
        aoc_2023_07::solve_part2(aoc_2023_07::Part2Parameters {
            input_data: include_str!("../../data/input/input.txt"),
        })
    });
}
