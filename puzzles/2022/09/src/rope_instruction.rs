use aoc_geometry::CardinalDirection2D;

pub struct RopeInstruction {
    direction: CardinalDirection2D,
    steps: u32,
}

impl RopeInstruction {
    pub fn new(direction: CardinalDirection2D, steps: u32) -> Self {
        Self { direction, steps }
    }

    pub fn direction(&self) -> &CardinalDirection2D {
        &self.direction
    }

    pub fn steps(&self) -> u32 {
        self.steps
    }
}
