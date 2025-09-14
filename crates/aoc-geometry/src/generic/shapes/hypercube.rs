use crate::BoundingBox;
use crate::Point;
use crate::PositionStatus;
use crate::generic::core::point_coordinate::PointCoordinate;
use crate::generic::core::vector_coordinate::VectorCoordinate;
use crate::{Vector, VectorType};
use std::collections::HashSet;

/// A hypercube in N-dimensional space with coordinates of type T.
///
/// # Type Parameters
///
/// * `T` - The numeric type for coordinates (must implement `CoordinateValue`)
/// * `N` - The number of dimensions (compile-time constant)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HyperCube<T: PointCoordinate, const N: usize> {
    min_vertex: Point<T, N>,
    max_vertex: Point<T, N>,
    sizes: [u64; N],
}

impl<T: PointCoordinate, const N: usize> HyperCube<T, N> {
    /// Creates a new hypercube from a vertex and a diagonal vector.
    ///
    /// The vertex is one corner of the hypercube, and the diagonal vector
    /// extends to the opposite corner.
    ///
    /// # Arguments
    ///
    /// * `vertex` - A point representing one vertex of the hypercube
    /// * `diagonal` - A vector representing the diagonal from the vertex to the opposite
    ///   vertex
    pub fn from_vertex_and_diagonal<U>(vertex: &Point<T, N>, diagonal: &Vector<U, N>) -> Self
    where
        U: VectorCoordinate,
    {
        assert_eq!(
            diagonal.is(),
            VectorType::Arbitrary,
            "Diagonal vector must have all its coordinates with non-zero values."
        );
        let vertex2_opt = vertex.move_by(diagonal);
        assert!(vertex2_opt.is_some(), "Other vertex is out of bounds.");
        let vertex2 = vertex2_opt.unwrap();
        let most_negative_point = Point::new([T::min_value(); N]);
        if Vector::<i128, N>::from_points(&most_negative_point, vertex)
            .unwrap()
            .manhattan_distance()
            < Vector::<i128, N>::from_points(&most_negative_point, &vertex2)
                .unwrap()
                .manhattan_distance()
        {
            Self {
                min_vertex: *vertex,
                max_vertex: vertex2,
                sizes: diagonal.absolute_coordinates(),
            }
        } else {
            Self {
                min_vertex: vertex2,
                max_vertex: *vertex,
                sizes: diagonal.absolute_coordinates(),
            }
        }
    }

    pub fn from_opposite_vertices(vertex1: &Point<T, N>, vertex2: &Point<T, N>) -> Self {
        let diagonal = Vector::<i128, N>::from_points(vertex1, vertex2)
            .expect("Diagonal vector cannot be created");
        HyperCube::from_vertex_and_diagonal(vertex1, &diagonal)
    }

    pub fn get_sizes(&self) -> [u64; N] {
        self.sizes
    }

    pub fn area(&self) -> u64 {
        self.get_sizes().iter().product()
    }

    pub fn is_outside(&self, point: &Point<T, N>) -> bool {
        let mut bounding_box: BoundingBox<T, N> = BoundingBox::new();
        bounding_box.update(&self.min_vertex);
        bounding_box.update(&self.max_vertex);
        bounding_box.is_outside(point)
    }

    pub fn is_on_border(&self, point: &Point<T, N>) -> bool {
        (0..N).any(|i| {
            point.get(i) == self.min_vertex.get(i) || point.get(i) == self.max_vertex.get(i)
        })
    }

    pub fn is_inside(&self, point: &Point<T, N>) -> bool {
        !self.is_outside(point) && !self.is_on_border(point)
    }

    pub fn check_position(&self, point: &Point<T, N>) -> PositionStatus {
        if self.is_outside(point) {
            PositionStatus::Outside
        } else if self.is_on_border(point) {
            PositionStatus::OnBorder
        } else {
            PositionStatus::Inside
        }
    }

    pub fn get_all_vertices(&self) -> HashSet<Point<T, N>> {
        let mut vertices: HashSet<Point<T, N>> = HashSet::with_capacity(2_usize.pow(N as u32));
        vertices.insert(self.min_vertex);
        for i in 0..N {
            let mut coords = *self.min_vertex.get_coordinates();
            coords[i] = *self.max_vertex.get(i);
            vertices.insert(Point::new(coords));
        }
        vertices.insert(self.max_vertex);
        for i in 0..N {
            let mut coords = *self.max_vertex.get_coordinates();
            coords[i] = *self.min_vertex.get(i);
            vertices.insert(Point::new(coords));
        }
        vertices
    }
}
