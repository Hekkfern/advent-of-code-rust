mod crane_instruction;
mod types;

use crate::crane_instruction::CraneInstruction;
use crate::types::{CrateStack, StackId};

fn parse_input_stacks(input: &str) -> Vec<CrateStack> {
    let lines: Vec<&str> = input.lines().collect();
    // The last line contains stack numbers
    let stack_count = lines.last().unwrap().split_whitespace().count();
    let mut stacks: Vec<CrateStack> = vec![Vec::new(); stack_count];
    // Iterate over crate lines (all except last), from top to bottom
    for line in lines.iter().take(lines.len() - 1) {
        // Each stack is at position 1 + 4*i (i = stack index)
        for (i, stack) in stacks.iter_mut().enumerate() {
            let pos = 1 + i * 4;
            if let Some(c) = line.chars().nth(pos)
                && c.is_ascii_alphabetic()
            {
                stack.push(c);
            }
        }
    }
    // Reverse each stack so index 0 is the bottom
    for stack in &mut stacks {
        stack.reverse();
    }
    stacks
}

fn parse_input_instruction(line: &str) -> CraneInstruction {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let num_crates = parts[1].parse::<u32>().unwrap();
    assert!(num_crates > 0);
    let origin = parts[3].parse::<StackId>().unwrap() - 1; // Convert to 0-based index
    let destination = parts[5].parse::<StackId>().unwrap() - 1; // Convert to 0-based index
    assert_ne!(origin, destination);
    CraneInstruction::new(num_crates, origin, destination)
}

fn generate_result(stacks: Vec<CrateStack>) -> String {
    stacks
        .iter()
        .filter(|stack| !stack.is_empty())
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

fn get_stack_crates(
    stacks: &mut [CrateStack],
    origin_idx: usize,
    destination_idx: usize,
) -> (&mut CrateStack, &mut CrateStack) {
    let (first, second) = if origin_idx < destination_idx {
        stacks.split_at_mut(destination_idx)
    } else {
        stacks.split_at_mut(origin_idx)
    };

    if origin_idx < destination_idx {
        (&mut first[origin_idx], &mut second[0])
    } else {
        (&mut second[0], &mut first[destination_idx])
    }
}

/// Parameters for solving Part 1 of the puzzle.
pub struct Part1Parameters {
    pub input_data: &'static str,
}

fn execute_cratemover9000_instruction(stacks: &mut [CrateStack], instruction: CraneInstruction) {
    let (origin_stack, destination_stack) = get_stack_crates(
        stacks,
        instruction.origin() as usize,
        instruction.destination() as usize,
    );
    destination_stack.extend(
        origin_stack
            .drain(origin_stack.len() - instruction.num_crates() as usize..)
            .rev(),
    );
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
    let parts: Vec<&str> = params.input_data.trim_end().split("\n\n").collect();
    let mut crate_stacks = parse_input_stacks(parts[0]);
    parts[1].lines().for_each(|line| {
        let instruction = parse_input_instruction(line);
        execute_cratemover9000_instruction(&mut crate_stacks, instruction);
    });
    generate_result(crate_stacks)
}

/// Parameters for solving Part 2 of the puzzle.
pub struct Part2Parameters {
    pub input_data: &'static str,
}

fn execute_cratemover9001_instruction(stacks: &mut [CrateStack], instruction: CraneInstruction) {
    let (origin_stack, destination_stack) = get_stack_crates(
        stacks,
        instruction.origin() as usize,
        instruction.destination() as usize,
    );
    destination_stack
        .extend(origin_stack.drain(origin_stack.len() - instruction.num_crates() as usize..));
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
    let parts: Vec<&str> = params.input_data.trim_end().split("\n\n").collect();
    let mut crate_stacks = parse_input_stacks(parts[0]);
    parts[1].lines().for_each(|line| {
        let instruction = parse_input_instruction(line);
        execute_cratemover9001_instruction(&mut crate_stacks, instruction);
    });
    generate_result(crate_stacks)
}
