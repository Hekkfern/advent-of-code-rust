fn parse_input(input: &str) -> impl Iterator<Item = u64> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().parse::<u64>().unwrap())
}

/// Calculates the fuel required for a given mass.
///
/// # Arguments
/// * `mass` - The mass for which to calculate the fuel
///
/// # Returns
/// The amount of fuel required for the given mass
fn calculate_fuel(mass: u64) -> u64 {
    (mass / 3).saturating_sub(2)
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
    let total_fuel: u64 = parse_input(params.input_data).map(calculate_fuel).sum();
    total_fuel.to_string()
}

/// Parameters for solving Part 2 of the puzzle.
pub struct Part2Parameters {
    pub input_data: &'static str,
}

/// Calculates the total fuel required, including the fuel for the fuel itself.
///
/// # Arguments
/// * `mass` - The mass for which to calculate the total fuel
///
/// # Returns
/// The total amount of fuel required for the given mass, including the fuel for the fuel
fn calculate_total_fuel(mass: u64) -> u64 {
    let mut total_fuel = 0;
    let mut current_mass = mass;
    while current_mass > 0 {
        current_mass = calculate_fuel(current_mass);
        if current_mass > 0 {
            total_fuel += current_mass;
        }
    }
    total_fuel
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
    let total_fuel: u64 = parse_input(params.input_data)
        .map(calculate_total_fuel)
        .sum();
    total_fuel.to_string()
}
