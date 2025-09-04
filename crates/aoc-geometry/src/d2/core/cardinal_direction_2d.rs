use crate::Vector;
use crate::generic::core::vector_coordinate::VectorCoordinate;

const DIMENSIONS: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardinalDirection2D {
    Up,
    Down,
    Left,
    Right,
}

impl CardinalDirection2D {
    pub fn rotate_clockwise(self) -> Self {
        match self {
            CardinalDirection2D::Up => CardinalDirection2D::Right,
            CardinalDirection2D::Right => CardinalDirection2D::Down,
            CardinalDirection2D::Down => CardinalDirection2D::Left,
            CardinalDirection2D::Left => CardinalDirection2D::Up,
        }
    }

    pub fn rotate_counter_clockwise(self) -> Self {
        match self {
            CardinalDirection2D::Up => CardinalDirection2D::Left,
            CardinalDirection2D::Left => CardinalDirection2D::Down,
            CardinalDirection2D::Down => CardinalDirection2D::Right,
            CardinalDirection2D::Right => CardinalDirection2D::Up,
        }
    }

    pub fn to_vector<T>(&self) -> Vector<T, DIMENSIONS>
    where
        T: VectorCoordinate,
    {
        match self {
            CardinalDirection2D::Up => Vector::new([T::zero(), T::one()]),
            CardinalDirection2D::Down => Vector::new([T::zero(), -T::one()]),
            CardinalDirection2D::Left => Vector::new([-T::one(), T::zero()]),
            CardinalDirection2D::Right => Vector::new([T::one(), T::zero()]),
        }
    }
}

impl std::ops::Neg for CardinalDirection2D {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            CardinalDirection2D::Up => CardinalDirection2D::Down,
            CardinalDirection2D::Down => CardinalDirection2D::Up,
            CardinalDirection2D::Left => CardinalDirection2D::Right,
            CardinalDirection2D::Right => CardinalDirection2D::Left,
        }
    }
}
