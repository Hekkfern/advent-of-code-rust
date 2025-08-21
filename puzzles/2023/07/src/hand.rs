use crate::card_type::CardType;
use crate::hand_type::HandType;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Hand {
    cards: [CardType; 5],
    bid: u32,
    strength: HandType,
}

impl Hand {
    fn analyze_cards(cards: &[CardType; 5], with_jokers: bool) -> HandType {
        // Sort the cards in descending order (highest card first)
        let mut sorted_cards = cards.clone();
        sorted_cards.sort_by(|a, b| b.cmp(a, with_jokers));
        // Translate cards into frequencies of equal cards
        let mut freqs: [u32; 5] = [1, 0, 0, 0, 0];
        let mut freq_idx: usize = 0;
        let mut num_jokers: u32 = 0;
        for i in 1..5 {
            if with_jokers && matches!(sorted_cards[i], CardType::Jack) {
                num_jokers += 1;
            } else {
                if sorted_cards[i - 1] != sorted_cards[i] {
                    freq_idx += 1;
                }
                freqs[freq_idx] += 1;
            }
        }
        freqs.sort_by(|a, b| b.cmp(a));
        if with_jokers {
            freqs[0] += num_jokers;
        }
        // Map the frequencies to the hand type
        match (freqs[0], freqs[1]) {
            (5, _) => HandType::FiveOfKind,
            (4, _) => HandType::FourOfKind,
            (3, 2) => HandType::FullHouse,
            (3, _) => HandType::ThreeOfKind,
            (2, 2) => HandType::TwoPair,
            (2, _) => HandType::OnePair,
            (_, _) => HandType::HighCard,
        }
    }

    pub fn new(cards: [CardType; 5], bid: u32, with_jokers: bool) -> Self {
        let strength = Self::analyze_cards(&cards, with_jokers);
        Self {
            cards,
            bid,
            strength,
        }
    }

    pub fn cmp(&self, other: &Self, with_jokers: bool) -> std::cmp::Ordering {
        let cmp = self.strength.cmp(&other.strength);
        if cmp != std::cmp::Ordering::Equal {
            cmp
        } else {
            self.cards
                .iter()
                .zip(other.cards.iter())
                .map(|(a, b)| a.cmp(b, with_jokers))
                .find(|&ord| ord != std::cmp::Ordering::Equal)
                .unwrap_or(std::cmp::Ordering::Equal)
        }
    }

    pub fn bid(&self) -> u32 {
        self.bid
    }
}
