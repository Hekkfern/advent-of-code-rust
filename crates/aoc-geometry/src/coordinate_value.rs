/// Trait alias that defines all the requirements for coordinate value types.
///
/// This consolidates all the trait bounds needed for `T` in the `Point<T>` type,
/// making it easier to maintain and ensuring consistency across the codebase.
pub trait CoordinateValue:
    num::Integer
    + num::Signed
    + num::Bounded
    + num::NumCast
    + Copy
    + Ord
    + std::fmt::Display
    + std::hash::Hash
{
}

/// Blanket implementation for any type that satisfies all the required traits.
impl<T> CoordinateValue for T where
    T: num::Integer
        + num::Signed
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
pub fn minimum_coordinate_value<T: CoordinateValue>() -> T {
    T::min_value() + T::one()
}

/// Returns the maximum value that can be used to create a valid coordinate.
///
/// # Returns
///
/// The maximum valid boundary value for coordinates of type `T`
pub fn maximum_coordinate_value<T: CoordinateValue>() -> T {
    T::max_value()
}
