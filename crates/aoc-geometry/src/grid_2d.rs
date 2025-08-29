#[cfg(test)]
mod tests;

use crate::point::Point;
use crate::position_status::PositionStatus;
use crate::vector::Vector;
use std::collections::HashSet;

type Coordinate = Point<usize, 2>;

const DIMENSIONS: usize = 2;
const ROW_INDEX: usize = 0;
const COL_INDEX: usize = 1;

/// An 2-D grid of values.
///
/// The positions in the grid are based on the origin point (0,0) and the sizes of each dimension.
///
/// # Type Parameters
///
/// * `ValueType` - The type of values stored in the grid
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Grid2D<ValueType> {
    /// The underlying data storage for the grid. It stores data in column-major order, meaning the
    /// coordinates are (x, y) or (col, row).
    data: ndarray::Array2<ValueType>,
}

impl<ValueType> Grid2D<ValueType> {
    /// Creates a new grid with the specified dimensions, filling with a default value.
    ///
    /// # Arguments
    ///
    /// * `sizes` - An array specifying the size along each dimension.  All sizes must be greater than zero.
    /// * `default_value` - The value to fill the grid with
    ///
    /// # Returns
    ///
    /// A new `Grid` with the specified dimensions filled with the default value.
    pub fn from_default_value(width: usize, height: usize, default_value: &ValueType) -> Self
    where
        ValueType: Clone,
    {
        assert!(
            width > 0 && height > 0,
            "All dimensions must be greater than zero"
        );
        Self {
            data: ndarray::Array2::from_elem((height, width), default_value.clone()),
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
    /// A new `Grid` constructed filled with the 1-D vector data.
    pub fn from_single_vec(width: usize, height: usize, grid_data: Vec<ValueType>) -> Self {
        assert!(
            width > 0 && height > 0,
            "All dimensions must be greater than zero"
        );
        assert_eq!(
            grid_data.len(),
            width * height,
            "Grid data length does not match specified dimensions"
        );
        Self {
            data: ndarray::Array2::from_shape_vec((height, width), grid_data).unwrap(),
        }
    }

    /// Creates a new grid filled with the specified 2-D vector data.
    ///
    /// # Arguments
    ///
    /// * `grid_data` - A 2-D vector representing the grid data
    ///
    /// # Returns
    ///
    /// A new `Grid` constructed filled with the 2-D vector data.
    pub fn from_double_vec(grid_data: Vec<Vec<ValueType>>) -> Self {
        assert!(
            !grid_data.is_empty() && !grid_data[0].is_empty(),
            "Grid data cannot be empty"
        );
        let height = grid_data.len();
        let width = grid_data[0].len();
        assert!(
            grid_data.iter().all(|row| row.len() == width),
            "All rows must have the same number of columns"
        );
        let flat_data: Vec<ValueType> = grid_data.into_iter().flatten().collect();
        Self {
            data: ndarray::Array2::from_shape_vec((height, width), flat_data).unwrap(),
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
    pub fn get_width(&self) -> usize {
        self.data.shape()[COL_INDEX]
    }

    pub fn get_height(&self) -> usize {
        self.data.shape()[ROW_INDEX]
    }

    /// Gets the sizes for width and height.
    ///
    /// # Returns
    ///
    /// A tuple whose first item is the width and second item is the height.
    pub fn get_sizes(&self) -> (usize, usize) {
        let sh = self.data.shape();
        (sh[COL_INDEX], sh[ROW_INDEX])
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
    pub fn get(&self, coords: &Coordinate) -> Option<&ValueType> {
        self.data.get((coords[1], coords[0]))
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
    pub fn get_mut(&mut self, coords: &Coordinate) -> Option<&mut ValueType> {
        self.data.get_mut((coords[1], coords[0]))
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
    pub fn set(&mut self, coords: &Coordinate, value: &ValueType) -> bool
    where
        ValueType: Clone,
    {
        if let Some(element) = self.get_mut(coords) {
            *element = value.clone();
            return true;
        }
        false
    }

    /// Determines whether the given coordinates are part of the grid (by being inside or on the border).
    ///
    /// # Arguments
    ///
    /// * `coord` - The N-dimensional coordinates
    ///
    /// # Returns
    ///
    /// True if the point is within the grid bounds, false otherwise.
    ///
    /// # Notes
    ///
    /// * This function is the inverse of `is_outside()`.
    pub fn contains(&self, coords: &Coordinate) -> bool {
        coords[1] < self.get_width() && coords[0] < self.get_height()
    }

    pub fn is_on_border(&self, coords: &Coordinate) -> bool {
        ((coords[0] == 0 || coords[0] == self.get_width() - 1) && coords[1] < self.get_height())
            || ((coords[1] == 0 || coords[1] == self.get_height() - 1)
                && coords[0] < self.get_width())
    }

    /// Determines whether the given coordinates are outside the grid.
    ///
    /// # Arguments
    ///
    /// * `coord` - The N-dimensional coordinates
    ///
    /// # Returns
    ///
    /// True if the point is within the grid bounds, false otherwise.
    ///
    /// # Notes
    ///
    /// * This function is the inverse of `contains()`.
    pub fn is_outside(&self, coords: &Coordinate) -> bool {
        !self.contains(coords)
    }

    /// Determines the position status of given coordinates in the grid.
    ///
    /// It checks whether the given coordinates are inside the grid, on the border, or outside the grid bounds.
    ///
    /// # Arguments
    ///
    /// * `coord` - The N-dimensional coordinates to check
    ///
    /// # Returns
    ///
    /// A `PositionStatus` enum indicating the status of the coordinates:
    /// - `Inside`: coordinates are within the grid and not on any border
    /// - `OnBorder`: coordinates are on the edge/border of the grid
    /// - `Outside`: coordinates are outside the grid bounds
    pub fn position_status(&self, coords: &Coordinate) -> PositionStatus {
        if self.is_outside(coords) {
            PositionStatus::Outside
        } else if self.is_on_border(coords) {
            PositionStatus::OnBorder
        } else {
            PositionStatus::Inside
        }
    }

    /// Rotates the grid 90 degrees counter-clockwise.
    pub fn rotate_counter_clockwise(&mut self) {
        self.data.swap_axes(0, 1);
        self.data.invert_axis(ndarray::Axis(1));
    }

    /// Rotates the grid 90 degrees clockwise.
    pub fn rotate_clockwise(&mut self) {
        self.data.swap_axes(0, 1);
        self.data.invert_axis(ndarray::Axis(0));
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
    pub fn find_first(&self, value: &ValueType) -> Option<Coordinate>
    where
        ValueType: PartialEq,
    {
        for ((row, col), v) in self.data.indexed_iter() {
            if v == value {
                return Some(Coordinate::new([col, row]));
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
    pub fn find_all(&self, value: &ValueType) -> Vec<Coordinate>
    where
        ValueType: PartialEq,
    {
        self.data
            .indexed_iter()
            .filter_map(|((row, col), v)| {
                if v == value {
                    Some(Coordinate::new([col, row]))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn subgrid(&self, start: &Coordinate, end: &Coordinate) -> Self
    where
        ValueType: Clone,
    {
        assert!(self.contains(start), "Start coordinate out of bounds");
        assert!(self.contains(end), "End coordinate out of bounds");
        assert_ne!(start, end, "Start and end coordinates must be different");
        let rows = start[0]..=end[0];
        let cols = start[1]..=end[1];
        let view = self.data.slice(ndarray::s![rows, cols]);
        let sub_data = view.to_owned();
        Self { data: sub_data }
    }

    /// Resizes the grid to new, bigger dimensions, filling new spaces with a default value.
    ///
    /// # Arguments
    ///
    /// * `new_dimensions` - The new dimensions for the grid. It must be equal to or larger than the current dimensions.
    /// * `default_value` - The value to fill new cells with
    pub fn expand(
        &mut self,
        new_dimensions: &[usize; DIMENSIONS],
        start: &Coordinate,
        default_value: &ValueType,
    ) where
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
    pub fn get_neighbors(&self, coords: &Coordinate) -> HashSet<Coordinate> {
        let mut neighbors = HashSet::new();
        let directions = [
            Vector::<i8, 2>::new([0, 1]),
            Vector::<i8, 2>::new([0, -1]),
            Vector::<i8, 2>::new([1, 0]),
            Vector::<i8, 2>::new([-1, 0]),
        ];
        for dir in directions.iter() {
            if let Some(neighbor) = coords.move_by(dir) {
                neighbors.insert(neighbor);
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
    pub fn try_move(&self, position: &Coordinate, direction: &Vector<i8, 2>) -> Option<Coordinate> {
        assert!(self.contains(position), "Current position is out of bounds");
        assert!(
            direction.is_normalized() && direction.is_axis(),
            "Direction must be normalized and along an axis"
        );
        // Calculate new coordinates based on the direction
        let mut new_coords = [0usize; DIMENSIONS];
        {
            let xi = position[0] as isize + direction[0] as isize;
            if xi < 0 || xi >= self.get_width() as isize {
                return None;
            }
            new_coords[0] = xi as usize;
        }
        {
            let yi = position[1] as isize + direction[1] as isize;
            if yi < 0 || yi >= self.get_height() as isize {
                return None;
            }
            new_coords[1] = yi as usize;
        }
        Some(Coordinate::new(new_coords))
    }
}

/// Index operation for grids using Point coordinates.
impl<ValueType> std::ops::Index<&Coordinate> for Grid2D<ValueType> {
    type Output = ValueType;

    /// Gets the element at the specified coordinates.
    ///
    /// # Panics
    ///
    /// Panics if the coordinates are out of bounds.
    fn index(&self, coords: &Coordinate) -> &Self::Output {
        self.get(coords).expect("Index out of bounds")
    }
}

/// Mutable index operation for grids using Point coordinates.
impl<ValueType> std::ops::IndexMut<&Coordinate> for Grid2D<ValueType> {
    /// Gets a mutable reference to the element at the specified coordinates.
    ///
    /// # Panics
    ///
    /// Panics if the coordinates are out of bounds.
    fn index_mut(&mut self, coords: &Coordinate) -> &mut Self::Output {
        self.get_mut(coords).expect("Index out of bounds")
    }
}
