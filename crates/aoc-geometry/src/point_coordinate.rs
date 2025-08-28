/// Trait alias that defines all the requirements for coordinate value types.
///
/// This consolidates all the trait bounds needed for `T` in the `Point<T>` type,
/// making it easier to maintain and ensuring consistency across the codebase.
pub trait PointCoordinate:
    num_integer::Integer
    + num_traits::Bounded
    + num_traits::NumCast
    + num_traits::CheckedAdd
    + num_traits::CheckedSub
    + Copy
    + Ord
    + std::fmt::Display
    + std::hash::Hash
{
}

/// Blanket implementation for any type that satisfies all the required traits.
impl<T> PointCoordinate for T where
    T: num_integer::Integer
        + num_traits::Bounded
        + num_traits::NumCast
        + num_traits::CheckedAdd
        + num_traits::CheckedSub
        + Copy
        + Ord
        + std::fmt::Display
        + std::hash::Hash
{
}
