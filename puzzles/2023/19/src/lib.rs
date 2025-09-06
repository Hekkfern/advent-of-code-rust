mod part1;
mod part2;

/// Parameters for solving Part 1 of the puzzle.
pub struct Part1Parameters {
    pub input_data: &'static str,
}

fn parse_workflow_for_part1(line: &str) -> part1::workflow::Workflow {
    /* extract name */
    let name_end = line.find('{').unwrap();
    let name = &line[0..name_end];
    let mut workflow = part1::workflow::Workflow::new(name);
    /* extract rules */
    let rules_str: Vec<_> = line[name_end + 1..line.len() - 1].split(",").collect();
    for rule_str in rules_str {
        let rule_str_parts: Vec<_> = rule_str.split(":").collect();
        if rule_str_parts.len() < 2 {
            // it is the unconditional rule at the end of the workflow
            workflow.add_rule(part1::rule::Rule::new(
                String::new(),
                rule_str_parts[0].into(),
            ));
        } else {
            workflow.add_rule(part1::rule::Rule::new(
                rule_str_parts[0].into(),
                rule_str_parts[1].into(),
            ));
        }
    }
    workflow
}

fn parse_part_for_part1(line: &str) -> part1::part::Part {
    let line = &line[1..line.len() - 1];
    let nums: Vec<u32> = line
        .split(',')
        .map(|s| {
            let value_str = &s[2..];
            value_str.parse().unwrap()
        })
        .collect();
    assert_eq!(nums.len(), 4);
    part1::part::Part::new(nums[0], nums[1], nums[2], nums[3])
}

fn parse_system_for_part1(input_data: &str) -> part1::system::System {
    let mut system = part1::system::System::new();
    for line in input_data.lines() {
        system.add_workflow(parse_workflow_for_part1(line.trim()));
    }
    system
}

fn parse_parts_for_part1(input_data: &str) -> Vec<part1::part::Part> {
    let mut parts = Vec::new();
    for line in input_data.lines() {
        parts.push(parse_part_for_part1(line.trim()));
    }
    parts
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
    let input_data_parts: Vec<_> = params.input_data.split("\n\n").collect();
    let system = parse_system_for_part1(input_data_parts[0]);
    let parts = parse_parts_for_part1(input_data_parts[1]);
    let part_sum: u32 = parts
        .iter()
        .filter(|part| matches!(system.run(part), part1::system::RunResult::Accepted))
        .map(|part| part.sum())
        .sum();
    part_sum.to_string()
}

/// Parameters for solving Part 2 of the puzzle.
pub struct Part2Parameters {
    pub input_data: &'static str,
}

fn parse_workflow_for_part2(line: &str) -> part2::workflow::Workflow {
    /* extract name */
    let name_end = line.find('{').unwrap();
    let name = &line[0..name_end];
    let mut workflow = part2::workflow::Workflow::new(name);
    /* extract rules */
    let rules_str: Vec<_> = line[name_end + 1..line.len() - 1].split(",").collect();
    for rule_str in rules_str {
        let rule_str_parts: Vec<_> = rule_str.split(":").collect();
        if rule_str_parts.len() < 2 {
            // it is the unconditional rule at the end of the workflow
            workflow.add_rule(part2::rule::Rule::new(
                String::new(),
                rule_str_parts[0].into(),
            ));
        } else {
            workflow.add_rule(part2::rule::Rule::new(
                rule_str_parts[0].into(),
                rule_str_parts[1].into(),
            ));
        }
    }
    workflow
}

fn parse_system_for_part2(input_data: &str) -> part2::system::System {
    let mut system = part2::system::System::new();
    for line in input_data.lines() {
        system.add_workflow(parse_workflow_for_part2(line.trim()));
    }
    system
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
    let input_data_parts: Vec<_> = params.input_data.split("\n\n").collect();
    let system = parse_system_for_part2(input_data_parts[0]);
    let num_combinations: u64 = system
        .search()
        .iter()
        .map(|part_range| part_range.calculate_number_of_combinations())
        .sum();
    num_combinations.to_string()
}
