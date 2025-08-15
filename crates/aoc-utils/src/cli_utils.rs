use clap::{Arg, Command, value_parser};

pub fn create_cli(year: u32, day: u32) -> Command {
    assert!(
        (2015..=2099).contains(&year),
        "Year must be between 2015 and 2099"
    );
    assert!((1..=25).contains(&day), "Day must be between 1 and 25");
    Command::new(concat!(
        "aoc-",
        stringify!(year),
        "-",
        stringify!(format!("{day:02}"))
    ))
    .about(format!("Solver for Advent of Code {year}, Day {day:02}"))
    .disable_help_subcommand(true)
    .arg_required_else_help(true)
    .arg(
        Arg::new("part")
            .short('p')
            .long("part")
            .required(true)
            .value_parser(value_parser!(u8).range(1..=2))
            .help("Part of the puzzle to run [values = 1, 2]"),
    )
}

pub fn solve_part<F>(part: u8, solver: F)
where
    F: FnOnce() -> String,
{
    println!("Solving Part {part}...");
    let start_time = std::time::Instant::now();
    let result = solver();
    let elapsed_time = start_time.elapsed();
    println!(
        "Result: {}",
        if !result.is_empty() {
            result
        } else {
            "No solution found".to_string()
        }
    );
    println!(
        "Solving Part {part} took {} microseconds.",
        elapsed_time.as_micros()
    );
}
