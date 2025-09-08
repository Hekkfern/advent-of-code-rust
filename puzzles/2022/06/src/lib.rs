mod char_sliding_window;

use char_sliding_window::CharSlidingWindow;

fn solve<const W_SIZE: usize>(input: &str) -> String {
    let mut window = CharSlidingWindow::<W_SIZE>::new();
    let mut char_counter: u32 = 0;
    for c in input.trim().chars() {
        char_counter += 1;
        window.add_char(c);
        if window.are_unique() {
            return char_counter.to_string();
        }
    }
    unreachable!();
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
    const WINDOW_SIZE: usize = 4;
    solve::<WINDOW_SIZE>(params.input_data)
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
    const WINDOW_SIZE: usize = 14;
    solve::<WINDOW_SIZE>(params.input_data)
}
