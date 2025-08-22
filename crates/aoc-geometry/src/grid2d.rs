#[cfg(test)]
mod tests;

use crate::position_status::PositionStatus;
use std::collections::HashSet;

const DIMENSIONS: usize = 2;

/// An 2-D grid of values.
///
/// The positions in the grid are based on the origin point (0,0) and the sizes of each dimension.
///
/// # Type Parameters
///
/// * `ValueType` - The type of values stored in the grid
#[derive(Debug, PartialEq, Eq)]
pub struct Grid2D<ValueType> {
    data: ndarray::Array2<ValueType>,
}

impl<ValueType> Grid2D<ValueType> {
    /// Creates a new grid with the specified dimensions, filling with a default value.
    ///
    /// # Arguments
    ///
    /// * `sizes` - An array specifying the size along each dimension
    /// * `default_value` - The value to fill the grid with
    ///
    /// # Returns
    ///
    /// A new `Grid` with the specified dimensions filled with the default value.
    pub fn from_default_value(sizes: &[usize; DIMENSIONS], default_value: &ValueType) -> Self
    where
        ValueType: Clone,
    {
        assert!(
            sizes.iter().all(|s| *s > 0),
            "All dimensions must be greater than zero"
        );
        Self {
            data: ndarray::Array2::from_elem(*sizes, default_value.clone()),
        }
    }

    /// Creates a new grid filled with the specified vector data.
    ///
    /// # Arguments
    ///
    /// * `grid_data` - A 1-D vector representing the grid data
    ///
    /// # Returns
    ///
    /// A new `Grid` constructed filled with the vector data.
    pub fn from_vec(sizes: &[usize; DIMENSIONS], grid_data: Vec<ValueType>) -> Self {
        assert!(
            sizes.iter().all(|s| *s > 0),
            "All dimensions must be greater than zero"
        );
        let total_size = sizes.iter().product();
        assert_eq!(
            grid_data.len(),
            total_size,
            "Grid data length does not match specified dimensions"
        );
        Self {
            data: ndarray::Array2::from_shape_vec(*sizes, grid_data).unwrap(),
        }
    }

    /// Gets the size along the specified axis.
    ///
    /// # Arguments
    ///
    /// * `axis` - The axis index (0-based)
    ///
    /// # Returns
    ///
    /// The size along the specified axis.
    pub fn get_size(&self, index: usize) -> usize {
        assert!(index < DIMENSIONS, "Axis index out of bounds");
        self.data.shape()[index]
    }

    /// Gets the sizes for every dimension as an array.
    ///
    /// # Returns
    ///
    /// A reference to the dimensions array.
    pub fn get_sizes(&self) -> [usize; DIMENSIONS] {
        let sh = self.data.shape();
        let mut array = [Default::default(); DIMENSIONS];
        array.copy_from_slice(sh);
        array
    }

    /// Gets the total number of elements in the grid.
    ///
    /// # Returns
    ///
    /// The total size of the grid.
    pub fn get_number_of_elements(&self) -> usize {
        self.data.len()
    }

    /// Accesses the element at the specified coordinates.
    ///
    /// # Arguments
    ///
    /// * `coords` - The N-dimensional coordinates
    ///
    /// # Returns
    ///
    /// A reference to the element at the specified position, or `None` if out of bounds.
    pub fn get(&self, coords: &[usize; DIMENSIONS]) -> Option<&ValueType> {
        self.data.get(*coords)
    }

    /// Mutably accesses the element at the specified coordinates.
    ///
    /// # Arguments
    ///
    /// * `coords` - The N-dimensional coordinates
    ///
    /// # Returns
    ///
    /// A mutable reference to the element at the specified position, or `None` if out of bounds.
    pub fn get_mut(&mut self, coords: &[usize; DIMENSIONS]) -> Option<&mut ValueType> {
        self.data.get_mut(*coords)
    }

    /// Sets the value at the specified coordinates.
    ///
    /// # Arguments
    ///
    /// * `coords` - The N-dimensional coordinates
    /// * `value` - The value to set
    ///
    /// # Returns
    ///
    /// `true` if the value was set successfully, `false` if coordinates are out of bounds.
    pub fn set(&mut self, coords: &[usize; DIMENSIONS], value: ValueType) -> bool {
        if let Some(element) = self.data.get_mut(*coords) {
            *element = value;
            return true;
        }
        false
    }

    /// Determines whether the given coordinates are part of the grid.
    ///
    /// # Arguments
    ///
    /// * `point` - The N-dimensional coordinates
    ///
    /// # Returns
    ///
    /// True if the point is within the grid bounds, false otherwise.
    pub fn contains(&self, coords: &[usize; DIMENSIONS]) -> bool {
        let shape = self.data.shape();
        if coords.len() != shape.len() {
            return false;
        }
        coords.iter().zip(shape.iter()).all(|(&i, &dim)| i < dim)
    }

    /// Rotates the grid 90 degrees counter-clockwise.
    ///
    /// # Panics
    ///
    /// Panics if the axis index is out of bounds.
    pub fn rotate_counter_clockwise(&mut self) {
        self.data.swap_axes(0, 1);
        self.data.invert_axis(ndarray::Axis(0));
    }

    pub fn rotate_clockwise(&mut self) {
        self.data.swap_axes(0, 1);
        self.data.invert_axis(ndarray::Axis(1));
    }

    /// Flips the grid horizontally (left to right).
    pub fn flip_horizontal(&mut self) {
        self.data.invert_axis(ndarray::Axis(1));
    }

    /// Flips the grid vertically (top to bottom).
    pub fn flip_vertical(&mut self) {
        self.data.invert_axis(ndarray::Axis(0));
    }

    /// Finds the first occurrence of a value in the grid.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to search for
    ///
    /// # Returns
    ///
    /// The coordinates of the first occurrence, or `None` if not found.
    pub fn find_first(&self, value: &ValueType) -> Option<[usize; DIMENSIONS]>
    where
        ValueType: PartialEq,
    {
        for ((row, col), v) in self.data.indexed_iter() {
            if v == value {
                return Some([row, col]);
            }
        }
        None
    }

    /// Finds all occurrences of a value in the grid.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to search for
    ///
    /// # Returns
    ///
    /// A vector of coordinates for each occurrence of the value.
    pub fn find_all(&self, value: &ValueType) -> Vec<[usize; DIMENSIONS]>
    where
        ValueType: PartialEq,
    {
        self.data
            .indexed_iter()
            .filter_map(|((row, col), v)| if v == value { Some([row, col]) } else { None })
            .collect()
    }

    pub fn subgrid(&self, start: &[usize; DIMENSIONS], end: &[usize; DIMENSIONS]) -> Option<Self>
    where
        ValueType: Clone,
    {
        // Validate bounds
        if self.is_outside(start) || self.is_outside(end) {
            return None;
        }
        for i in 0..DIMENSIONS {
            if start[i] > end[i] || end[i] >= self.get_size(i) {
                return None;
            }
        }
        let rows = start[0]..=end[0];
        let cols = start[1]..=end[1];
        let view = self.data.slice(ndarray::s![rows, cols]);
        let sub_data = view.to_owned();
        Some(Self { data: sub_data })
    }

    /// Checks if a point is outside the grid bounds.
    pub fn is_outside(&self, coords: &[usize; DIMENSIONS]) -> bool {
        for i in 0..DIMENSIONS {
            if coords[i] >= self.get_size(i) {
                return true;
            }
        }
        false
    }

    /// Resizes the grid to new, bigger dimensions, filling new spaces with a default value.
    ///
    /// # Arguments
    ///
    /// * `new_dimensions` - The new dimensions for the grid. It must be equal to or larger than the current dimensions.
    /// * `default_value` - The value to fill new cells with
    pub fn expand(&mut self, new_dimensions: &[usize; DIMENSIONS], default_value: &ValueType)
    where
        ValueType: Clone,
    {
        assert!(
            new_dimensions
                .iter()
                .zip(self.data.shape())
                .all(|(&new_dim, &old_dim)| new_dim >= old_dim),
            "All dimensions must be equal or greater than the old ones"
        );
        let mut new_data = ndarray::Array2::from_elem(*new_dimensions, default_value.clone());
        let min_dims = self.data.shape();
        for i in 0..DIMENSIONS {
            let min_size = min_dims[i].min(new_dimensions[i]);
            for j in 0..min_size {
                new_data
                    .slice_mut(ndarray::s![.., j])
                    .assign(&self.data.slice(ndarray::s![.., j]));
            }
        }
        self.data = new_data;
    }

    /// Gets all valid neighboring coordinates for a given point.
    ///
    /// This method returns all coordinates that are adjacent to the given point
    /// (differing by ±1 in exactly one coordinate) and are within grid bounds.
    ///
    /// # Arguments
    ///
    /// * `coord` - The point to get neighbors for
    ///
    /// # Returns
    ///
    /// A list of valid neighboring coordinates.
    pub fn get_neighbors(&self, coord: &[usize; DIMENSIONS]) -> HashSet<[usize; DIMENSIONS]> {
        let mut neighbors = HashSet::new();
        for axis in 0..DIMENSIONS {
            for &delta in &[-1isize, 1] {
                let mut neighbor = *coord;
                let val = coord[axis] as isize + delta;
                if val >= 0 && (val as usize) < self.get_size(axis) {
                    neighbor[axis] = val as usize;
                    neighbors.insert(neighbor);
                }
            }
        }
        neighbors
    }

    /// Fills the entire grid with a single value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to fill the grid with
    pub fn fill(&mut self, value: &ValueType)
    where
        ValueType: Clone,
    {
        self.data.fill(value.clone());
    }

    /// Applies a function to transform all elements in the grid.
    ///
    /// # Arguments
    ///
    /// * `f` - A function that takes a value and returns a transformed value
    pub fn map<F, U>(&self, f: F) -> Grid2D<U>
    where
        ValueType: Clone,
        F: Fn(&ValueType) -> U,
        U: Clone,
    {
        let new_data = self.data.mapv(|v| f(&v));
        Grid2D { data: new_data }
    }

    /// Returns an iterator over all values in the grid.
    ///
    /// # Returns
    ///
    /// An iterator over references to the values in the grid.
    pub fn iter(&self) -> ndarray::iter::Iter<ValueType, ndarray::Ix2> {
        self.data.iter()
    }

    /// Returns a mutable iterator over all values in the grid.
    ///
    /// # Returns
    ///
    /// A mutable iterator over references to the values in the grid.
    pub fn iter_mut(&mut self) -> ndarray::iter::IterMut<ValueType, ndarray::Ix2> {
        self.data.iter_mut()
    }

    /// Determines the position status of given coordinates in the grid.
    ///
    /// It checks whether the given coordinates are inside the grid, on the border, or outside the grid bounds.
    ///
    /// # Arguments
    ///
    /// * `point` - The N-dimensional coordinates to check
    ///
    /// # Returns
    ///
    /// A `PositionStatus` enum indicating the status of the coordinates:
    /// - `Inside`: coordinates are within the grid and not on any border
    /// - `OnBorder`: coordinates are on the edge/border of the grid
    /// - `Outside`: coordinates are outside the grid bounds
    pub fn position_status(&self, coord: &[usize; DIMENSIONS]) -> PositionStatus {
        // Check if outside
        for i in 0..DIMENSIONS {
            if coord[i] >= self.get_size(i) {
                return PositionStatus::Outside;
            }
        }
        // Check if on the border
        for i in 0..DIMENSIONS {
            if coord[i] == 0 || coord[i] == self.get_size(i) - 1 {
                return PositionStatus::OnBorder;
            }
        }
        // Otherwise, inside
        PositionStatus::Inside
    }

    fn is_array_unary(arr: &[usize; DIMENSIONS]) -> bool {
        let ones = arr.iter().filter(|&&x| x == 1).count();
        let zeros = arr.iter().filter(|&&x| x == 0).count();
        ones == 1 && zeros == arr.len() - 1
    }

    /// Attempts to move a position in the grid according to a given direction.
    ///
    /// # Arguments
    ///
    /// * `position` - The current position in the grid as an array of coordinates.
    /// * `direction` - The direction to move, represented as a `Direction2D
    ///
    /// # Returns
    ///
    /// `Some(new_coords)` if the move is valid, or `None` if out of bounds.
    pub fn try_move(
        &self,
        position: &[usize; DIMENSIONS],
        direction: &[usize; DIMENSIONS],
    ) -> Option<[usize; DIMENSIONS]> {
        assert!(
            Self::is_array_unary(direction),
            "Direction must be a unit vector in one dimension"
        );
        // Check if current position is outside
        if self.is_outside(position) {
            return None;
        }
        // Calculate new coordinates based on the direction
        let mut new_coords = [0usize; DIMENSIONS];
        for i in 0..DIMENSIONS {
            let val: isize = position[i] as isize + direction[i] as isize;
            // Check if the new coordinate is within bounds
            if val < 0 || val >= self.get_size(i) as isize {
                return None;
            }
            new_coords[i] = val as usize;
        }
        Some(new_coords)
    }
}

/// Index operation for grids using Point coordinates.
impl<ValueType> std::ops::Index<&[usize; DIMENSIONS]> for Grid2D<ValueType> {
    type Output = ValueType;

    /// Gets the element at the specified coordinates.
    ///
    /// # Panics
    ///
    /// Panics if the coordinates are out of bounds.
    fn index(&self, coord: &[usize; DIMENSIONS]) -> &Self::Output {
        self.get(coord).expect("Index out of bounds")
    }
}

/// Mutable index operation for grids using Point coordinates.
impl<ValueType> std::ops::IndexMut<&[usize; DIMENSIONS]> for Grid2D<ValueType> {
    /// Gets a mutable reference to the element at the specified coordinates.
    ///
    /// # Panics
    ///
    /// Panics if the coordinates are out of bounds.
    fn index_mut(&mut self, point: &[usize; DIMENSIONS]) -> &mut Self::Output {
        self.get_mut(point).expect("Index out of bounds")
    }
}
