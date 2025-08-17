use aoc_geometry::point::Point;

#[derive(Eq, PartialEq, Debug)]
pub struct Symbol {
    character: char,
    position: Point<i32, 2>,
}

impl Symbol {
    pub fn new(character: char, position: Point<i32, 2>) -> Self {
        Self {
            character,
            position,
        }
    }

    pub fn character(&self) -> char {
        self.character
    }

    pub fn position(&self) -> &Point<i32, 2> {
        &self.position
    }
}
