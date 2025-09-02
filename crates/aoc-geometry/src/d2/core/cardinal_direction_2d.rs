use crate::Vector;

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
}

impl From<CardinalDirection2D> for Vector<i8, DIMENSIONS> {
    fn from(dir: CardinalDirection2D) -> Self {
        match dir {
            CardinalDirection2D::Up => Vector::new([0, 1]),
            CardinalDirection2D::Down => Vector::new([0, -1]),
            CardinalDirection2D::Left => Vector::new([-1, 0]),
            CardinalDirection2D::Right => Vector::new([1, 0]),
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
