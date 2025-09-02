use crate::AxisDirection;
use crate::Point;
use crate::generic::core::point_coordinate::PointCoordinate;
use crate::PositionStatus;

/// A multidimensional bounding box that tracks the minimum and maximum coordinates across all dimensions.
///
/// The bounding box is used to efficiently determine the spatial extent of a collection of points
/// in N-dimensional space. It maintains the minimum and maximum values for each coordinate axis.
///
/// # Type Parameters
///
/// * `T` - The type of coordinate values.
/// * `N` - The number of dimensions (must be known at compile time).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoundingBox<T: PointCoordinate, const N: usize> {
    /// The minimum values for each coordinate axis.
    minimums: [T; N],
    /// The maximum values for each coordinate axis.
    maximums: [T; N],
}

impl<T: PointCoordinate, const N: usize> Default for BoundingBox<T, N> {
    fn default() -> Self {
        Self {
            minimums: [T::max_value(); N],
            maximums: [T::min_value(); N],
        }
    }
}

impl<T: PointCoordinate, const N: usize> BoundingBox<T, N> {
    /// Creates a new empty bounding box.
    ///
    /// The bounding box is initialized with minimum values set to the maximum possible value
    /// and maximum values set to the minimum possible value. This ensures that the first
    /// point added will properly initialize the bounds.
    ///
    /// # Returns
    ///
    /// A new empty `BoundingBox`
    pub fn new() -> Self {
        Self::default()
    }

    /// Updates the bounding box to include the given point.
    ///
    /// For each coordinate axis, the minimum and maximum values are updated
    /// if the point's coordinates extend beyond the current bounds.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to include in the bounding box
    pub fn update(&mut self, point: &Point<T, N>) {
        for i in 0..N {
            self.minimums[i] = self.minimums[i].min(*point.get(i));
            self.maximums[i] = self.maximums[i].max(*point.get(i));
        }
    }

    /// Returns the minimum value for the specified coordinate axis.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the coordinate axis (0-based)
    ///
    /// # Returns
    ///
    /// A reference to the minimum value for the specified axis
    pub fn get_minimum(&self, index: usize) -> &T {
        assert!(index < N, "Axis index out of bounds");
        &self.minimums[index]
    }

    /// Returns the maximum value for the specified coordinate axis.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the coordinate axis (0-based)
    ///
    /// # Returns
    ///
    /// A reference to the maximum value for the specified axis
    pub fn get_maximum(&self, index: usize) -> &T {
        assert!(index < N, "Axis index out of bounds");
        &self.maximums[index]
    }

    /// Checks if a point is outside the bounding box.
    ///
    /// Returns `true` if the point lies outside the bounding box on any axis.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to check
    ///
    /// # Returns
    ///
    /// `true` if the point is outside the bounding box, `false` otherwise
    pub fn is_outside(&self, point: &Point<T, N>) -> bool {
        for i in 0..N {
            if self.is_outside_for_axis(i, point) {
                return true;
            }
        }
        false
    }

    pub fn is_on_border(&self, point: &Point<T, N>) -> bool {
        (0..N).any(|i| *point.get(i) == self.minimums[i] || *point.get(i) == self.maximums[i])
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

    /// Checks if a point is outside the bounding box on a specific axis.
    ///
    /// Returns `true` if the point's coordinate on the specified axis is outside
    /// the corresponding bounds of the bounding box.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the coordinate axis to check (0-based)
    /// * `point` - The point to check
    ///
    /// # Returns
    ///
    /// `true` if the point is outside the bounds on the specified axis, `false` otherwise
    pub fn is_outside_for_axis(&self, index: usize, point: &Point<T, N>) -> bool {
        assert!(index < N, "Axis index out of bounds");
        let coordinate = *point.get(index);
        coordinate < self.minimums[index] || coordinate > self.maximums[index]
    }

    /// Checks if a point is outside the bounding box on a specific axis in a specific direction.
    ///
    /// Returns `true` if the point's coordinate on the specified axis exceeds the bound
    /// in the given direction.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the coordinate axis to check (0-based)
    /// * `direction` - The direction to check (`Positive` or `Negative`)
    /// * `point` - The point to check
    ///
    /// # Returns
    ///
    /// `true` if the point exceeds the bound in the specified direction, `false` otherwise
    pub fn is_outside_for_axis_and_direction(
        &self,
        index: usize,
        direction: AxisDirection,
        point: &Point<T, N>,
    ) -> bool {
        assert!(index < N, "Axis index out of bounds");
        let coordinate = *point.get(index);
        match direction {
            AxisDirection::Positive => coordinate > self.maximums[index],
            AxisDirection::Negative => coordinate < self.minimums[index],
        }
    }

    /// Resets the bounding box to its initial empty state.
    ///
    /// After calling this method, the bounding box will be empty and ready to
    /// accumulate new points.
    pub fn reset(&mut self) {
        self.minimums = [T::max_value(); N];
        self.maximums = [T::min_value(); N];
    }
}
