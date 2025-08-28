#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub enum PipeType {
    None,
    Start,
    Horizontal,
    Vertical,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

impl From<&str> for PipeType {
    fn from(s: &str) -> Self {
        let c = s.chars().next().unwrap();
        PipeType::from(c)
    }
}

impl From<char> for PipeType {
    fn from(c: char) -> Self {
        match c {
            'F' => PipeType::SouthEast,
            '-' => PipeType::Horizontal,
            'J' => PipeType::NorthWest,
            '|' => PipeType::Vertical,
            'L' => PipeType::NorthEast,
            '7' => PipeType::SouthWest,
            '.' => PipeType::None,
            'S' => PipeType::Start,
            _ => unreachable!(),
        }
    }
}
