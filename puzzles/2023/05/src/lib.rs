use crate::instructions::Instructions;
use crate::rangemap::RangeMap;
use aoc_intervals::interval::Interval;
use aoc_intervals::interval_set::IntervalSet;

mod instructions;
mod rangemap;
mod rangemap_section;

fn parse_map(block: &str) -> RangeMap {
    let mut map = RangeMap::new();
    for line in block.lines().skip(1) {
        // skip the title
        let nums: Vec<i64> = line
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();
        assert_eq!(nums.len(), 3);
        map.add_section(nums[0], nums[1], nums[2]);
    }
    map
}

/// Parameters for solving Part 1 of the puzzle.
pub struct Part1Parameters {
    pub input_data: &'static str,
}

fn parse_individual_seeds(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect()
}

fn parse_input_for_part1(input: &str) -> (Vec<i64>, Instructions) {
    let mut sections = input.trim().split("\n\n");
    let seeds_line = sections.next().unwrap().trim();
    let seeds = parse_individual_seeds(seeds_line.trim_start_matches("seeds: "));
    let maps: Vec<RangeMap> = sections.map(|block| parse_map(block)).collect();
    assert_eq!(maps.len(), 7, "Expected 7 mapping blocks");
    let instructions = Instructions::new(
        maps[0].clone(),
        maps[1].clone(),
        maps[2].clone(),
        maps[3].clone(),
        maps[4].clone(),
        maps[5].clone(),
        maps[6].clone(),
    );
    (seeds, instructions)
}

fn get_seed_location(instructions: &Instructions, seed: i64) -> i64 {
    instructions.humidity2location().convert_key(
        instructions.temperature2humidity().convert_key(
            instructions.light2temperature().convert_key(
                instructions.water2light().convert_key(
                    instructions.fertilizer2water().convert_key(
                        instructions
                            .soil2fertilizer()
                            .convert_key(instructions.seed2soil().convert_key(seed)),
                    ),
                ),
            ),
        ),
    )
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
    let (seeds, instructions) = parse_input_for_part1(params.input_data);
    seeds
        .iter()
        .map(|x| get_seed_location(&instructions, *x))
        .min()
        .unwrap()
        .to_string()
}

/// Parameters for solving Part 2 of the puzzle.
pub struct Part2Parameters {
    pub input_data: &'static str,
}

fn parse_ranged_seeds(line: &str) -> IntervalSet<i64> {
    let mut intervals = IntervalSet::new();
    let nums: Vec<i64> = line
        .split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();
    assert_eq!(nums.len() % 2, 0, "Expected an even number of seed values");
    for pair in nums.chunks(2) {
        intervals.add(Interval::from_size(pair[0], pair[1]));
    }
    intervals
}

fn parse_input_for_part2(input: &str) -> (IntervalSet<i64>, Instructions) {
    let mut sections = input.trim().split("\n\n");
    let seeds_line = sections.next().unwrap().trim();
    let intervals = parse_ranged_seeds(seeds_line.trim_start_matches("seeds: "));
    let maps: Vec<RangeMap> = sections.map(|block| parse_map(block)).collect();
    assert_eq!(maps.len(), 7, "Expected 7 mapping blocks");
    let instructions = Instructions::new(
        maps[0].clone(),
        maps[1].clone(),
        maps[2].clone(),
        maps[3].clone(),
        maps[4].clone(),
        maps[5].clone(),
        maps[6].clone(),
    );
    (intervals, instructions)
}

fn convert_seed_range(range_map: &RangeMap, seed_range: &IntervalSet<i64>) -> IntervalSet<i64> {
    let mut result = IntervalSet::new();
    for interval in seed_range.get().iter() {
        result = result.join(&range_map.convert_interval(&interval));
    }
    result
}

fn get_seed_range_locations(
    instructions: &Instructions,
    seed_range: &IntervalSet<i64>,
) -> IntervalSet<i64> {
    convert_seed_range(
        instructions.humidity2location(),
        &convert_seed_range(
            instructions.temperature2humidity(),
            &convert_seed_range(
                instructions.light2temperature(),
                &convert_seed_range(
                    instructions.water2light(),
                    &convert_seed_range(
                        instructions.fertilizer2water(),
                        &convert_seed_range(
                            instructions.soil2fertilizer(),
                            &convert_seed_range(instructions.seed2soil(), seed_range),
                        ),
                    ),
                ),
            ),
        ),
    )
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
    let (seeds, instructions) = parse_input_for_part2(params.input_data);
    let locations = get_seed_range_locations(&instructions, &seeds);
    locations.get()[0].get_min().to_string()
}
