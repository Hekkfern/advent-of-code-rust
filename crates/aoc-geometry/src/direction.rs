/// Represents a direction for axis-based operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Positive direction along an axis.
    Positive,
    /// Negative direction along an axis.
    Negative,
}

impl Direction {
    /// Returns the multiplier associated with the direction.
    ///
    /// # Returns
    ///
    /// * `1` for `Positive`
    /// * `-1` for `Negative`
    pub fn multiplier(&self) -> i8 {
        match self {
            Direction::Positive => 1,
            Direction::Negative => -1,
        }
    }
}
