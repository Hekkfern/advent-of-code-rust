#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum TileType {
    EmptySpace,
    MirrorSlash,
    MirrorBackslash,
    SplitterVertical,
    SplitterHorizontal,
}

impl From<&str> for TileType {
    fn from(s: &str) -> Self {
        let c = s.chars().next().unwrap();
        TileType::from(c)
    }
}

impl From<char> for TileType {
    fn from(c: char) -> Self {
        match c {
            '.' => TileType::EmptySpace,
            '/' => TileType::MirrorSlash,
            '\\' => TileType::MirrorBackslash,
            '|' => TileType::SplitterVertical,
            '-' => TileType::SplitterHorizontal,
            _ => unreachable!(),
        }
    }
}
