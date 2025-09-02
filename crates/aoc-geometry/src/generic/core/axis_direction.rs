/// Represents a direction for axis-based operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AxisDirection {
    /// Positive direction along an axis.
    Positive,
    /// Negative direction along an axis.
    Negative,
}

impl AxisDirection {
    /// Returns the multiplier associated with the direction.
    ///
    /// # Returns
    ///
    /// * `1` for `Positive`
    /// * `-1` for `Negative`
    pub fn multiplier(&self) -> i8 {
        match self {
            AxisDirection::Positive => 1,
            AxisDirection::Negative => -1,
        }
    }
}
