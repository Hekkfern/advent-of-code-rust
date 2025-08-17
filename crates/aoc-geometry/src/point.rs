#[cfg(test)]
mod tests_2d;

#[cfg(test)]
mod tests_3d;

use crate::coordinate_value::{
    CoordinateValue, maximum_coordinate_value, minimum_coordinate_value,
};
use crate::vector::Vector;
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
pub struct Point<T: CoordinateValue, const N: usize> {
    coordinates: [T; N],
}

impl<T: CoordinateValue, const N: usize> Point<T, N> {
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
        for coordinate in coordinates {
            assert!(
                coordinate >= minimum_coordinate_value()
                    && coordinate <= maximum_coordinate_value(),
                "Coordinate value out of bounds: {}",
                coordinate
            );
        }
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
}

/// Negation operation for points.
///
/// Returns a new point with all coordinates negated, i.e., the mirror image of the original point across the origin.
impl<T: CoordinateValue, const N: usize> std::ops::Neg for Point<T, N> {
    type Output = Self;

    /// Negates all coordinates of the point.
    fn neg(self) -> Self::Output {
        Point {
            coordinates: self.coordinates.map(|c| -c),
        }
    }
}

/// Display formatting for points.
///
/// Formats the point as "(x,y)" for 2D, "(x,y,z)" for 3D, etc.
impl<T: CoordinateValue, const N: usize> std::fmt::Display for Point<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let coords = self.coordinates.map(|c| c.to_string());
        write!(f, "({})", coords.join(","))
    }
}

/// Index operation for points.
///
/// Allows accessing coordinates using bracket notation.
impl<T: CoordinateValue, const N: usize> std::ops::Index<usize> for Point<T, N> {
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
impl<T: CoordinateValue, const N: usize> std::ops::Add<Vector<T, N>> for Point<T, N> {
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
    fn add(self, vector: Vector<T, N>) -> Self::Output {
        let mut result = [T::zero(); N];
        for i in 0..N {
            result[i] = self.coordinates[i] + vector[i];
        }
        Point::new(result)
    }
}

/// Subtraction operation between a point and a vector.
///
/// Returns a new point that is the result of translating the original point by the negated vector.
impl<T: CoordinateValue, const N: usize> std::ops::Sub<Vector<T, N>> for Point<T, N> {
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
    fn sub(self, vector: Vector<T, N>) -> Self::Output {
        self + (-vector)
    }
}
