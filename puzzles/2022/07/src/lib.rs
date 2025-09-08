fn parse<'a>(input: &mut impl Iterator<Item = &'a str>) -> Vec<u64> {
    let mut total = 0;
    let mut subdirs = vec![];
    loop {
        match input
            .next()
            .map(|s| s.split_whitespace().collect::<Vec<_>>())
            .as_deref()
        {
            Some(["$", "cd", ".."]) | None => break,
            Some(["$", "cd", s]) if *s != "/" => {
                subdirs.extend(parse(input));
                total += subdirs.last().unwrap();
            }
            Some([s, _]) if *s != "$" && *s != "dir" => {
                total += s.parse::<u64>().unwrap();
            }
            _ => (),
        }
    }
    subdirs.push(total);
    subdirs
}

/// Parameters for solving Part 1 of the puzzle.
pub struct Part1Parameters {
    pub input_data: &'static str,
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
    parse(&mut params.input_data.lines())
        .into_iter()
        .filter(|&s| s <= 100_000)
        .sum::<u64>()
        .to_string()
}

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
    let mut sizes = parse(&mut params.input_data.lines());
    let missing = 30_000_000 - (70_000_000 - sizes.last().unwrap());
    sizes.sort_unstable();
    sizes
        .into_iter()
        .find(|&s| s >= missing)
        .unwrap()
        .to_string()
}
