use crate::point::Point;
use crate::point_coordinate::PointCoordinate;
use crate::vector::Vector;
use crate::vector_coordinate::VectorCoordinate;

const NUM_OF_VERTEXES_IN_LINE: usize = 2;

/// Represents a line segment in N-dimensional space defined by two distinct points.
///
/// A line is determined by exactly two points and provides various geometric operations
/// such as distance calculations and axis alignment checks. The line is directed from
/// the first point to the second point.
///
/// # Type Parameters
///
/// * `T` - The type of coordinate values.
/// * `N` - The number of dimensions (must be known at compile time).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Line<T: PointCoordinate, const N: usize> {
    /// The two points that define the line segment.
    vertices: [Point<T, N>; NUM_OF_VERTEXES_IN_LINE],
}

impl<T: PointCoordinate, const N: usize> Line<T, N> {
    /// Returns the vector from the first point to the second point.
    ///
    /// This internal method calculates the directional vector that represents
    /// the line's direction and magnitude.
    ///
    /// # Returns
    ///
    /// A `Vector` representing the direction and length of the line
    fn inherent_vector(&self) -> Vector<i128, N> {
        Vector::<i128, N>::from_points(&self.vertices[0], &self.vertices[1])
            .expect("Inherent vector cannot be created")
    }

    /// Creates a new line from two distinct points.
    ///
    /// The line will be directed from the first point to the second point.
    ///
    /// # Arguments
    ///
    /// * `p1` - The first point (start of the line)
    /// * `p2` - The second point (end of the line)
    ///
    /// # Returns
    ///
    /// A new `Line` connecting the two points
    ///
    /// # Panics
    ///
    /// Panics if the two points are identical, as they cannot form a line
    pub fn from_points(p1: &Point<T, N>, p2: &Point<T, N>) -> Self {
        assert!(p1 != p2, "Points must be distinct to form a line.");
        Self {
            vertices: [*p1, *p2],
        }
    }

    /// Creates a new line from a starting point and a direction vector.
    ///
    /// The line will start at the given point and extend in the direction
    /// and magnitude specified by the vector.
    ///
    /// # Arguments
    ///
    /// * `p1` - The starting point of the line
    /// * `v` - The vector defining the direction and length of the line
    ///
    /// # Returns
    ///
    /// A new `Line` starting at the point and extending by the vector
    ///
    /// # Panics
    ///
    /// Panics if the vector is zero, as it cannot define a line direction
    pub fn from_point_and_vector<U>(p1: &Point<T, N>, v: &Vector<U, N>) -> Self
    where
        U: VectorCoordinate,
    {
        assert!(!v.is_zero(), "Vector must be non-zero to form a line.");
        let p2 = p1.move_by(v);
        assert!(p2.is_some(), "Other vertex is out of bounds.");
        Self {
            vertices: [*p1, p2.unwrap()],
        }
    }

    /// Gets the absolute length of each coordinate.
    ///
    /// This method returns an array where each element is the absolute value
    /// of the corresponding coordinate, converted to u64.
    ///
    /// # Returns
    ///
    /// An array of absolute coordinate values as u64.
    pub fn absolute_coordinates(&self) -> [u64; N] {
        self.inherent_vector().absolute_coordinates()
    }

    /// Calculates the Manhattan Distance.
    ///
    /// The Manhattan distance (also known as L1 norm or taxicab distance)
    /// is the sum of the absolute values of all coordinates.
    ///
    /// # Returns
    ///
    /// The Manhattan distance as u64.
    pub fn manhattan_distance(&self) -> u64 {
        self.inherent_vector().manhattan_distance()
    }

    /// Returns the two points that define the line.
    ///
    /// # Returns
    ///
    /// A reference to the array containing the two vertices of the line
    pub fn get_vertexes(&self) -> &[Point<T, N>; NUM_OF_VERTEXES_IN_LINE] {
        &self.vertices
    }

    /// Checks if this is an axis-aligned line (only one coordinate is non-zero).
    ///
    /// # Returns
    ///
    /// `true` if exactly one coordinate is non-zero, `false` otherwise.
    pub fn is_axis(&self) -> bool {
        self.inherent_vector().is_axis()
    }
}

/// Display formatting for lines.
///
/// Formats the line as "[(x,y),(x,y)]" for 2D, "[(x,y,z),(x,y,z)]" for 3D, etc.
impl<T: PointCoordinate, const N: usize> std::fmt::Display for Line<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let points = self.vertices.map(|c| c.to_string());
        write!(f, "[{}]", points.join(","))
    }
}
