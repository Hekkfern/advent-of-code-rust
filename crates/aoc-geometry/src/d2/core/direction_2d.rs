use crate::Vector;

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

impl From<Direction2D> for Vector<i8, DIMENSIONS> {
    fn from(dir: Direction2D) -> Self {
        match dir {
            Direction2D::Up => Vector::new([0, 1]),
            Direction2D::UpLeft => Vector::new([-1, 1]),
            Direction2D::UpRight => Vector::new([1, 1]),
            Direction2D::Down => Vector::new([0, -1]),
            Direction2D::DownLeft => Vector::new([-1, 1]),
            Direction2D::DownRight => Vector::new([1, 1]),
            Direction2D::Left => Vector::new([-1, 0]),
            Direction2D::Right => Vector::new([1, 0]),
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
