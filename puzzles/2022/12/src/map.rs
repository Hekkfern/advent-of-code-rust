use crate::position::Position;
use crate::types::Height;
use aoc_geometry::Grid2D;

pub struct Map {
    grid: Grid2D<Height>,
    origin: Position,
    destination: Position,
}

impl Map {
    pub fn new(grid: Grid2D<Height>, origin: Position, destination: Position) -> Map {
        Map {
            grid,
            origin,
            destination,
        }
    }

    pub fn get_grid(&self) -> &Grid2D<Height> {
        &self.grid
    }

    pub fn get_origin(&self) -> &Position {
        &self.origin
    }

    pub fn get_destination(&self) -> &Position {
        &self.destination
    }
}
