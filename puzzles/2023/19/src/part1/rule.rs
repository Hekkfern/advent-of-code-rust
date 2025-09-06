use crate::part1::part::Part;

#[derive(Eq, PartialEq, Hash, Debug)]
enum ActionType {
    Accepted,
    Rejected,
    GoTo(String),
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum RunResult {
    Accepted,
    Rejected,
    GoTo(String),
    Skip,
}

pub struct Rule {
    get_category_fn: Box<dyn Fn(&Part) -> u32>,
    condition_fn: Box<dyn Fn(u32) -> bool>,
    action_type: ActionType,
}

impl Rule {
    pub fn new(condition_statement: String, action_statement: String) -> Self {
        let action_type = match &action_statement[..] {
            "A" => ActionType::Accepted,
            "R" => ActionType::Rejected,
            _ => ActionType::GoTo(action_statement),
        };
        if !condition_statement.is_empty() {
            let category_str = &condition_statement[0..1];
            let condition_symbol = &condition_statement[1..2];
            let threshold_value: u32 = condition_statement[2..].parse().unwrap();
            let category: Box<dyn Fn(&Part) -> u32> = match category_str {
                "x" => Box::new(|part: &Part| part.x()),
                "m" => Box::new(|part: &Part| part.m()),
                "a" => Box::new(|part: &Part| part.a()),
                "s" => Box::new(|part: &Part| part.s()),
                _ => unreachable!(),
            };
            let condition: Box<dyn Fn(u32) -> bool> = match condition_symbol {
                ">" => Box::new(move |value| value > threshold_value),
                "<" => Box::new(move |value| value < threshold_value),
                _ => unreachable!(),
            };
            Rule {
                get_category_fn: category,
                condition_fn: condition,
                action_type,
            }
        } else {
            Rule {
                get_category_fn: Box::new(|_| 0),
                condition_fn: Box::new(|_| true),
                action_type,
            }
        }
    }

    pub fn run(&self, part: &Part) -> RunResult {
        if (self.condition_fn)((self.get_category_fn)(part)) {
            match &self.action_type {
                ActionType::Accepted => RunResult::Accepted,
                ActionType::Rejected => RunResult::Rejected,
                ActionType::GoTo(destination) => RunResult::GoTo(destination.clone()),
            }
        } else {
            RunResult::Skip
        }
    }
}
