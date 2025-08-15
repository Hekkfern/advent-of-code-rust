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
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let result: u64 = re
        .captures_iter(params.input_data)
        .map(|capture| {
            let (_, [a, b]) = capture.extract();
            let a: u64 = a.parse().unwrap();
            let b: u64 = b.parse().unwrap();
            a * b
        })
        .sum();
    result.to_string()
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
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut sum: u64 = 0;
    let mut is_mul_enabled = true;
    for m in re.captures_iter(params.input_data) {
        let instruction = m.get(0).unwrap().as_str();
        match instruction {
            "do()" => is_mul_enabled = true,
            "don't()" => is_mul_enabled = false,
            _ => {
                if is_mul_enabled {
                    let a: u64 = m.get(1).unwrap().as_str().parse().unwrap();
                    let b: u64 = m.get(2).unwrap().as_str().parse().unwrap();
                    sum += a * b;
                }
            }
        }
    }
    sum.to_string()
}
