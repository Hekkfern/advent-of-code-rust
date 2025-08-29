#[derive(Eq, PartialEq, Hash, Debug)]
pub struct History {
    sequence: Vec<i64>,
}

impl History {
    pub fn new(seq: Vec<i64>) -> Self {
        Self { sequence: seq }
    }

    pub fn extrapolate_right(&self) -> i64 {
        let next_level = Self::extrapolate_right_recursive(&self.sequence)
            .expect("Should always have a next level");
        self.sequence.last().unwrap() + next_level
    }

    pub fn extrapolate_left(&self) -> i64 {
        let next_level = Self::extrapolate_left_recursive(&self.sequence)
            .expect("Should always have a next level");
        self.sequence.first().unwrap() - next_level
    }

    // Private helper: calculate differences between consecutive elements
    fn calculate_differences(input: &[i64]) -> Vec<i64> {
        if input.len() < 2 {
            return Vec::new();
        }
        input.windows(2).map(|w| w[1] - w[0]).collect()
    }

    // Private helper: check if all elements are zero
    fn are_all_zeros(input: &[i64]) -> bool {
        input.iter().all(|&item| item == 0)
    }

    // Private helper: recursive extrapolation to the right
    fn extrapolate_right_recursive(input: &[i64]) -> Option<i64> {
        let diffs = Self::calculate_differences(input);
        if Self::are_all_zeros(&diffs) {
            return None;
        }
        match Self::extrapolate_right_recursive(&diffs) {
            Some(next_level) => diffs.last().unwrap() + next_level,
            None => diffs[0],
        }
        .into()
    }

    // Private helper: recursive extrapolation to the left
    fn extrapolate_left_recursive(input: &[i64]) -> Option<i64> {
        let diffs = Self::calculate_differences(input);
        if Self::are_all_zeros(&diffs) {
            return None;
        }
        match Self::extrapolate_left_recursive(&diffs) {
            Some(next_level) => diffs.first().unwrap() - next_level,
            None => diffs[0],
        }
        .into()
    }
}
