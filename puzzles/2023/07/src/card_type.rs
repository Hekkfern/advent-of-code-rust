#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub enum CardType {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl CardType {
    pub fn value(&self, with_jokers: bool) -> u32 {
        match (with_jokers, self) {
            (true, CardType::Jack) => 0,
            (true, _) => 1 + (self.clone() as u32),
            (false, _) => self.clone() as u32,
        }
    }

    pub fn cmp(&self, other: &Self, with_jokers: bool) -> std::cmp::Ordering {
        self.value(with_jokers).cmp(&other.value(with_jokers))
    }
}

impl From<&str> for CardType {
    fn from(s: &str) -> Self {
        let c = s.chars().next().unwrap();
        CardType::from(c)
    }
}

impl From<char> for CardType {
    fn from(c: char) -> Self {
        match c {
            '2' => CardType::Two,
            '3' => CardType::Three,
            '4' => CardType::Four,
            '5' => CardType::Five,
            '6' => CardType::Six,
            '7' => CardType::Seven,
            '8' => CardType::Eight,
            '9' => CardType::Nine,
            'T' => CardType::Ten,
            'J' => CardType::Jack,
            'Q' => CardType::Queen,
            'K' => CardType::King,
            'A' => CardType::Ace,
            _ => unreachable!(),
        }
    }
}
