#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum MatchResult {
    Win,
    Lost,
    Draw,
}


impl From<&str> for MatchResult {
    fn from(s: &str) -> Self {
        let c = s.chars().next().unwrap();
        MatchResult::from(c)
    }
}

impl From<char> for MatchResult {
    fn from(c: char) -> Self {
        match c {
            'X' => MatchResult::Lost,
            'Y' => MatchResult::Draw,
            'Z' => MatchResult::Win,
            _ => unreachable!(),
        }
    }
}

