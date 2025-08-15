use crate::game::{Game, GameRound};
use rayon::prelude::*;

mod game;

/// Parameters for solving Part 1 of the puzzle.
pub struct Part1Parameters {
    pub input_data: &'static str,
}

fn parse_input_line(line: &str) -> Game {
    let (game_id_part, rounds_part) = line.trim().split_once(':').unwrap();
    let id = game_id_part
        .trim()
        .strip_prefix("Game ")
        .unwrap()
        .parse()
        .unwrap();
    let rounds = rounds_part
        .split(';')
        .map(|round_str| {
            let mut green = 0;
            let mut red = 0;
            let mut blue = 0;
            round_str.split(',').for_each(|color_count| {
                let mut parts = color_count.split_whitespace();
                if let (Some(num), Some(color)) = (parts.next(), parts.next()) {
                    let num = num.parse::<u32>().unwrap();
                    match color {
                        "green" => green = num,
                        "red" => red = num,
                        "blue" => blue = num,
                        _ => {}
                    }
                }
            });
            GameRound::new(green, red, blue)
        })
        .collect();
    Game::new(id, rounds)
}

fn is_game_possible(game: &Game) -> bool {
    const MAX_NUM_GREEN_BALLS: u32 = 13;
    const MAX_NUM_RED_BALLS: u32 = 12;
    const MAX_NUM_BLUE_BALLS: u32 = 14;

    game.rounds.par_iter().all(|round| {
        round.num_green_balls <= MAX_NUM_GREEN_BALLS
            && round.num_red_balls <= MAX_NUM_RED_BALLS
            && round.num_blue_balls <= MAX_NUM_BLUE_BALLS
    })
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
        .filter_map(|line| {
            let game = parse_input_line(line.trim());
            if is_game_possible(&game) {
                Some(game.id)
            } else {
                None
            }
        })
        .sum();
    value.to_string()
}

/// Parameters for solving Part 2 of the puzzle.
pub struct Part2Parameters {
    pub input_data: &'static str,
}

fn calculate_min_power(game: &Game) -> u32 {
    let mut max_green_balls: u32 = 0;
    let mut max_red_balls: u32 = 0;
    let mut max_blue_balls: u32 = 0;

    game.rounds.iter().for_each(|round| {
        max_green_balls = max_green_balls.max(round.num_green_balls);
        max_red_balls = max_red_balls.max(round.num_red_balls);
        max_blue_balls = max_blue_balls.max(round.num_blue_balls);
    });
    max_green_balls * max_red_balls * max_blue_balls
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
            let game = parse_input_line(line.trim());
            calculate_min_power(&game)
        })
        .sum();
    value.to_string()
}
