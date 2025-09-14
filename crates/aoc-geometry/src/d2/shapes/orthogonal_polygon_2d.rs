#[cfg(test)]
mod orthogonal_polygon_2d_tests;

use crate::OrthogonalLine;
use crate::Point;
use crate::Vector;
use crate::generic::core::point_coordinate::PointCoordinate;
use crate::generic::core::vector_coordinate::VectorCoordinate;
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
        assert!(
            Self::are_vertices_orthogonal(&vertices),
            "The provided vertices do not form an orthogonal polygon."
        );
        Self { vertices }
    }

    fn are_vertices_orthogonal(vertices: &Vec<Point<T, DIMENSIONS>>) -> bool {
        vertices
            .iter()
            .zip(vertices.iter().cycle().skip(1).take(vertices.len()))
            .all(|(a, b)| {
                Vector::<i64, DIMENSIONS>::from_points(a, b)
                    .map(|v| v.is_axis())
                    .unwrap_or(false)
            })
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
    ///   in a counter-clock-wise or clock-wise sequence.
    fn calculate_arbitrary_polygon_area(perimeter_points: &Vec<Point<T, DIMENSIONS>>) -> f64 {
        let (left_sum, right_sum) = perimeter_points
            .iter()
            .zip(
                perimeter_points
                    .iter()
                    .cycle()
                    .skip(1)
                    .take(perimeter_points.len()),
            )
            .fold((0, 0), |(left, right): (i128, i128), (a, b)| {
                (
                    left + cast::<T, i128>(a[0]).unwrap() * cast::<T, i128>(b[1]).unwrap(),
                    right + cast::<T, i128>(b[0]).unwrap() * cast::<T, i128>(a[1]).unwrap(),
                )
            });

        cast::<i128, f64>(left_sum - right_sum).unwrap().abs() * 0.5
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
        let angle: f64 = self
            .vertices
            .iter()
            .zip(
                self.vertices
                    .iter()
                    .cycle()
                    .skip(1)
                    .take(self.vertices.len()),
            )
            .map(|(a, b)| {
                let v1 = Vector::<i64, DIMENSIONS>::from_points(a, point).unwrap();
                let v2 = Vector::<i64, DIMENSIONS>::from_points(b, point).unwrap();
                calculate_angle_between_vectors(&v1, &v2)
            })
            .sum();
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
        self.vertices
            .iter()
            .zip(
                self.vertices
                    .iter()
                    .cycle()
                    .skip(1)
                    .take(self.vertices.len()),
            )
            .any(|(a, b)| {
                let line = OrthogonalLine::from_points(a, b);
                line.contains_point(point)
            })
    }

    /// Calculates the perimeter length of this shape.
    pub fn perimeter(&self) -> u64 {
        self.vertices
            .iter()
            .zip(
                self.vertices
                    .iter()
                    .cycle()
                    .skip(1)
                    .take(self.vertices.len()),
            )
            .map(|(a, b)| {
                Vector::<i64, DIMENSIONS>::from_points(b, a)
                    .unwrap()
                    .manhattan_distance()
            })
            .sum()
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
    pub fn number_of_intrinsic_points(&self) -> u64 {
        let boundary_points = self.get_boundary_points();
        let area = Self::calculate_arbitrary_polygon_area(&boundary_points);
        // Using Pick's Theorem: A = I + B/2 - 1  =>  I = A - B/2 + 1
        (area - (boundary_points.len() as f64 * 0.5) + 1.0) as u64
    }

    fn get_boundary_points(&self) -> Vec<Point<T, DIMENSIONS>> {
        self.vertices
            .iter()
            .zip(
                self.vertices
                    .iter()
                    .cycle()
                    .skip(1)
                    .take(self.vertices.len()),
            )
            .flat_map(|(start, end)| {
                let vector = Vector::<i64, DIMENSIONS>::from_points(start, end).unwrap();
                let unary_vector = vector.normalize();
                let mut current_point = *start;
                let mut points = Vec::new();
                while current_point != *end {
                    current_point = current_point.move_by(&unary_vector).unwrap();
                    points.push(current_point);
                }
                points
            })
            .collect()
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
