use crate::types::WorryLevel;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Operation {
    Add(Option<WorryLevel>),
    Multiply(Option<WorryLevel>),
}

impl Operation {
    pub fn apply(&self, old: WorryLevel) -> WorryLevel {
        match self {
            Operation::Add(Some(v)) => old + v,
            Operation::Add(None) => old + old,
            Operation::Multiply(Some(v)) => old * v,
            Operation::Multiply(None) => old * old,
        }
    }
}
