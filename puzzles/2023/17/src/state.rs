use aoc_geometry::CardinalDirection2D;
use aoc_geometry::GridCoordinate2D;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct State {
    pub position: GridCoordinate2D,
    pub direction: CardinalDirection2D,
    pub steps: u8,
    pub heat_loss: u32,
}

// `PartialOrd` needs to be implemented to implement `Ord`.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .heat_loss
            .cmp(&self.heat_loss)
            .then_with(|| other.steps.cmp(&self.steps))
    }
}
