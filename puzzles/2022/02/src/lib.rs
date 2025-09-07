use crate::match_result::MatchResult;
use crate::opponent_choice::OpponentChoice;
use crate::player_choice::PlayerChoice;

mod match_result;
mod opponent_choice;
mod player_choice;

const POINTS_PER_USED_ROCK: u32 = 1;
const POINTS_PER_USED_PAPER: u32 = 2;
const POINTS_PER_USED_SCISSORS: u32 = 3;
const POINTS_PER_LOST: u32 = 0;
const POINTS_PER_DRAW: u32 = 3;
const POINTS_PER_WIN: u32 = 6;

/// Parameters for solving Part 1 of the puzzle.
pub struct Part1Parameters {
    pub input_data: &'static str,
}

fn get_points_per_shapes(opponent_choice: OpponentChoice, player_choice: PlayerChoice) -> u32 {
    match (opponent_choice, player_choice) {
        (OpponentChoice::Rock, PlayerChoice::Rock) => POINTS_PER_USED_ROCK + POINTS_PER_DRAW,
        (OpponentChoice::Rock, PlayerChoice::Paper) => POINTS_PER_USED_PAPER + POINTS_PER_WIN,
        (OpponentChoice::Rock, PlayerChoice::Scissors) => {
            POINTS_PER_USED_SCISSORS + POINTS_PER_LOST
        }
        (OpponentChoice::Paper, PlayerChoice::Rock) => POINTS_PER_USED_ROCK + POINTS_PER_LOST,
        (OpponentChoice::Paper, PlayerChoice::Paper) => POINTS_PER_USED_PAPER + POINTS_PER_DRAW,
        (OpponentChoice::Paper, PlayerChoice::Scissors) => {
            POINTS_PER_USED_SCISSORS + POINTS_PER_WIN
        }
        (OpponentChoice::Scissors, PlayerChoice::Rock) => POINTS_PER_USED_ROCK + POINTS_PER_WIN,
        (OpponentChoice::Scissors, PlayerChoice::Paper) => POINTS_PER_USED_PAPER + POINTS_PER_LOST,
        (OpponentChoice::Scissors, PlayerChoice::Scissors) => {
            POINTS_PER_USED_SCISSORS + POINTS_PER_DRAW
        }
    }
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
    let value: u32 = params
        .input_data
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let opponent_choice = OpponentChoice::from(parts.next().unwrap());
            let player_choice = PlayerChoice::from(parts.next().unwrap());
            (opponent_choice, player_choice)
        })
        .map(|(opponent_choice, player_choice)| {
            get_points_per_shapes(opponent_choice, player_choice)
        })
        .sum();
    value.to_string()
}

/// Parameters for solving Part 2 of the puzzle.
pub struct Part2Parameters {
    pub input_data: &'static str,
}

///  Look-up table which links the opponent shape and the result of the match, to the shape that
/// the player must use to reach the said result.
fn convert_result(opponent_choice: OpponentChoice, match_result: MatchResult) -> PlayerChoice {
    match (opponent_choice, match_result) {
        (OpponentChoice::Rock, MatchResult::Lost) => PlayerChoice::Scissors,
        (OpponentChoice::Rock, MatchResult::Draw) => PlayerChoice::Rock,
        (OpponentChoice::Rock, MatchResult::Win) => PlayerChoice::Paper,
        (OpponentChoice::Paper, MatchResult::Lost) => PlayerChoice::Rock,
        (OpponentChoice::Paper, MatchResult::Draw) => PlayerChoice::Paper,
        (OpponentChoice::Paper, MatchResult::Win) => PlayerChoice::Scissors,
        (OpponentChoice::Scissors, MatchResult::Lost) => PlayerChoice::Paper,
        (OpponentChoice::Scissors, MatchResult::Draw) => PlayerChoice::Scissors,
        (OpponentChoice::Scissors, MatchResult::Win) => PlayerChoice::Rock,
    }
}

fn calculate_round_score_from_result(
    opponent_choice: OpponentChoice,
    match_result: MatchResult,
) -> u32 {
    let player_choice = convert_result(opponent_choice.clone(), match_result);
    get_points_per_shapes(opponent_choice, player_choice)
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
    let value: u32 = params
        .input_data
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let opponent_choice = OpponentChoice::from(parts.next().unwrap());
            let match_result = MatchResult::from(parts.next().unwrap());
            (opponent_choice, match_result)
        })
        .map(|(opponent_choice, match_result)| {
            calculate_round_score_from_result(opponent_choice, match_result)
        })
        .sum();
    value.to_string()
}
