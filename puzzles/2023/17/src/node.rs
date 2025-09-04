use aoc_geometry::CardinalDirection2D;
use aoc_geometry::GridCoordinate2D;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Node {
    pub position: GridCoordinate2D,
    pub direction: CardinalDirection2D,
    pub steps: u8,
}
