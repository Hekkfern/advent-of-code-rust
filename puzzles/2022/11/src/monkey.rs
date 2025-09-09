use crate::operation::Operation;
use crate::types::WorryLevel;

pub type MonkeyId = u8;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Monkey {
    id: MonkeyId,
    operation: Operation,
    divisor: WorryLevel,
    target_true: MonkeyId,
    target_false: MonkeyId,
    items: Vec<WorryLevel>,
}

impl Monkey {
    pub fn new(
        id: MonkeyId,
        operation: Operation,
        divisor: WorryLevel,
        target_true: MonkeyId,
        target_false: MonkeyId,
        items: Vec<WorryLevel>,
    ) -> Self {
        Self {
            id,
            operation,
            divisor,
            target_true,
            target_false,
            items,
        }
    }

    pub fn id(&self) -> MonkeyId {
        self.id
    }

    pub fn operation(&self) -> &Operation {
        &self.operation
    }

    pub fn divisor(&self) -> WorryLevel {
        self.divisor
    }

    pub fn target_true(&self) -> MonkeyId {
        self.target_true
    }

    pub fn target_false(&self) -> MonkeyId {
        self.target_false
    }

    pub fn items(&self) -> &Vec<WorryLevel> {
        &self.items
    }

    pub fn add_item(&mut self, item: WorryLevel) {
        self.items.push(item);
    }

    pub fn clear_items(&mut self) {
        self.items.clear();
    }
}
