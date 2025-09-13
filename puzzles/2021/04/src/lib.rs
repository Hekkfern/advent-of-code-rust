mod bingo_board;
// -----------------------------------------------------------
// ------------------------ Common ---------------------------
// -----------------------------------------------------------

fn parse_input(input: &str) -> (Vec<u8>, Vec<bingo_board::BingoBoard>) {
    let mut sections = input.trim().split("\n\n");
    let numbers: Vec<u8> = sections
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect();
    let boards: Vec<bingo_board::BingoBoard> = sections
        .map(|section| {
            let nums: Vec<u8> = section
                .lines()
                .flat_map(|line| {
                    line.split_whitespace()
                        .map(|s| s.parse::<u8>().unwrap())
                        .collect::<Vec<u8>>()
                })
                .collect();
            bingo_board::BingoBoard::new(nums)
        })
        .collect();
    (numbers, boards)
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
    let (numbers, mut boards) = parse_input(params.input_data);
    for &number in &numbers {
        for board in boards.iter_mut() {
            board.mark_number(number);
            if board.has_bingo() {
                let unmarked_sum = board.unmarked_sum();
                let result = unmarked_sum * number as u32;
                return result.to_string();
            }
        }
    }
    unreachable!()
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
    // TODO
    String::from("")
}
