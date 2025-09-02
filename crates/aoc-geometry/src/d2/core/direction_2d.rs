use crate::Vector;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl From<Direction2D> for Vector<i32, 2> {
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
