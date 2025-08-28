/// Trait alias that defines all the requirements for interval value types.
///
/// This consolidates all the trait bounds needed for `T` in the `Interval<T>` type,
/// making it easier to maintain and ensuring consistency across the codebase.
pub trait IntervalValue:
    num_integer::Integer
    + num_traits::Signed
    + num_traits::Bounded
    + num_traits::NumCast
    + num_traits::CheckedSub
    + Copy
    + Ord
    + std::fmt::Display
    + std::hash::Hash
{
}

/// Blanket implementation for any type that satisfies all the required traits.
impl<T> IntervalValue for T where
    T: num_integer::Integer
        + num_traits::Signed
        + num_traits::Bounded
        + num_traits::NumCast
        + num_traits::CheckedSub
        + Copy
        + Ord
        + std::fmt::Display
        + std::hash::Hash
{
}

/// Returns the minimum value that can be used to create a valid interval.
///
/// # Returns
///
/// The minimum valid boundary value for intervals of type `T`
pub fn minimum_interval_value<T: IntervalValue>() -> T {
    T::min_value() + T::one()
}

/// Returns the maximum value that can be used to create a valid interval.
///
/// # Returns
///
/// The maximum valid boundary value for intervals of type `T`
pub fn maximum_interval_value<T: IntervalValue>() -> T {
    T::max_value()
}
