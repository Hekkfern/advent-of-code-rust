use aoc_intervals::interval::Interval;

pub type Range = Interval<i32>;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct PartRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl PartRange {
    pub fn new() -> Self {
        Self {
            x: Range::from_boundaries(1, 4000),
            m: Range::from_boundaries(1, 4000),
            a: Range::from_boundaries(1, 4000),
            s: Range::from_boundaries(1, 4000),
        }
    }

    pub fn get_x(&self) -> &Range {
        &self.x
    }

    pub fn set_x(&mut self, new_value: &Range) {
        self.x = new_value.clone();
    }

    pub fn get_m(&self) -> &Range {
        &self.m
    }

    pub fn set_m(&mut self, new_value: &Range) {
        self.m = new_value.clone();
    }

    pub fn get_a(&self) -> &Range {
        &self.a
    }

    pub fn set_a(&mut self, new_value: &Range) {
        self.a = new_value.clone();
    }

    pub fn get_s(&self) -> &Range {
        &self.s
    }

    pub fn set_s(&mut self, new_value: &Range) {
        self.s = new_value.clone();
    }

    pub fn calculate_number_of_combinations(&self) -> u64 {
        self.x.count() * self.m.count() * self.a.count() * self.s.count()
    }
}
