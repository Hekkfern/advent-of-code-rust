mod instruction;
mod map;
mod network_node;

use crate::instruction::Instruction;
use crate::map::Map;
use crate::network_node::NetworkNode;

fn parse_input(input: &str) -> Map {
    let mut sections = input.trim().split("\n\n");

    let instructions_section = sections.next().unwrap();
    let nodes_section = sections.next().unwrap();

    let instructions: Vec<Instruction> = instructions_section
        .trim()
        .chars()
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => unreachable!(),
        })
        .collect();

    let mut map = Map::new();
    map.add_instructions(instructions);

    for line in nodes_section.lines() {
        let parts: Vec<&str> = line.trim().split("=").map(|s| s.trim()).collect();
        let node_id = parts[0].to_string();
        let clean_part1 = parts[1].replace(&['(', ')'][..], "");
        let next_nodes: Vec<&str> = clean_part1.split(',').map(|s| s.trim()).collect();
        let left_node = next_nodes[0].to_string();
        let right_node = next_nodes[1].to_string();

        let node = NetworkNode::new(left_node, right_node);
        map.add_node(node_id, node);
    }

    map
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
    let map = parse_input(params.input_data);
    let steps = map.navigate_from_aaa_to_zzz();
    steps.to_string()
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
    let map = parse_input(params.input_data);
    let steps = map.navigate_from_all_xxa_to_all_xxz();
    steps.to_string()
}
