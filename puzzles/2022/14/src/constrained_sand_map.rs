use aoc_geometry::{AxisDirection, BoundingBox, OrthogonalLine, Point, Vector};
use std::collections::HashSet;

pub struct ConstrainedSandMap {
    rocks: HashSet<Point<i32, 2>>,
    bounding_box: BoundingBox<i32, 2>,
    sand: HashSet<Point<i32, 2>>,
}

impl ConstrainedSandMap {
    pub fn new(rock_vertices: &Vec<Vec<Point<i32, 2>>>) -> Self {
        let mut rocks = HashSet::new();
        let mut bounding_box = BoundingBox::new();

        for vertex_group in rock_vertices {
            bounding_box.update(&vertex_group[0]);
            for window in vertex_group.windows(2) {
                let line = OrthogonalLine::from_points(&window[0], &window[1]);
                for point in line.iter() {
                    rocks.insert(point);
                }
                bounding_box.update(&window[1]);
            }
        }

        ConstrainedSandMap {
            rocks,
            bounding_box,
            sand: HashSet::new(),
        }
    }

    fn is_sand_outside(&self, position: &Point<i32, 2>) -> bool {
        self.bounding_box
            .is_outside_for_axis_and_direction(0, AxisDirection::Positive, position)
            || self.bounding_box.is_outside_for_axis_and_direction(
                0,
                AxisDirection::Negative,
                position,
            )
            || self.bounding_box.is_outside_for_axis_and_direction(
                1,
                AxisDirection::Positive,
                position,
            )
    }

    fn move_sand(
        &mut self,
        position: &Point<i32, 2>,
        movement: Vector<i32, 2>,
    ) -> Option<Point<i32, 2>> {
        assert!(movement.is_normalized(), "Movement must be normalized");
        let next_position = position.move_by(&movement).unwrap();
        if self.rocks.contains(&next_position) || self.sand.contains(&next_position) {
            None
        } else {
            Some(next_position)
        }
    }

    /// Drops a unit of sand from the source (500, 0) until it comes to rest or falls out of bounds.
    ///
    /// # Return
    ///
    /// `true` if the sand comes to rest within bounds, `false` if it falls out of bounds.
    pub fn drop_sand(&mut self) -> bool {
        let mut current_position = Point::new([500, 0]);
        loop {
            if let Some(next_position) = self.move_sand(&current_position, Vector::new([0, 1])) {
                if self.is_sand_outside(&current_position) {
                    return false;
                }
                current_position = next_position;
                continue;
            }
            if let Some(next_position) = self.move_sand(&current_position, Vector::new([-1, 1])) {
                if self.is_sand_outside(&current_position) {
                    return false;
                }
                current_position = next_position;
                continue;
            }
            if let Some(next_position) = self.move_sand(&current_position, Vector::new([1, 1])) {
                if self.is_sand_outside(&current_position) {
                    return false;
                }
                current_position = next_position;
                continue;
            }
            // Sand cannot move anymore
            self.sand.insert(current_position);
            return true;
        }
    }

    pub fn get_number_of_sand_units(&self) -> u32 {
        self.sand.len() as u32
    }
}
