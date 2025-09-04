use crate::Vector;
use crate::generic::core::vector_coordinate::VectorCoordinate;

const DIMENSIONS: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction2D {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction2D {
    pub fn to_vector<T>(&self) -> Vector<T, DIMENSIONS>
    where
        T: VectorCoordinate,
    {
        match self {
            Direction2D::Up => Vector::new([T::zero(), T::one()]),
            Direction2D::UpLeft => Vector::new([-T::one(), T::one()]),
            Direction2D::UpRight => Vector::new([T::one(), T::one()]),
            Direction2D::Down => Vector::new([T::zero(), -T::one()]),
            Direction2D::DownLeft => Vector::new([-T::one(), T::one()]),
            Direction2D::DownRight => Vector::new([T::one(), T::one()]),
            Direction2D::Left => Vector::new([-T::one(), T::zero()]),
            Direction2D::Right => Vector::new([T::one(), T::zero()]),
        }
    }
}

impl std::ops::Neg for Direction2D {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Direction2D::Up => Direction2D::Down,
            Direction2D::Down => Direction2D::Up,
            Direction2D::Left => Direction2D::Right,
            Direction2D::Right => Direction2D::Left,
            Direction2D::DownLeft => Direction2D::UpRight,
            Direction2D::DownRight => Direction2D::UpLeft,
            Direction2D::UpLeft => Direction2D::DownRight,
            Direction2D::UpRight => Direction2D::DownLeft,
        }
    }
}
