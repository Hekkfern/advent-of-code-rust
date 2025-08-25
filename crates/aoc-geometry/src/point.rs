#[cfg(test)]
mod tests_2d;

#[cfg(test)]
mod tests_3d;

use crate::point_coordinate::{
    PointCoordinate, maximum_point_coordinate_value, minimum_point_coordinate_value,
};
use crate::vector::Vector;
use crate::vector_coordinate::VectorCoordinate;
use num::cast::cast;
use std::collections::HashSet;

/// A point in N-dimensional space with coordinates of type T.
///
/// This struct represents a point with N coordinates, where each coordinate
/// is of type T that must implement the `CoordinateValue` trait. The point
/// supports various geometric operations and can be used in conjunction
/// with vectors for spatial calculations.
///
/// # Type Parameters
///
/// * `T` - The numeric type for coordinates (must implement `CoordinateValue`)
/// * `N` - The number of dimensions (compile-time constant)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<T: PointCoordinate, const N: usize> {
    coordinates: [T; N],
}

impl<T: PointCoordinate, const N: usize> Point<T, N> {
    /// Creates a new point at the origin (all coordinates are zero).
    ///
    /// # Returns
    ///
    /// A new `Point` with all coordinates set to zero.
    pub fn origin() -> Self {
        Point {
            coordinates: [T::zero(); N],
        }
    }

    /// Creates a new point with the specified coordinates.
    ///
    /// # Arguments
    ///
    /// * `coordinates` - An array of N coordinates of type T
    ///
    /// # Returns
    ///
    /// A new `Point` with the given coordinates.
    pub fn new(coordinates: [T; N]) -> Self {
        assert!(
            coordinates
                .iter()
                .all(|&c| c >= minimum_point_coordinate_value()
                    && c <= maximum_point_coordinate_value()),
            "Coordinate value out of bounds"
        );
        Point { coordinates }
    }

    /// Gets a reference to the coordinate at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the coordinate to retrieve (0-based)
    ///
    /// # Returns
    ///
    /// A reference to the coordinate at the given index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    pub fn get(&self, index: usize) -> &T {
        assert!(index < N, "Axis index out of bounds");
        &self.coordinates[index]
    }

    /// Gets a reference to all coordinates as an array.
    ///
    /// # Returns
    ///
    /// A reference to the internal coordinates array.
    pub fn get_coordinates(&self) -> &[T; N] {
        &self.coordinates
    }

    /// Gets all neighboring points (points that differ by ±1 in exactly one coordinate).
    ///
    /// For each dimension, this method creates two neighbors: one with the coordinate
    /// incremented by 1 and one with the coordinate decremented by 1.
    ///
    /// # Returns
    ///
    /// A vector containing all 2×N neighboring points.
    pub fn get_neighbours(&self) -> HashSet<Self> {
        let mut neighbours = HashSet::with_capacity(2 * N);
        for i in 0..N {
            let mut neighbour = self.coordinates;
            neighbour[i] = neighbour[i] + T::one();
            neighbours.insert(Point::new(neighbour.clone()));
            neighbour[i] = neighbour[i] - T::one() - T::one();
            neighbours.insert(Point::new(neighbour.clone()));
        }
        neighbours
    }

    /// Mirrors the point across a specified origin point.
    ///
    /// # Arguments
    ///
    /// * `origin` - The origin point to mirror across
    ///
    /// # Returns
    ///
    /// A new point that is the mirror image of this point across the selected origin
    pub fn mirror(&self, origin: &Point<T, N>) -> Self {
        let vector: Vector<i64, _> = Vector::from_points(origin, self);
        let mirrored_vector = -vector;
        *origin + mirrored_vector
    }
}

/// Display formatting for points.
///
/// Formats the point as "(x,y)" for 2D, "(x,y,z)" for 3D, etc.
impl<T: PointCoordinate, const N: usize> std::fmt::Display for Point<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let coords = self.coordinates.map(|c| c.to_string());
        write!(f, "({})", coords.join(","))
    }
}

/// Index operation for points.
///
/// Allows accessing coordinates using bracket notation.
impl<T: PointCoordinate, const N: usize> std::ops::Index<usize> for Point<T, N> {
    type Output = T;

    /// Gets the coordinate at the specified index.
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < N, "Axis index out of bounds");
        self.get(index)
    }
}

/// Addition operation between a point and a vector.
///
/// Returns a new point that is the result of translating the original point by the vector.
impl<T: PointCoordinate, const N: usize, U: VectorCoordinate> std::ops::Add<Vector<U, N>>
    for Point<T, N>
{
    type Output = Self;

    /// Adds a vector to a point, resulting in a translated point.
    ///
    /// # Arguments
    ///
    /// * `vector` - The vector to add to this point
    ///
    /// # Returns
    ///
    /// A new point with coordinates that are the sum of the original point and vector coordinates.
    fn add(self, vector: Vector<U, N>) -> Self::Output {
        let mut result = [T::zero(); N];
        for i in 0..N {
            let mut new_coord = self.coordinates[i];
            if vector[i].is_positive() {
                new_coord = new_coord + cast(vector[i]).unwrap();
            } else {
                new_coord = new_coord - cast(vector[i]).unwrap();
            }
            result[i] = new_coord;
        }
        Point::new(result)
    }
}

/// Subtraction operation between a point and a vector.
///
/// Returns a new point that is the result of translating the original point by the negated vector.
impl<T: PointCoordinate, const N: usize, U: VectorCoordinate> std::ops::Sub<Vector<U, N>>
    for Point<T, N>
{
    type Output = Self;

    /// Subtracts a vector from a point, resulting in a translated point.
    ///
    /// This operation is equivalent to adding the negated vector to the point.
    ///
    /// # Arguments
    ///
    /// * `vector` - The vector to subtract from this point
    ///
    /// # Returns
    ///
    /// A new point with coordinates that are the difference between the original point and vector coordinates.
    fn sub(self, vector: Vector<U, N>) -> Self::Output {
        self + (-vector)
    }
}
