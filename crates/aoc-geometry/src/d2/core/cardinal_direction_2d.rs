use crate::Vector;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardinalDirection2D {
    Up,
    Down,
    Left,
    Right,
}

impl From<CardinalDirection2D> for Vector<i32, 2> {
    fn from(dir: CardinalDirection2D) -> Self {
        match dir {
            CardinalDirection2D::Up => Vector::new([0, 1]),
            CardinalDirection2D::Down => Vector::new([0, -1]),
            CardinalDirection2D::Left => Vector::new([-1, 0]),
            CardinalDirection2D::Right => Vector::new([1, 0]),
        }
    }
}
