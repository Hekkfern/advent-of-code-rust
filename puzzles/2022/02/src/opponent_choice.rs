#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum OpponentChoice {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for OpponentChoice {
    fn from(s: &str) -> Self {
        let c = s.chars().next().unwrap();
        OpponentChoice::from(c)
    }
}

impl From<char> for OpponentChoice {
    fn from(c: char) -> Self {
        match c {
            'A' => OpponentChoice::Rock,
            'B' => OpponentChoice::Paper,
            'C' => OpponentChoice::Scissors,
            _ => unreachable!(),
        }
    }
}
