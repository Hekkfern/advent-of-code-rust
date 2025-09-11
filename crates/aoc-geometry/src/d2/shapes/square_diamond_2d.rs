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
        center: Point<T, DIMENSIONS>,
        perimeter_point: Point<T, DIMENSIONS>,
    ) -> Self {
        let v = Vector::<i64, DIMENSIONS>::from_points(&center, &perimeter_point).unwrap();
        Self::from_center_and_distance(center, v.manhattan_distance())
    }

    pub fn from_center_and_distance(center: Point<T, DIMENSIONS>, distance: u64) -> Self {
        let converted_distance = T::from(distance).unwrap();
        let vertices = [
            Point::new([center[0] - converted_distance, center[1]]),
            Point::new([center[0], center[1] - converted_distance]),
            Point::new([center[0] + converted_distance, center[1]]),
            Point::new([center[0], center[1] + converted_distance]),
        ];
        SquareDiamond2D {
            center,
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
}
