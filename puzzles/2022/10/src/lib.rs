mod instruction_type;

use instruction_type::InstructionType;

// -----------------------------------------------------------
// ------------------------ Common ---------------------------
// -----------------------------------------------------------

const STARTING_REGISTER_VALUE: i32 = 1;

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

const LIT_CRT_CHARACTER: char = '#';
const DARK_CRT_CHARACTER: char = '.';
const CRT_ROW_LENGTH: usize = 40;

type CrtScreen = Vec<Vec<char>>;

fn draw(crt_screen: &mut CrtScreen, row_idx: u8, crt_pixel_position: u32, register_value: u32) {
    let is_lit =
        crt_pixel_position >= (register_value - 1) && crt_pixel_position <= (register_value + 1);
    crt_screen[row_idx as usize].push(if is_lit {
        LIT_CRT_CHARACTER
    } else {
        DARK_CRT_CHARACTER
    });
}

fn move_crt_pointer(
    crt_screen: &mut CrtScreen,
    crt_pixel_position: &mut u32,
    crt_screen_row_idx: &mut u8,
) {
    *crt_pixel_position += 1;
    if *crt_pixel_position >= CRT_ROW_LENGTH as u32 {
        *crt_pixel_position = 0;
        *crt_screen_row_idx += 1;
        crt_screen.push(vec![]);
    }
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
    let mut register_value: i32 = STARTING_REGISTER_VALUE;
    let mut crt_pixel_position: u32 = 0;
    let mut crt_screen: CrtScreen = vec![vec![]];
    let mut crt_screen_row_index: u8 = 0;
    params.input_data.trim().lines().for_each(|line| {
        let instruction = InstructionType::from(line.trim());
        match instruction {
            InstructionType::Noop => {
                draw(
                    &mut crt_screen,
                    crt_screen_row_index,
                    crt_pixel_position,
                    register_value as u32,
                );
                move_crt_pointer(
                    &mut crt_screen,
                    &mut crt_pixel_position,
                    &mut crt_screen_row_index,
                );
            }
            InstructionType::Addx(value) => {
                for _ in 0..cycles_per_instruction(&instruction) {
                    draw(
                        &mut crt_screen,
                        crt_screen_row_index,
                        crt_pixel_position,
                        register_value as u32,
                    );
                    move_crt_pointer(
                        &mut crt_screen,
                        &mut crt_pixel_position,
                        &mut crt_screen_row_index,
                    );
                }
                register_value += value;
            }
        }
    });
    crt_screen.pop();
    let result = crt_screen
        .iter()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
    result
}
