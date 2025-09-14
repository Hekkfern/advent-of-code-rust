#[cfg(test)]
mod vector_tests;

use crate::Point;
use crate::generic::core::point_coordinate::PointCoordinate;
use crate::generic::core::vector_coordinate::VectorCoordinate;
use num_traits::cast::cast;

/// Classification of vector types based on their properties.
///
/// This enum categorizes vectors into different types based on their coordinate values
/// and geometric properties.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VectorType {
    /// A vector with arbitrary coordinate values (general case).
    Arbitrary,
    /// A zero vector where all coordinates are zero.
    Zero,
    /// An axis-aligned vector where only one coordinate is non-zero.
    Axis,
}

/// A vector in N-dimensional space with coordinates of type T.
///
/// This struct represents a vector with N coordinates, where each coordinate
/// is of type T that must implement the `CoordinateValue` trait. The vector
/// supports various mathematical operations including addition, subtraction,
/// scaling, and geometric calculations like Manhattan distance.
///
/// # Type Parameters
///
/// * `T` - The numeric type for coordinates (must implement `CoordinateValue`)
/// * `N` - The number of dimensions (compile-time constant)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector<T: VectorCoordinate, const N: usize> {
    coordinates: [T; N],
}

impl<T: VectorCoordinate, const N: usize> Vector<T, N> {
    /// Creates a new zero vector (all coordinates are zero).
    ///
    /// # Returns
    ///
    /// A new `Vector` with all coordinates set to zero.
    pub fn zero() -> Self {
        Self {
            coordinates: [T::zero(); N],
        }
    }

    /// Creates a new vector with the specified coordinates.
    ///
    /// # Arguments
    ///
    /// * `coordinates` - An array of N coordinates of type T
    ///
    /// # Returns
    ///
    /// A new `Vector` with the given coordinates.
    pub fn new(coordinates: [T; N]) -> Self {
        Self { coordinates }
    }

    /// Creates a new vector where the origin is the coordinate (0,0) and the destination is the selected point.
    ///
    /// This method converts a point to a vector by treating the point's coordinates
    /// as the vector's displacement from the origin.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to convert to a vector
    ///
    /// # Returns
    ///
    /// A new `Vector` with coordinates matching the point's coordinates.
    pub fn from_point<U>(point: &Point<U, N>) -> Option<Self>
    where
        U: PointCoordinate,
    {
        Self::from_points(&Point::<U, N>::origin(), point)
    }

    /// Creates a new vector from two points, representing the displacement from origin to destination.
    ///
    /// # Arguments
    ///
    /// * `origin` - The starting point of the vector
    /// * `destination` - The ending point of the vector
    ///
    /// # Returns
    ///
    /// A new `Vector` representing the difference between the destination and origin points.
    pub fn from_points<U>(origin: &Point<U, N>, destination: &Point<U, N>) -> Option<Self>
    where
        U: PointCoordinate,
    {
        let mut coordinates = [T::zero(); N];
        for i in 0..N {
            coordinates[i] =
                cast::<U, T>(destination[i])?.checked_sub(&cast::<U, T>(origin[i])?)?;
        }
        Some(Vector { coordinates })
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

    /// Gets the absolute length of each coordinate.
    ///
    /// This method returns an array where each element is the absolute value
    /// of the corresponding coordinate, converted to u64.
    ///
    /// # Returns
    ///
    /// An array of absolute coordinate values as u64.
    pub fn absolute_coordinates(&self) -> [u64; N] {
        self.coordinates.map(|x| cast(x.abs()).unwrap())
    }

    /// Gets the maximum, absolute coordinate among all of them.
    ///
    /// This method finds the coordinate with the largest absolute value.
    /// Useful for calculating Chebyshev distance.
    ///
    /// # Returns
    ///
    /// The coordinate with the maximum absolute value.
    pub fn max_coordinate(&self) -> T {
        self.coordinates
            .iter()
            .map(num_traits::Signed::abs)
            .max()
            .unwrap()
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
        self.absolute_coordinates().iter().sum()
    }

    /// Converts a vector so the lengths becomes ones (positive or negative) at most, keeping the same direction.
    ///
    /// This method normalizes the vector by clamping each coordinate to the range [-1, 1],
    /// preserving the general direction while ensuring no coordinate exceeds unit magnitude.
    ///
    /// # Returns
    ///
    /// A new normalized `Vector` with coordinates clamped to [-1, 1].
    pub fn normalize(&self) -> Self {
        Vector::new(self.coordinates.map(|x| x.clamp(-T::one(), T::one())))
    }

    pub fn is_normalized(&self) -> bool {
        self.coordinates
            .iter()
            .all(|&x| x >= -T::one() && x <= T::one())
    }

    /// Checks if this is a zero vector (all coordinates are zero).
    ///
    /// # Returns
    ///
    /// `true` if all coordinates are zero, `false` otherwise.
    pub fn is_zero(&self) -> bool {
        self.coordinates.iter().all(|&x| x == T::zero())
    }

    /// Checks if this is an axis-aligned vector (only one coordinate is non-zero).
    ///
    /// # Returns
    ///
    /// `true` if exactly one coordinate is non-zero, `false` otherwise.
    pub fn is_axis(&self) -> bool {
        self.coordinates.iter().filter(|&&x| x != T::zero()).count() == 1
    }

    /// Determines the type of this vector based on its properties.
    ///
    /// # Returns
    ///
    /// A `Type` enum indicating whether the vector is Zero, Axis, or Arbitrary.
    pub fn is(&self) -> VectorType {
        if self.is_zero() {
            VectorType::Zero
        } else if self.is_axis() {
            VectorType::Axis
        } else {
            VectorType::Arbitrary
        }
    }

    /// Converts this vector to an axis-aligned vector, i.e., a vector that has only one non-zero coordinate.
    ///
    /// This method creates a new vector where only the coordinate at the specified index retains its value,
    /// while all other coordinates are set to zero.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the coordinate to retain (0-based)
    ///
    /// # Returns
    ///
    /// A new `Vector` with all coordinates set to zero except for the one at the specified index.
    pub fn to_axis(&self, index: usize) -> Self {
        assert!(index < N, "Axis index out of bounds");
        let mut coordinates = [T::zero(); N];
        coordinates[index] = self.coordinates[index];
        Vector::new(coordinates)
    }
}

/// Display formatting for vectors.
///
/// Formats the vector as "(x,y)" for 2D, "(x,y,z)" for 3D, etc.
impl<T: VectorCoordinate, const N: usize> std::fmt::Display for Vector<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let coords = self.coordinates.map(|c| c.to_string());
        write!(f, "({})", coords.join(","))
    }
}

/// Index operation for vectors.
///
/// Allows accessing coordinates using bracket notation.
impl<T: VectorCoordinate, const N: usize> std::ops::Index<usize> for Vector<T, N> {
    type Output = T;

    /// Gets the coordinate at the specified index.
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < N, "Axis index out of bounds");
        self.get(index)
    }
}

impl<T: VectorCoordinate, const N: usize> std::ops::Add for Vector<T, N> {
    type Output = Option<Self>;
    fn add(self, other: Self) -> Self::Output {
        let mut result = [T::zero(); N];
        for i in 0..N {
            result[i] = self.coordinates[i].checked_add(&other.coordinates[i])?;
        }
        Some(Vector::new(result))
    }
}

impl<T: VectorCoordinate, const N: usize> std::ops::Sub for Vector<T, N> {
    type Output = Option<Self>;
    fn sub(self, other: Self) -> Self::Output {
        let mut result = [T::zero(); N];
        for i in 0..N {
            result[i] = self.coordinates[i].checked_sub(&other.coordinates[i])?;
        }
        Some(Vector::new(result))
    }
}

impl<T: VectorCoordinate, const N: usize> std::ops::Neg for Vector<T, N> {
    type Output = Option<Self>;
    fn neg(self) -> Self::Output {
        let mut coordinates = [T::zero(); N];
        for i in 0..N {
            coordinates[i] = self.coordinates[i].checked_neg()?;
        }
        Some(Vector::new(coordinates))
    }
}

impl<T: VectorCoordinate, const N: usize> std::ops::Mul<i32> for Vector<T, N> {
    type Output = Option<Self>;
    fn mul(self, scalar: i32) -> Self::Output {
        let mut coordinates = [T::zero(); N];
        for i in 0..N {
            coordinates[i] = self.coordinates[i].checked_mul(&T::from(scalar)?)?;
        }
        Some(Vector::new(coordinates))
    }
}
