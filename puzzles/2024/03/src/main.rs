use aoc_utils::cli_utils::{create_cli, solve_part};

fn main() {
    let cli = create_cli(2024, 3);

    let cli_matches = cli.get_matches();
    let part: &u8 = cli_matches.get_one("part").unwrap();
    match part {
        1 => {
            solve_part(1, || {
                aoc_2024_03::solve_part1(aoc_2024_03::Part1Parameters {
                    input_data: include_str!("../data/input/input.txt"),
                })
            });
        }
        2 => {
            solve_part(2, || {
                aoc_2024_03::solve_part2(aoc_2024_03::Part2Parameters {
                    input_data: include_str!("../data/input/input.txt"),
                })
            });
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
