/// Trait alias that defines all the requirements for coordinate value types.
///
/// This consolidates all the trait bounds needed for `T` in the `Point<T>` type,
/// making it easier to maintain and ensuring consistency across the codebase.
pub trait PointCoordinate:
    num::Integer
    + num::Bounded
    + num::NumCast
    + Copy
    + Ord
    + std::fmt::Display
    + std::hash::Hash
{
}

/// Blanket implementation for any type that satisfies all the required traits.
impl<T> PointCoordinate for T where
    T: num::Integer
        + num::Bounded
        + num::NumCast
        + Copy
        + Ord
        + std::fmt::Display
        + std::hash::Hash
{
}

/// Returns the minimum value that can be used to create a valid coordinate.
///
/// # Returns
///
/// The minimum valid boundary value for coordinates of type `T`
pub fn minimum_point_coordinate_value<T: PointCoordinate>() -> T {
    T::min_value()
}

/// Returns the maximum value that can be used to create a valid coordinate.
///
/// # Returns
///
/// The maximum valid boundary value for coordinates of type `T`
pub fn maximum_point_coordinate_value<T: PointCoordinate>() -> T {
    T::max_value()
}
