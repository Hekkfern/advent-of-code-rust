use crate::types::Height;
use aoc_geometry::GridCoordinate2D;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub struct Position {
    point: GridCoordinate2D,
    height: Height,
}

impl Position {
    pub fn new(point: GridCoordinate2D, height: Height) -> Position {
        Position { point, height }
    }

    pub fn default() -> Position {
        Position {
            point: GridCoordinate2D::new([0, 0]),
            height: 0,
        }
    }

    pub fn get_point(&self) -> &GridCoordinate2D {
        &self.point
    }

    pub fn get_height(&self) -> Height {
        self.height
    }
}
