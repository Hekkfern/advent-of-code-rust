#[cfg(test)]
mod orthogonal_line_tests;

use crate::AxisDirection;
use crate::Point;
use crate::Vector;
use crate::generic::core::point_coordinate::PointCoordinate;
use crate::generic::core::vector_coordinate::VectorCoordinate;

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
        let vector = Vector::<i64, N>::from_points(p1, p2);
        assert!(vector.is_some(), "Vector cannot be created from points.");
        assert!(
            vector.unwrap().is_axis(),
            "Orthogonal line must be axis-aligned."
        );
        Self {
            vertices: [*p1, *p2],
        }
    }

    /// Attempts to create a new orthogonal line from two distinct points.
    ///
    /// This function is similar to `from_points`, but returns `None` instead of panicking
    /// if the points are identical or if the vector formed by the points is neither axis-aligned.
    ///
    /// # Arguments
    ///
    /// * `p1` - The first point (start of the line)
    /// * `p2` - The second point (end of the line)
    ///
    /// # Returns
    ///
    /// `Some(Line)` if the points form a valid orthogonal line, `None` otherwise
    pub fn try_from_points(p1: &Point<T, N>, p2: &Point<T, N>) -> Option<Self> {
        if p1 == p2 {
            return None;
        }
        let vector = Vector::<i64, N>::from_points(p1, p2)?;
        if !vector.is_axis() {
            return None;
        }
        Some(Self {
            vertices: [*p1, *p2],
        })
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
        let p2 = p1.move_by(v);
        assert!(p2.is_some(), "Other vertex is out of bounds.");
        Self {
            vertices: [*p1, p2.unwrap()],
        }
    }

    /// Calculates the length of the line.
    ///
    /// # Returns
    ///
    /// The length.
    pub fn length(&self) -> u64 {
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

    pub fn contains_point(&self, point: &Point<T, N>) -> bool {
        let axis = self.get_axis();
        // Check if the point matches the fixed coordinates
        for i in 0..N {
            if i != axis && self.vertices[0].get(i) != point.get(i) {
                return false;
            }
        }
        // Check if the point's coordinate along the line's axis is within the line segment
        let min_coord = std::cmp::min(self.vertices[0].get(axis), self.vertices[1].get(axis));
        let max_coord = std::cmp::max(self.vertices[0].get(axis), self.vertices[1].get(axis));
        point.get(axis) >= min_coord && point.get(axis) <= max_coord
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
            AxisDirection::Positive
        } else {
            AxisDirection::Negative
        };

        OrthogonalLineIterator {
            current: start,
            end,
            axis,
            direction,
            finished: false,
        }
    }

    /// Checks if this line overlaps with another orthogonal line, in one or more points.
    ///
    /// Two lines overlap in one of these situations:
    /// * they are collinear and share at least one point.
    /// * they are aligned along different axes and intersect at a single point.
    ///
    /// # Arguments
    ///
    /// * `other` - The other orthogonal line to check for overlap
    ///
    /// # Returns
    ///
    /// `true` if the lines overlap, `false` otherwise
    pub fn overlaps(&self, other: &Self) -> bool {
        let self_axis = self.get_axis();
        let other_axis = other.get_axis();
        if self_axis == other_axis {
            // Check for collinear overlap
            other
                .vertices
                .iter()
                .any(|point| self.contains_point(point))
        } else {
            // Check for intersection at a single point
            let self_common_coords: Vec<_> = self.vertices[0]
                .get_coordinates()
                .iter()
                .zip(self.vertices[1].get_coordinates().iter())
                .filter_map(|(x, y)| if x == y { Some(*x) } else { None })
                .collect();
            let other_common_coords: Vec<_> = other.vertices[0]
                .get_coordinates()
                .iter()
                .zip(other.vertices[1].get_coordinates().iter())
                .filter_map(|(x, y)| if x == y { Some(*x) } else { None })
                .collect();
            self_common_coords
                .iter()
                .any(|x| other_common_coords.contains(x))
        }
    }

    /// Gets the common points between two orthogonal lines.
    ///
    /// If the lines are collinear, returns all points along the overlapping segment.
    /// If they are perpendicular and intersect, returns the intersection point.
    /// If they do not overlap, returns an empty vector.
    ///
    /// # Arguments
    /// * `line1` - The first orthogonal line
    /// * `line2` - The second orthogonal line
    ///
    /// # Returns
    /// A vector of overlapping points, which may be empty
    pub fn intersect(&self, other: &Self) -> Vec<Point<T, N>> {
        let axis1 = self.get_axis();
        let axis2 = other.get_axis();
        let vertices1 = self.get_vertexes();
        let vertices2 = other.get_vertexes();

        if axis1 == axis2 {
            // Collinear case: check for overlap segment
            let fixed_axis = if axis1 == 0 { 1 } else { 0 };
            let fixed_coord_1 = *vertices1[0].get(fixed_axis);
            let fixed_coord_2 = *vertices2[0].get(fixed_axis);
            if fixed_coord_1 != fixed_coord_2 {
                return vec![];
            }
            let start1 = *vertices1[0].get(axis1);
            let end1 = *vertices1[1].get(axis1);
            let start2 = *vertices2[0].get(axis2);
            let end2 = *vertices2[1].get(axis2);
            let min_overlap = std::cmp::max(std::cmp::min(start1, end1), std::cmp::min(start2, end2));
            let max_overlap = std::cmp::min(std::cmp::max(start1, end1), std::cmp::max(start2, end2));
            if min_overlap > max_overlap {
                return vec![];
            }
            let mut points = Vec::new();
            let mut coord = min_overlap;
            while coord <= max_overlap {
                let mut coords = [T::zero(); N];
                coords[axis1] = coord;
                coords[fixed_axis] = fixed_coord_1;
                points.push(Point::<T, N>::new(coords));
                coord = coord + T::one();
            }
            points
        } else {
            // Perpendicular case: check for intersection
            let mut coords = [T::zero(); N];
            coords[axis1] = *vertices2[0].get(axis1);
            coords[axis2] = *vertices1[0].get(axis2);
            let intersection = Point::<T, N>::new(coords);
            if self.contains_point(&intersection) && other.contains_point(&intersection) {
                vec![intersection]
            } else {
                vec![]
            }
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
    direction: AxisDirection,
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
        if self.direction == AxisDirection::Positive {
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
