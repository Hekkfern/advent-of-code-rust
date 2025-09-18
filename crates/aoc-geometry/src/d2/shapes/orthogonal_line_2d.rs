#[cfg(test)]
mod orthogonal_line_2d_tests;

use crate::Point;
use crate::Vector;
use crate::generic::core::point_coordinate::PointCoordinate;
use crate::generic::core::vector_coordinate::VectorCoordinate;

const DIMENSIONS: usize = 2;
const NUM_OF_VERTEXES_IN_LINE: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrthogonalLine2DType {
    /// An axis-aligned vector where only one coordinate is non-zero. It tells which axis it is aligned with.
    Axis(usize),
    /// A diagonal vector where at least two coordinates have the same absolute value.
    Diagonal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrthogonalLine2D<T: PointCoordinate> {
    /// The two points that define the line segment.
    vertices: [Point<T, DIMENSIONS>; NUM_OF_VERTEXES_IN_LINE],
}

impl<T: PointCoordinate> OrthogonalLine2D<T> {
    /// Returns the vector from the first point to the second point.
    ///
    /// This internal method calculates the directional vector that represents
    /// the line's direction and magnitude.
    ///
    /// # Returns
    ///
    /// A `Vector` representing the direction and length of the line
    fn inherent_vector(&self) -> Vector<i128, DIMENSIONS> {
        Vector::<i128, DIMENSIONS>::from_points(&self.vertices[0], &self.vertices[1])
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
    /// Panics if:
    /// * the two points are identical, as they cannot form a line
    /// * the vector formed by the points is neither axis-aligned nor diagonal
    pub fn from_points(p1: &Point<T, DIMENSIONS>, p2: &Point<T, DIMENSIONS>) -> Self {
        assert!(p1 != p2, "Points must be distinct to form a line.");
        let vector = Vector::<i64, DIMENSIONS>::from_points(p1, p2);
        assert!(vector.is_some(), "Vector cannot be created from points.");
        let vector = vector.unwrap();
        assert!(
            vector.is_axis() || vector.is_diagonal(),
            "Orthogonal line must be axis-aligned or diagonal."
        );
        Self {
            vertices: [*p1, *p2],
        }
    }

    /// Attempts to create a new orthogonal line from two distinct points.
    ///
    /// This function is similar to `from_points`, but returns `None` instead of panicking
    /// if the points are identical or if the vector formed by the points is neither axis-aligned
    /// nor diagonal.
    ///
    /// # Arguments
    ///
    /// * `p1` - The first point (start of the line)
    /// * `p2` - The second point (end of the line)
    ///
    /// # Returns
    ///
    /// `Some(Line)` if the points form a valid orthogonal line, `None` otherwise
    pub fn try_from_points(p1: &Point<T, DIMENSIONS>, p2: &Point<T, DIMENSIONS>) -> Option<Self> {
        if p1 == p2 {
            return None;
        }
        let vector = Vector::<i64, DIMENSIONS>::from_points(p1, p2)?;
        if !vector.is_axis() && !vector.is_diagonal() {
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
    pub fn from_point_and_vector<U>(p1: &Point<T, DIMENSIONS>, v: &Vector<U, DIMENSIONS>) -> Self
    where
        U: VectorCoordinate,
    {
        assert!(!v.is_zero(), "Vector must be non-zero to form a line.");
        assert!(
            v.is_axis() || v.is_diagonal(),
            "Orthogonal line must be axis-aligned or diagonal."
        );
        let p2 = p1.move_by(v);
        assert!(p2.is_some(), "Other vertex is out of bounds.");
        Self {
            vertices: [*p1, p2.unwrap()],
        }
    }

    /// Determines the type of the orthogonal line.
    ///
    /// # Returns
    ///
    /// An `OrthogonalLineType` indicating whether the line is axis-aligned or diagonal.
    pub fn is_type(&self) -> OrthogonalLine2DType {
        let vector = self.inherent_vector();
        if vector.is_axis() {
            OrthogonalLine2DType::Axis(self.get_axis().unwrap())
        } else {
            OrthogonalLine2DType::Diagonal
        }
    }

    /// Calculates the length of the line.
    ///
    /// # Returns
    ///
    /// The length.
    pub fn length(&self) -> u64 {
        *self
            .inherent_vector()
            .absolute_coordinates()
            .iter()
            .max()
            .unwrap()
            + 1
    }

    /// Returns the two points that define the line.
    ///
    /// # Returns
    ///
    /// A reference to the array containing the two vertices of the line
    pub fn get_vertexes(&self) -> &[Point<T, DIMENSIONS>; NUM_OF_VERTEXES_IN_LINE] {
        &self.vertices
    }

    /// Gets the dimension index of the axis that this line is aligned with.
    ///
    /// # Returns
    ///
    /// If `self.is_axis() == true`, it returns the index of the axis (0-based, and less than N)
    /// that the line is aligned with. `None` otherwise.
    pub fn get_axis(&self) -> Option<usize> {
        let vector = self.inherent_vector();
        if !vector.is_axis() {
            return None;
        }
        vector
            .get_coordinates()
            .iter()
            .position(|&coord| coord != 0)
    }

    pub fn contains_point(&self, point: &Point<T, DIMENSIONS>) -> bool {
        if self.vertices.contains(&point) {
            return true;
        }
        let inherent_vector = self.inherent_vector();
        let point_vector =
            Vector::<i128, DIMENSIONS>::from_points(&self.vertices[0], point).unwrap();
        if !point_vector.is_axis() && !point_vector.is_diagonal() {
            return false;
        }
        if !inherent_vector.is_collinear(&point_vector) {
            return false;
        }
        // Check if point is between the two endpoints for all coordinates
        self.vertices[0]
            .get_coordinates()
            .iter()
            .zip(self.vertices[1].get_coordinates().iter())
            .zip(point.get_coordinates().iter())
            .all(|((&a, &b), &p)| (a <= p && p <= b) || (b <= p && p <= a))
    }

    /// Creates an iterator that yields all points along the line from start to end.
    ///
    /// The iterator moves one unit at a time along the axis the line is aligned with.
    ///
    /// # Returns
    ///
    /// An iterator that yields `Point<T, DIMENSIONS>` values
    pub fn iter(&self) -> OrthogonalLine2DIterator<T> {
        let start = self.vertices[0];
        let end = self.vertices[1];
        let v = Vector::<i128, DIMENSIONS>::from_points(&start, &end)
            .unwrap()
            .normalize();
        OrthogonalLine2DIterator {
            current: start,
            end,
            direction: v.convert().unwrap(),
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
        let line_type_1 = self.is_type();
        let line_type_2 = other.is_type();
        match (line_type_1, line_type_2) {
            (OrthogonalLine2DType::Axis(axis1), OrthogonalLine2DType::Axis(axis2))
                if axis1 == axis2 =>
            {
                // Check for collinear overlap
                other
                    .vertices
                    .iter()
                    .any(|point| self.contains_point(point))
            }
            (OrthogonalLine2DType::Axis(axis1), OrthogonalLine2DType::Axis(axis2)) => {
                // Check for intersection at a single point
                let mut coords = [T::zero(); DIMENSIONS];
                coords[axis1] = *other.vertices[0].get(axis1);
                coords[axis2] = *self.vertices[0].get(axis2);
                let intersection = Point::<T, DIMENSIONS>::new(coords);
                self.contains_point(&intersection)
            }
            (OrthogonalLine2DType::Diagonal, OrthogonalLine2DType::Diagonal)
                if self.is_collinear(other) =>
            {
                // Check for collinear overlap
                other
                    .vertices
                    .iter()
                    .any(|point| self.contains_point(point))
            }
            (OrthogonalLine2DType::Diagonal, OrthogonalLine2DType::Diagonal)
            | (OrthogonalLine2DType::Axis(_), OrthogonalLine2DType::Diagonal)
            | (OrthogonalLine2DType::Diagonal, OrthogonalLine2DType::Axis(_)) => {
                self.iter().any(|p| other.contains_point(&p))
            }
        }
    }

    /// Gets the fixed coordinates of the line, i.e., the coordinates that do not change along the line.
    /// The changing coordinates will be `0`.
    fn extract_fixed_coordinates(&self) -> [T; DIMENSIONS] {
        let mut fixed_coords = [T::zero(); DIMENSIONS];
        let axis = self.get_axis().unwrap();
        for i in 0..DIMENSIONS {
            if i != axis {
                fixed_coords[i] = *self.vertices[0].get(i);
            }
        }
        fixed_coords
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
    pub fn intersect(&self, other: &Self) -> Vec<Point<T, DIMENSIONS>> {
        let line_type_1 = self.is_type();
        let line_type_2 = other.is_type();
        match (line_type_1, line_type_2) {
            (OrthogonalLine2DType::Axis(axis1), OrthogonalLine2DType::Axis(axis2))
                if axis1 == axis2 =>
            {
                let fixed_coord_1 = self.extract_fixed_coordinates();
                let fixed_coord_2 = other.extract_fixed_coordinates();
                if fixed_coord_1 != fixed_coord_2 {
                    return vec![];
                }
                let start1 = *self.vertices[0].get(axis1);
                let end1 = *self.vertices[1].get(axis1);
                let start2 = *other.vertices[0].get(axis2);
                let end2 = *other.vertices[1].get(axis2);
                let min_overlap =
                    std::cmp::max(std::cmp::min(start1, end1), std::cmp::min(start2, end2));
                let max_overlap =
                    std::cmp::min(std::cmp::max(start1, end1), std::cmp::max(start2, end2));
                if min_overlap > max_overlap {
                    vec![]
                } else if min_overlap == max_overlap {
                    let mut coords = fixed_coord_1;
                    coords[axis1] = min_overlap;
                    vec![Point::<T, DIMENSIONS>::new(coords)]
                } else {
                    // get points
                    let mut intersection_vertex1 = fixed_coord_1;
                    intersection_vertex1[axis1] = min_overlap;
                    let mut intersection_vertex2 = fixed_coord_1;
                    intersection_vertex2[axis1] = max_overlap;
                    let intersection_line = OrthogonalLine2D::from_points(
                        &Point::<T, DIMENSIONS>::new(intersection_vertex1),
                        &Point::<T, DIMENSIONS>::new(intersection_vertex2),
                    );
                    intersection_line.iter().collect()
                }
            }
            (OrthogonalLine2DType::Axis(axis1), OrthogonalLine2DType::Axis(axis2)) => {
                // Perpendicular case: check for intersection
                let mut coords = [T::zero(); DIMENSIONS];
                coords[axis1] = *other.vertices[0].get(axis1);
                coords[axis2] = *self.vertices[0].get(axis2);
                let intersection = Point::<T, DIMENSIONS>::new(coords);
                if self.contains_point(&intersection) && other.contains_point(&intersection) {
                    vec![Point::<T, DIMENSIONS>::new(coords)]
                } else {
                    vec![]
                }
            }
            (OrthogonalLine2DType::Diagonal, OrthogonalLine2DType::Diagonal)
                if self.is_collinear(other) =>
            {
                // Find the overlap between the two lines
                let points_self: Vec<_> = self.iter().collect();
                let points_other: Vec<_> = other.iter().collect();
                points_self
                    .into_iter()
                    .filter(|p| points_other.contains(p))
                    .collect()
            }
            (OrthogonalLine2DType::Diagonal, OrthogonalLine2DType::Diagonal)
            | (OrthogonalLine2DType::Axis(_), OrthogonalLine2DType::Diagonal)
            | (OrthogonalLine2DType::Diagonal, OrthogonalLine2DType::Axis(_)) => {
                if let Some(intersection) = self.iter().find(|p| other.contains_point(p)) {
                    vec![intersection]
                } else {
                    vec![]
                }
            }
        }
    }

    /// Checks if this line is collinear with another orthogonal line.
    /// Two lines are collinear if they lie on the same infinite line.
    ///
    /// # Arguments
    ///
    /// * `other` - The other orthogonal line to check for collinearity
    ///
    /// # Returns
    ///
    /// `true` if the lines are collinear, `false` otherwise
    pub fn is_collinear(&self, other: &Self) -> bool {
        /// Creates the smallest orthogonal line that contains both input lines.
        /// It assumes that the two lines are collinear.
        fn create_container_line<T: PointCoordinate>(
            line1: &OrthogonalLine2D<T>,
            line2: &OrthogonalLine2D<T>,
        ) -> Option<OrthogonalLine2D<T>> {
            let mut min_coords = [T::max_value(); DIMENSIONS];
            let mut max_coords = [T::min_value(); DIMENSIONS];
            for vertex in line1.vertices.iter().chain(line2.vertices.iter()) {
                for i in 0..DIMENSIONS {
                    min_coords[i] = std::cmp::min(min_coords[i], vertex[i]);
                    max_coords[i] = std::cmp::max(max_coords[i], vertex[i]);
                }
            }
            let start_point = Point::<T, DIMENSIONS>::new(min_coords);
            let end_point = Point::<T, DIMENSIONS>::new(max_coords);
            OrthogonalLine2D::try_from_points(&start_point, &end_point)
        }

        let v1 = self.inherent_vector();
        let v2 = other.inherent_vector();
        if !v1.is_collinear(&v2) {
            return false;
        }
        if let Some(super_line) = create_container_line(self, other)
            && super_line.is_type() == self.is_type()
        {
            self.vertices.iter().all(|p| super_line.contains_point(p))
                && other.vertices.iter().all(|p| super_line.contains_point(p))
        } else {
            false
        }
    }
}

/// Iterator for points along an orthogonal line.
///
/// This iterator yields all points from the start to the end of the line,
/// moving one unit at a time along the axis the line is aligned with.
pub struct OrthogonalLine2DIterator<T: PointCoordinate> {
    current: Point<T, DIMENSIONS>,
    end: Point<T, DIMENSIONS>,
    direction: Vector<i8, DIMENSIONS>,
    finished: bool,
}

impl<T: PointCoordinate> Iterator for OrthogonalLine2DIterator<T> {
    type Item = Point<T, DIMENSIONS>;

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
        self.current = self.current.move_by(&self.direction).unwrap();

        Some(current_point)
    }
}

impl<T: PointCoordinate> IntoIterator for OrthogonalLine2D<T> {
    type Item = Point<T, DIMENSIONS>;
    type IntoIter = OrthogonalLine2DIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: PointCoordinate> IntoIterator for &OrthogonalLine2D<T> {
    type Item = Point<T, DIMENSIONS>;
    type IntoIter = OrthogonalLine2DIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Display formatting for orthogonal lines.
///
/// Formats the line as "[(x,y),(x,y)]" for 2D, "[(x,y,z),(x,y,z)]" for 3D, etc.
impl<T: PointCoordinate> std::fmt::Display for OrthogonalLine2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let points = self.vertices.map(|c| c.to_string());
        write!(f, "[{}]", points.join(","))
    }
}
