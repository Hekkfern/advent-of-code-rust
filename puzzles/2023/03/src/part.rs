use aoc_geometry::Point;

#[derive(Eq, PartialEq, Debug)]
pub struct Part {
    part_number: u32,
    vertexes: [Point<i32, 2>; 2],
}

impl Part {
    pub fn new(part_number: u32, vertexes: [Point<i32, 2>; 2]) -> Self {
        Self {
            part_number,
            vertexes,
        }
    }

    pub fn number(&self) -> u32 {
        self.part_number
    }

    pub fn vertexes(&self) -> &[Point<i32, 2>; 2] {
        &self.vertexes
    }
}
