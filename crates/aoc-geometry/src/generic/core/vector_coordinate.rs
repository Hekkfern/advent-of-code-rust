/// Trait alias that defines all the requirements for coordinate value types for `Vector<T>`.
///
/// This consolidates all the trait bounds needed for `T` in the `Vector<T>` type,
/// making it easier to maintain and ensuring consistency across the codebase.
pub trait VectorCoordinate:
    num_integer::Integer
    + num_traits::Signed
    + num_traits::Bounded
    + num_traits::NumCast
    + num_traits::CheckedAdd
    + num_traits::CheckedSub
    + num_traits::CheckedMul
    + num_traits::CheckedNeg
    + Copy
    + Ord
    + std::fmt::Display
    + std::hash::Hash
{
}

/// Blanket implementation for any type that satisfies all the required traits.
impl<T> VectorCoordinate for T where
    T: num_integer::Integer
        + num_traits::Signed
        + num_traits::Bounded
        + num_traits::NumCast
        + num_traits::CheckedAdd
        + num_traits::CheckedSub
        + num_traits::CheckedMul
        + num_traits::CheckedNeg
        + Copy
        + Ord
        + std::fmt::Display
        + std::hash::Hash
{
}
