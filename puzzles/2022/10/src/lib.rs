mod instruction_type;

use instruction_type::InstructionType;

// -----------------------------------------------------------
// ------------------------ Common ---------------------------
// -----------------------------------------------------------

fn cycles_per_instruction(instruction: &InstructionType) -> u32 {
    match instruction {
        InstructionType::Noop => 1,
        InstructionType::Addx(_) => 2,
    }
}

// -----------------------------------------------------------
// ------------------------ Part 1 ---------------------------
// -----------------------------------------------------------

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
    const STARTING_REGISTER_VALUE: i32 = 1;
    const INITIAL_CHECKPOINT_CYCLE: u32 = 20;
    const CHECKPOINT_CYCLE_INTERVAL: u32 = 40;
    let mut cycle_counter: u32 = 1;
    let mut cycle_strength: u32 = 0;
    let mut register_value = STARTING_REGISTER_VALUE;
    let mut next_checkpoint_cycle = INITIAL_CHECKPOINT_CYCLE;
    params.input_data.trim().lines().for_each(|line| {
        let instruction = InstructionType::from(line.trim());
        if (matches!(instruction, InstructionType::Addx(_))
            && cycle_counter <= next_checkpoint_cycle
            && (cycle_counter + cycles_per_instruction(&instruction)) > next_checkpoint_cycle)
            || (matches!(instruction, InstructionType::Noop)
                && cycle_counter == next_checkpoint_cycle)
        {
            cycle_strength += next_checkpoint_cycle * (register_value as u32);
            next_checkpoint_cycle += CHECKPOINT_CYCLE_INTERVAL;
        }
        if let InstructionType::Addx(value) = instruction {
            register_value += value;
        }
        cycle_counter += cycles_per_instruction(&instruction);
    });
    cycle_strength.to_string()
}

// -----------------------------------------------------------
// ------------------------ Part 2 ---------------------------
// -----------------------------------------------------------

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
    const LIT_CRT_CHARACTER: char = '#';
    const DARK_CRT_CHARACTER: char = '.';
    const CRT_ROW_LENGTH: usize = 40;
    // TODO
    String::from("")
}
