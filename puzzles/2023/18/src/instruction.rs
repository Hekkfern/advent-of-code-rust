use aoc_geometry::CardinalDirection2D;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Instruction {
    direction: CardinalDirection2D,
    steps: u64,
}

impl Instruction {
    pub fn new(direction: CardinalDirection2D, steps: u64) -> Self {
        Self { direction, steps }
    }

    pub fn direction(&self) -> &CardinalDirection2D {
        &self.direction
    }

    pub fn steps(&self) -> u64 {
        self.steps
    }
}
