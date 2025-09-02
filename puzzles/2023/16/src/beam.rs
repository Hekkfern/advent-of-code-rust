use aoc_geometry::CardinalDirection2D;
use aoc_geometry::GridCoordinate2D;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct Beam {
    coordinates: GridCoordinate2D,
    direction: CardinalDirection2D,
}

impl Beam {
    pub fn new(coordinates: GridCoordinate2D, direction: CardinalDirection2D) -> Self {
        Self {
            coordinates,
            direction,
        }
    }

    pub fn get_coordinates(&self) -> &GridCoordinate2D {
        &self.coordinates
    }

    pub fn get_direction(&self) -> &CardinalDirection2D {
        &self.direction
    }
}
