use crate::part2::part_range::{PartRange, Range};
use aoc_intervals::interval::Location;

pub type GetCategoryFnType = Box<dyn Fn(&PartRange) -> &Range>;
pub type SetCategoryFnType = Box<dyn Fn(&mut PartRange, &Range)>;
pub type ConditionFnType = Box<dyn Fn(&Range) -> (Option<Range>, Option<Range>)>;

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum ActionType {
    Accepted,
    Rejected,
    GoTo(String),
}

pub struct Rule {
    get_category_fn: Option<GetCategoryFnType>,
    set_category_fn: Option<SetCategoryFnType>,
    condition_fn: Option<ConditionFnType>,
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
            let threshold_value: i32 = condition_statement[2..].parse().unwrap();
            let get_category_fn: GetCategoryFnType = match category_str {
                "x" => Box::new(|part_range: &PartRange| part_range.get_x()),
                "m" => Box::new(|part_range: &PartRange| part_range.get_m()),
                "a" => Box::new(|part_range: &PartRange| part_range.get_a()),
                "s" => Box::new(|part_range: &PartRange| part_range.get_s()),
                _ => unreachable!(),
            };
            let set_category_fn: SetCategoryFnType = match category_str {
                "x" => Box::new(|part_range: &mut PartRange, new_value: &Range| {
                    part_range.set_x(new_value)
                }),
                "m" => Box::new(|part_range: &mut PartRange, new_value: &Range| {
                    part_range.set_m(new_value)
                }),
                "a" => Box::new(|part_range: &mut PartRange, new_value: &Range| {
                    part_range.set_a(new_value)
                }),
                "s" => Box::new(|part_range: &mut PartRange, new_value: &Range| {
                    part_range.set_s(new_value)
                }),
                _ => unreachable!(),
            };
            let condition: ConditionFnType = match condition_symbol {
                "<" => Box::new(move |range| {
                    if range.has_one_value() {
                        return (None, None);
                    }
                    match range.get_location(threshold_value) {
                        Location::LeftOutside | Location::LeftBoundary => (None, Some(*range)),
                        Location::RightBoundary => (
                            Some(Range::from_boundaries(range.get_min(), threshold_value - 1)),
                            Some(Range::from_boundaries(threshold_value, threshold_value)),
                        ),
                        Location::RightOutside => (Some(*range), None),
                        Location::Within => (
                            Some(Range::from_boundaries(range.get_min(), threshold_value - 1)),
                            Some(Range::from_boundaries(threshold_value, range.get_max())),
                        ),
                    }
                }),
                ">" => Box::new(move |range| {
                    if range.has_one_value() {
                        return (None, None);
                    }
                    match range.get_location(threshold_value) {
                        Location::LeftOutside => (Some(*range), None),
                        Location::LeftBoundary => (
                            Some(Range::from_boundaries(threshold_value + 1, range.get_max())),
                            Some(Range::from_boundaries(threshold_value, threshold_value)),
                        ),
                        Location::RightBoundary | Location::RightOutside => (None, Some(*range)),
                        Location::Within => (
                            Some(Range::from_boundaries(threshold_value + 1, range.get_max())),
                            Some(Range::from_boundaries(range.get_min(), threshold_value)),
                        ),
                    }
                }),
                _ => unreachable!(),
            };
            Rule {
                get_category_fn: Some(get_category_fn),
                set_category_fn: Some(set_category_fn),
                condition_fn: Some(condition),
                action_type,
            }
        } else {
            Rule {
                get_category_fn: None,
                set_category_fn: None,
                condition_fn: None,
                action_type,
            }
        }
    }

    pub fn has_condition(&self) -> bool {
        self.get_category_fn.is_some()
            && self.set_category_fn.is_some()
            && self.condition_fn.is_some()
    }

    pub fn get_action(&self) -> &ActionType {
        &self.action_type
    }

    pub fn process(&self, part: &PartRange) -> (Option<PartRange>, Option<PartRange>) {
        if self.condition_fn.is_none() {
            match self.action_type {
                ActionType::Accepted => (Some(part.clone()), None),
                ActionType::Rejected => (None, Some(part.clone())),
                ActionType::GoTo(_) => (Some(part.clone()), Some(part.clone())),
            }
        } else {
            let condition_result =
                self.condition_fn.as_ref().unwrap()(self.get_category_fn.as_ref().unwrap()(part));
            let mut accepted = part.clone();
            self.set_category_fn.as_ref().unwrap()(&mut accepted, &condition_result.0.unwrap());
            let mut rejected = part.clone();
            self.set_category_fn.as_ref().unwrap()(&mut rejected, &condition_result.1.unwrap());
            (Some(accepted), Some(rejected))
        }
    }
}
