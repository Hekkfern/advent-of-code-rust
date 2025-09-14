use crate::generic::core::point_coordinate::PointCoordinate;
use crate::{Point, Vector};

const DIMENSIONS: usize = 2;
const NUMBER_OF_VERTICES: usize = 4;

pub struct SquareDiamond2D<T: PointCoordinate> {
    center: Point<T, DIMENSIONS>,
    distance: u64,
    vertices: [Point<T, DIMENSIONS>; NUMBER_OF_VERTICES],
}

impl<T: PointCoordinate> SquareDiamond2D<T> {
    pub fn from_center_and_perimeter_point(
        center: &Point<T, DIMENSIONS>,
        perimeter_point: &Point<T, DIMENSIONS>,
    ) -> Self {
        let v = Vector::<i64, DIMENSIONS>::from_points(center, perimeter_point).unwrap();
        Self::from_center_and_distance(center, v.manhattan_distance())
    }

    pub fn from_center_and_distance(center: &Point<T, DIMENSIONS>, distance: u64) -> Self {
        let converted_distance = T::from(distance).unwrap();
        let vertices = [
            Point::new([center[0] - converted_distance, center[1]]),
            Point::new([center[0], center[1] - converted_distance]),
            Point::new([center[0] + converted_distance, center[1]]),
            Point::new([center[0], center[1] + converted_distance]),
        ];
        SquareDiamond2D {
            center: *center,
            distance,
            vertices,
        }
    }

    pub fn center(&self) -> &Point<T, DIMENSIONS> {
        &self.center
    }

    pub fn get_vertexes(&self) -> &[Point<T, DIMENSIONS>; NUMBER_OF_VERTICES] {
        &self.vertices
    }

    /// Gets the distance from the center to the perimeter.
    pub fn distance(&self) -> u64 {
        self.distance
    }

    pub fn is_outside(&self, point: &Point<T, DIMENSIONS>) -> bool {
        let v = Vector::<i64, DIMENSIONS>::from_points(&self.center, point).unwrap();
        v.manhattan_distance() > self.distance
    }

    pub fn is_inside(&self, point: &Point<T, DIMENSIONS>) -> bool {
        let v = Vector::<i64, 2>::from_points(&self.center, point).unwrap();
        v.manhattan_distance() <= self.distance
    }

    pub fn is_on_border(&self, point: &Point<T, DIMENSIONS>) -> bool {
        let v = Vector::<i64, 2>::from_points(&self.center, point).unwrap();
        v.manhattan_distance() == self.distance
    }

    pub fn area(&self) -> u64 {
        let diagonal_length = (2 * self.distance) + 1;
        (diagonal_length.pow(2) / 2) + 1
    }

    /// Returns an iterator over all points that are just outside the border of the shape.
    /// Each point is adjacent to exactly one border point (up, down, left, or right).
    /// The iteration starts at the top vertex and proceeds clockwise around the shape.
    pub fn step_around_outside_border(&self) -> impl Iterator<Item = Point<T, DIMENSIONS>> {
        let center = self.center;
        let d = T::from(self.distance + 1).unwrap();

        // The diamond's outside border consists of 4 straight lines (top-right, right-bottom, bottom-left, left-top)
        // We'll walk from (center.x, center.y - d) clockwise

        let mut points = Vec::with_capacity((self.distance as usize + 1) * 4);

        // Top to right
        for i in 0..=self.distance {
            points.push(Point::new([
                center[0] + T::from(i).unwrap(),
                center[1] - d + T::from(i).unwrap(),
            ]));
        }
        // Right to bottom
        for i in 0..=self.distance {
            points.push(Point::new([
                center[0] + d - T::from(i).unwrap(),
                center[1] + T::from(i).unwrap(),
            ]));
        }
        // Bottom to left
        for i in 0..=self.distance {
            points.push(Point::new([
                center[0] - T::from(i).unwrap(),
                center[1] + d - T::from(i).unwrap(),
            ]));
        }
        // Left to top
        for i in 0..=self.distance {
            points.push(Point::new([
                center[0] - d + T::from(i).unwrap(),
                center[1] - T::from(i).unwrap(),
            ]));
        }

        points.into_iter()
    }
}
