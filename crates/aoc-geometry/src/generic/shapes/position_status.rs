/// Classification of position status within a grid or bounded area.
///
/// This enum categorizes positions based on their relationship to the boundaries
/// of a grid or other bounded geometric structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionStatus {
    /// Position is inside the boundaries (not touching any border).
    Inside,
    /// Position is outside the boundaries.
    Outside,
    /// Position is on the border of the boundaries (touching at least one border).
    OnBorder,
}
