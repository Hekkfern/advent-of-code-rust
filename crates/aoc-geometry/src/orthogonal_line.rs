use crate::direction::Direction;
use crate::point::Point;
use crate::point_coordinate::PointCoordinate;
use crate::vector::Vector;
use crate::vector_coordinate::VectorCoordinate;

const NUM_OF_VERTEXES_IN_ORTHOGONAL_LINE: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrthogonalLine<T: PointCoordinate, const N: usize> {
    /// The two points that define the line segment.
    vertices: [Point<T, N>; NUM_OF_VERTEXES_IN_ORTHOGONAL_LINE],
}

impl<T: PointCoordinate, const N: usize> OrthogonalLine<T, N> {
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
        let vector = Vector::<i64, N>::from_points(&p1, &p2);
        assert!(vector.is_some(), "Vector cannot be created from points.");
        assert!(
            vector.unwrap().is_axis(),
            "Orthogonal line must be axis-aligned."
        );
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
        assert!(v.is_axis(), "Orthogonal line must be axis-aligned.");
        let p2 = p1.move_by(&v);
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
    pub fn get_vertexes(&self) -> &[Point<T, N>; NUM_OF_VERTEXES_IN_ORTHOGONAL_LINE] {
        &self.vertices
    }

    /// Gets the dimension index of the axis that this line is aligned with.
    ///
    /// # Returns
    ///
    /// The index of the axis (0-based, and less than N) that the line is aligned with.
    pub fn get_axis(&self) -> usize {
        let vector = self.inherent_vector();
        for i in 0..N {
            if *vector.get(i) != 0 {
                return i;
            }
        }
        panic!("Line is not aligned with any axis, all coordinates are zero.");
    }

    /// Creates an iterator that yields all points along the line from start to end.
    ///
    /// The iterator moves one unit at a time along the axis the line is aligned with.
    ///
    /// # Returns
    ///
    /// An iterator that yields `Point<T, N>` values
    pub fn iter(&self) -> OrthogonalLineIterator<T, N> {
        let axis = self.get_axis();
        let start = self.vertices[0];
        let end = self.vertices[1];

        // Determine direction (1 or -1) along the axis
        let direction = if start.get(axis) <= end.get(axis) {
            Direction::Positive
        } else {
            Direction::Negative
        };

        OrthogonalLineIterator {
            current: start,
            end,
            axis,
            direction,
            finished: false,
        }
    }
}

/// Iterator for points along an orthogonal line.
///
/// This iterator yields all points from the start to the end of the line,
/// moving one unit at a time along the axis the line is aligned with.
pub struct OrthogonalLineIterator<T: PointCoordinate, const N: usize> {
    current: Point<T, N>,
    end: Point<T, N>,
    axis: usize,
    direction: Direction,
    finished: bool,
}

impl<T: PointCoordinate, const N: usize> Iterator for OrthogonalLineIterator<T, N> {
    type Item = Point<T, N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let current_point = self.current;

        // Check if we've reached the end point
        if self.current == self.end {
            self.finished = true;
            return Some(current_point);
        }

        // Move one unit in the direction along the axis
        let mut next_coordinates = *self.current.get_coordinates();
        if self.direction == Direction::Positive {
            next_coordinates[self.axis] = next_coordinates[self.axis] + T::one();
        } else {
            next_coordinates[self.axis] = next_coordinates[self.axis] - T::one();
        }
        self.current = Point::new(next_coordinates);

        Some(current_point)
    }
}

impl<T: PointCoordinate, const N: usize> IntoIterator for OrthogonalLine<T, N> {
    type Item = Point<T, N>;
    type IntoIter = OrthogonalLineIterator<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: PointCoordinate, const N: usize> IntoIterator for &OrthogonalLine<T, N> {
    type Item = Point<T, N>;
    type IntoIter = OrthogonalLineIterator<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Display formatting for orthogonal lines.
///
/// Formats the line as "[(x,y),(x,y)]" for 2D, "[(x,y,z),(x,y,z)]" for 3D, etc.
impl<T: PointCoordinate, const N: usize> std::fmt::Display for OrthogonalLine<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let points = self.vertices.map(|c| c.to_string());
        write!(f, "[{}]", points.join(","))
    }
}
