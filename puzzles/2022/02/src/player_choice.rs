#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum PlayerChoice {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for PlayerChoice {
    fn from(s: &str) -> Self {
        let c = s.chars().next().unwrap();
        PlayerChoice::from(c)
    }
}

impl From<char> for PlayerChoice {
    fn from(c: char) -> Self {
        match c {
            'X' => PlayerChoice::Rock,
            'Y' => PlayerChoice::Paper,
            'Z' => PlayerChoice::Scissors,
            _ => unreachable!(),
        }
    }
}
