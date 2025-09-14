use crate::monkey::{Monkey, MonkeyId};
use crate::operation::Operation;
use crate::types::WorryLevel;
use std::collections::HashMap;

mod monkey;
mod operation;
mod types;

// -----------------------------------------------------------
// ------------------------ Common ---------------------------
// -----------------------------------------------------------

fn parse_operation(s: &str) -> Operation {
    if let Some(rest) = s.strip_prefix("old + ") {
        let value = rest.parse::<WorryLevel>().unwrap();
        Operation::Add(Some(value))
    } else if s == "old + old" {
        Operation::Add(None)
    } else if s == "old * old" {
        Operation::Multiply(None)
    } else if let Some(rest) = s.strip_prefix("old * ") {
        let value = rest.parse::<WorryLevel>().unwrap();
        Operation::Multiply(Some(value))
    } else {
        unreachable!()
    }
}

fn parse_monkey(input: &str) -> Monkey {
    let mut lines = input.lines().map(|l| l.trim());

    // Parse Monkey ID
    let id: MonkeyId = lines
        .next()
        .unwrap()
        .strip_prefix("Monkey ")
        .and_then(|s| s.strip_suffix(":"))
        .unwrap()
        .parse()
        .unwrap();

    // Parse Starting items
    let items_str = lines
        .next()
        .unwrap()
        .strip_prefix("Starting items: ")
        .unwrap();
    let items: Vec<WorryLevel> = items_str.split(", ").map(|s| s.parse().unwrap()).collect();

    // Parse Operation
    let op_str = lines
        .next()
        .unwrap()
        .strip_prefix("Operation: new = ")
        .unwrap();
    let operation = parse_operation(op_str);

    // Parse Divisor
    let divisor_str = lines
        .next()
        .unwrap()
        .strip_prefix("Test: divisible by ")
        .unwrap();
    let divisor: WorryLevel = divisor_str.parse().unwrap();

    // Parse target_true
    let target_true_str = lines
        .next()
        .unwrap()
        .strip_prefix("If true: throw to monkey ")
        .unwrap();
    let target_true: MonkeyId = target_true_str.parse().unwrap();

    // Parse target_false
    let target_false_str = lines
        .next()
        .unwrap()
        .strip_prefix("If false: throw to monkey ")
        .unwrap();
    let target_false: MonkeyId = target_false_str.parse().unwrap();

    Monkey::new(id, operation, divisor, target_true, target_false, items)
}

fn parse_input(input: &str) -> HashMap<MonkeyId, Monkey> {
    input
        .trim()
        .split("\n\n")
        .map(parse_monkey)
        .map(|monkey| (monkey.id(), monkey))
        .collect()
}

fn inspect_and_throw_all(
    monkeys: &mut HashMap<MonkeyId, Monkey>,
    relief_enabled: bool,
) -> HashMap<MonkeyId, u32> {
    let mut monkey_inspections: HashMap<MonkeyId, u32> =
        monkeys.keys().map(|&id| (id, 0)).collect();
    let divisor_product = if !relief_enabled {
        monkeys.values().map(|x| x.divisor()).product()
    } else {
        0
    };
    for id in 0..monkeys.len() as MonkeyId {
        let cloned_monkey = monkeys.get(&id).unwrap().clone();
        let mut inspection_count: u32 = 0;
        for item in cloned_monkey.items() {
            let mut item_value = cloned_monkey.operation().apply(*item);
            if relief_enabled {
                item_value /= 3;
            } else {
                item_value %= divisor_product;
            }
            let target = if item_value % cloned_monkey.divisor() == 0 {
                cloned_monkey.target_true()
            } else {
                cloned_monkey.target_false()
            };
            monkeys.get_mut(&target).unwrap().add_item(item_value);
            inspection_count += 1;
        }
        monkeys.get_mut(&id).unwrap().clear_items();
        *monkey_inspections.get_mut(&id).unwrap() += inspection_count;
    }
    monkey_inspections
}

// -----------------------------------------------------------
// ------------------------ Part 1 ---------------------------
// -----------------------------------------------------------

/// Parameters for solving Part 1 of the puzzle.
pub struct Part1Parameters {
    pub input_data: &'static str,
}

fn calculate_monkey_business(monkey_inspections: &HashMap<MonkeyId, u32>) -> u64 {
    let mut inspections = monkey_inspections.values().copied().collect::<Vec<_>>();
    let (result, _, _) = inspections.select_nth_unstable_by(2, |a, b| b.cmp(a)); // descending order
    result[0] as u64 * result[1] as u64
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
    const NUMBER_OF_ROUNDS: u32 = 20;
    let mut monkeys = parse_input(params.input_data);
    let mut monkey_inspections: HashMap<MonkeyId, u32> =
        monkeys.keys().map(|&id| (id, 0)).collect();

    for _round in 0..NUMBER_OF_ROUNDS {
        let inspections = inspect_and_throw_all(&mut monkeys, true);
        for (id, count) in inspections {
            *monkey_inspections.entry(id).or_insert(0) += count;
        }
    }

    calculate_monkey_business(&monkey_inspections).to_string()
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
    const NUMBER_OF_ROUNDS: u32 = 10_000;
    let mut monkeys = parse_input(params.input_data);
    let mut monkey_inspections: HashMap<MonkeyId, u32> =
        monkeys.keys().map(|&id| (id, 0)).collect();

    for _round in 0..NUMBER_OF_ROUNDS {
        let inspections = inspect_and_throw_all(&mut monkeys, false);
        for (id, count) in inspections {
            *monkey_inspections.entry(id).or_insert(0) += count;
        }
    }

    calculate_monkey_business(&monkey_inspections).to_string()
}
