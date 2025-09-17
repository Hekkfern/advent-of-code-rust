use aoc_geometry::{OrthogonalLine2D, Point, Vector};
use std::collections::HashSet;

pub struct InfiniteSandMap {
    rocks: HashSet<Point<i32, 2>>,
    sand: HashSet<Point<i32, 2>>,
    assumed_floor_y: i32,
}

impl InfiniteSandMap {
    pub fn new(rock_vertices: &Vec<Vec<Point<i32, 2>>>) -> Self {
        let mut rocks = HashSet::new();
        const FLOOR_OFFSET: i32 = 2;
        let mut assumed_floor_y: i32 = FLOOR_OFFSET;

        for vertex_group in rock_vertices {
            assumed_floor_y = std::cmp::max(assumed_floor_y, vertex_group[0].get(1) + FLOOR_OFFSET);
            for window in vertex_group.windows(2) {
                let line = OrthogonalLine2D::from_points(&window[0], &window[1]);
                for point in line.iter() {
                    rocks.insert(point);
                }
                assumed_floor_y = std::cmp::max(assumed_floor_y, window[1].get(1) + FLOOR_OFFSET);
            }
        }

        InfiniteSandMap {
            rocks,
            sand: HashSet::new(),
            assumed_floor_y,
        }
    }

    fn move_sand(
        &mut self,
        position: &Point<i32, 2>,
        movement: Vector<i32, 2>,
    ) -> Option<Point<i32, 2>> {
        assert!(movement.is_normalized(), "Movement must be normalized");
        let next_position = position.move_by(&movement).unwrap();
        if self.rocks.contains(&next_position)
            || self.sand.contains(&next_position)
            || *next_position.get(1) >= self.assumed_floor_y
        {
            None
        } else {
            Some(next_position)
        }
    }

    /// Drops a unit of sand from the source (500, 0) until it reaches the origin.
    ///
    /// # Return
    ///
    /// `true` if sand can still come out of the origin, `false` otherwise.
    pub fn drop_sand(&mut self) -> bool {
        let origin = Point::new([500, 0]);
        let mut current_position = origin;
        loop {
            if let Some(next_position) = self.move_sand(&current_position, Vector::new([0, 1])) {
                current_position = next_position;
                continue;
            }
            if let Some(next_position) = self.move_sand(&current_position, Vector::new([-1, 1])) {
                current_position = next_position;
                continue;
            }
            if let Some(next_position) = self.move_sand(&current_position, Vector::new([1, 1])) {
                current_position = next_position;
                continue;
            }
            // Sand cannot move anymore
            self.sand.insert(current_position);
            return current_position != origin;
        }
    }

    pub fn get_number_of_sand_units(&self) -> u32 {
        self.sand.len() as u32
    }
}
