#[cfg(test)]
mod tests;

use crate::orthogonal_line::OrthogonalLine;
use crate::point::Point;
use crate::point_coordinate::PointCoordinate;
use crate::vector::Vector;
use crate::vector_coordinate::VectorCoordinate;
use num_traits::cast::cast;

const DIMENSIONS: usize = 2;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrthogonalPolygon2D<T: PointCoordinate> {
    vertices: Vec<Point<T, DIMENSIONS>>,
}

impl<T: PointCoordinate> OrthogonalPolygon2D<T> {
    /// Creates a new orthogonal polygon from a list of vertices.
    ///
    /// # Arguments
    ///
    /// * `vertices` - List of vertexes of the shape, in sequential order following the perimeter (clockwise or counterclockwise).
    pub fn from_vertices(vertices: Vec<Point<T, DIMENSIONS>>) -> Self {
        assert!(
            vertices.len() >= 3,
            "A polygon must have at least 3 vertices."
        );
        Self { vertices }
    }

    /// Returns all the vertexes of this shape.
    pub fn get_vertexes(&self) -> &Vec<Point<T, DIMENSIONS>> {
        &self.vertices
    }

    /// Calculates the area of this shape.
    pub fn area(&self) -> f64 {
        Self::calculate_arbitrary_polygon_area(&self.vertices)
    }

    /// Applies the "Shoelace formula" (https://en.wikipedia.org/wiki/Shoelace_formula), a mathematical
    /// algorithm to determine the area of a simple polygon whose vertices are described by their
    /// Cartesian coordinates in the place.
    ///
    /// # Arguments
    ///
    /// * `perimeter_points` - List of all the points forming the perimeter of the polygon, ordered
    ///                        in a counter-clock-wise or clock-wise sequence.
    fn calculate_arbitrary_polygon_area(perimeter_points: &Vec<Point<T, DIMENSIONS>>) -> f64 {
        let n = perimeter_points.len();
        let mut left_sum = T::zero();
        let mut right_sum = T::zero();

        for i in 0..n {
            let j = (i + 1) % n;
            let xi = perimeter_points[i][0];
            let yi = perimeter_points[i][1];
            let xj = perimeter_points[j][0];
            let yj = perimeter_points[j][1];

            left_sum = left_sum + xi * yj;
            right_sum = right_sum + xj * yi;
        }

        (cast::<T, f64>(left_sum).unwrap() - cast::<T, f64>(right_sum).unwrap()).abs() * 0.5
    }

    /// Determines whether the specified point is outside the shape.
    ///
    /// WARNING: Points on the edge are not considered.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to check.
    ///
    /// # Returns
    ///
    /// * `true` if the point is outside the shape, `false` otherwise.
    pub fn is_outside(&self, point: &Point<T, DIMENSIONS>) -> bool {
        !self.is_inside(point) && !self.is_on_edge(point)
    }

    /// Determines whether the specified point is inside the shape using
    /// [Winding Number algorithm](https://en.wikipedia.org/wiki/Point_in_polygon#Winding_number_algorithm).
    ///
    /// WARNING: Points on the edge are not considered.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to check.
    ///
    /// # Returns
    ///
    /// * `true` if the point is inside the shape, `false` otherwise.
    pub fn is_inside(&self, point: &Point<T, DIMENSIONS>) -> bool {
        use std::f64::consts::PI;
        let n = self.vertices.len();
        let mut angle = 0.0;

        for i in 0..n {
            let v1 = Vector::<i64, DIMENSIONS>::from_points(&self.vertices[i], point).unwrap();
            let v2 =
                Vector::<i64, DIMENSIONS>::from_points(&self.vertices[(i + 1) % n], point).unwrap();
            angle += calculate_angle_between_vectors(&v1, &v2);
        }

        angle.abs() >= PI
    }

    /// Determines whether the specified point is on the border the shape.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to check.
    ///
    /// # Returns
    ///
    /// * `true` if the point is on the border of the shape, `false` otherwise.
    pub fn is_on_edge(&self, point: &Point<T, DIMENSIONS>) -> bool {
        let n = self.vertices.len();
        for i in 0..n {
            let a = &self.vertices[i];
            let b = &self.vertices[(i + 1) % n];
            let line = OrthogonalLine::from_points(&a, &b);
            if line.contains_point(&point) {
                return true;
            }
        }
        false
    }

    /// Calculates the perimeter length of this shape.
    pub fn perimeter(&self) -> u64 {
        let n = self.vertices.len();
        let mut length: u64 = 0;

        for index in 0..n {
            let next_index = (index + 1) % n;
            length += Vector::<i64, DIMENSIONS>::from_points(
                &self.vertices[next_index],
                &self.vertices[index],
            )
            .unwrap()
            .manhattan_distance();
        }
        length
    }

    /// Calculates the number of intrinsic points inside this shape.
    ///
    /// Applies [Pick's theorem](https://en.wikipedia.org/wiki/Pick%27s_theorem), a formula for the
    /// area of a simple polygon with integer vertex coordinates, to find the number of integer
    /// points interior to the polygon.
    ///
    /// # Returns
    ///
    /// The number of intrinsic points inside the shape.
    pub fn calculate_number_of_intrinsic_points(&self) -> u64 {
        let boundary_points = self.calculate_boundary_points();
        let boundary_count = boundary_points.len() as u64;
        let area = Self::calculate_arbitrary_polygon_area(&boundary_points);
        // Using Pick's Theorem: A = I + B/2 - 1  =>  I = A - B/2 + 1
        (area - (boundary_count as f64 * 0.5) + 1.0) as u64
    }

    fn calculate_boundary_points(&self) -> Vec<Point<T, DIMENSIONS>> {
        let n = self.vertices.len();
        let mut points = Vec::with_capacity(n * 2);

        for index in 0..n {
            let next_index = (index + 1) % n;
            let vector = Vector::<i64, DIMENSIONS>::from_points(
                &self.vertices[index],
                &self.vertices[next_index],
            )
            .unwrap();
            let unary_vector = vector.normalize();

            let mut current_point = self.vertices[index];
            while current_point != self.vertices[next_index] {
                current_point = current_point.move_by(&unary_vector).unwrap();
                points.push(current_point);
            }
        }
        points
    }
}

/// Calculates the angle (in radians) between two vectors.
fn calculate_angle_between_vectors<U>(v1: &Vector<U, DIMENSIONS>, v2: &Vector<U, DIMENSIONS>) -> f64
where
    U: VectorCoordinate,
{
    use std::f64::consts::PI;
    const TWO_PI: f64 = 2.0 * PI;

    let theta1 = v1[1].to_f64().unwrap().atan2(v1[0].to_f64().unwrap());
    let theta2 = v2[1].to_f64().unwrap().atan2(v2[0].to_f64().unwrap());
    let mut diff = theta2 - theta1;
    // Normalize to [-PI, PI)
    diff = (diff + PI).rem_euclid(TWO_PI) - PI;
    diff
}
