#[cfg(test)]
mod point_tests;

use crate::AxisDirection;
use crate::generic::core::point_coordinate::PointCoordinate;
use crate::Vector;
use crate::generic::core::vector_coordinate::VectorCoordinate;
use num_traits::cast::cast;
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
        Self {
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
        Self { coordinates }
    }

    pub fn extremes(sides: [AxisDirection; N]) -> Self {
        let mut coords = [T::zero(); N];
        for i in 0..N {
            coords[i] = match sides[i] {
                AxisDirection::Positive => T::max_value(),
                AxisDirection::Negative => T::min_value(),
            }
        }
        Point::new(coords)
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

    /// Checks if the point is at the origin (all coordinates are zero).
    ///
    /// # Returns
    ///
    /// `true` if the point is at the origin, `false` otherwise.
    pub fn is_origin(&self) -> bool {
        self.coordinates.iter().all(|&c| c == T::zero())
    }

    /// Gets all neighboring points (points that differ by ±1 in exactly one coordinate).
    ///
    /// For each dimension, this method creates two neighbors: one with the coordinate
    /// incremented by 1 and one with the coordinate decremented by 1.
    ///
    /// # Returns
    ///
    /// A vector containing up to 2×N neighboring points.
    pub fn get_neighbors(&self) -> HashSet<Self> {
        let mut neighbours = HashSet::with_capacity(2 * N);
        for i in 0..N {
            // Try incrementing
            if let Some(inc) = self.coordinates[i].checked_add(&T::one()) {
                let mut neighbour = self.coordinates;
                neighbour[i] = inc;
                neighbours.insert(Point::new(neighbour));
            }
            // Try decrementing
            if let Some(dec) = self.coordinates[i].checked_sub(&T::one()) {
                let mut neighbour = self.coordinates;
                neighbour[i] = dec;
                neighbours.insert(Point::new(neighbour));
            }
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
    pub fn mirror(&self, origin: &Point<T, N>) -> Option<Self> {
        let vector = Vector::<i64, N>::from_points(origin, self)?;
        let mirrored_vector = vector.invert()?;
        origin.move_by(&mirrored_vector)
    }

    /// Adds a vector to a point, resulting in a translated point.
    ///
    /// # Arguments
    ///
    /// * `vector` - The vector to add to this point
    ///
    /// # Returns
    ///
    /// A new point with coordinates that are the sum of the original point and vector coordinates.
    pub fn move_by<U>(self, vector: &Vector<U, N>) -> Option<Self>
    where
        U: VectorCoordinate,
    {
        let mut result = [T::zero(); N];
        for i in 0..N {
            let v = vector[i];
            let p = self.coordinates[i];
            let v_abs = cast(v.abs())?;
            let new_coord = if v.is_positive() || v.is_zero() {
                p.checked_add(&cast(v)?)?
            } else {
                p.checked_sub(&v_abs)?
            };
            result[i] = new_coord;
        }
        Some(Point::new(result))
    }

    pub fn is_in(&self, axis: usize, position: T) -> bool {
        assert!(axis < N, "Axis index out of bounds");
        self.coordinates[axis] == position
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
