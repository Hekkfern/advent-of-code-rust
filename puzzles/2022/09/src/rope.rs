use aoc_geometry::{CardinalDirection2D, Point, Vector};

pub type Coordinate = Point<i32, 2>;

pub struct Rope<const ROPE_LENGTH: usize> {
    knots: [Coordinate; ROPE_LENGTH],
}

impl<const ROPE_LENGTH: usize> Rope<ROPE_LENGTH> {
    pub fn new() -> Self {
        Self {
            knots: [Coordinate::origin(); ROPE_LENGTH],
        }
    }

    pub fn tail(&self) -> &Coordinate {
        self.knots.last().unwrap()
    }

    fn update_tails(&mut self) {
        for idx in 1..self.knots.len() {
            let v = Vector::<i32, 2>::from_points(&self.knots[idx], &self.knots[idx - 1]).unwrap();
            if v.max_coordinate() <= 1 {
                return;
            }
            let desired_movement = v.normalize();
            let new_coord = self.knots[idx].move_by(&desired_movement).unwrap();
            self.knots[idx] = new_coord;
        }
    }

    pub fn move_head(&mut self, direction: CardinalDirection2D) {
        self.knots[0] = self.knots[0]
            .move_by(&direction.to_vector::<i32>())
            .unwrap();
        self.update_tails();
    }
}
