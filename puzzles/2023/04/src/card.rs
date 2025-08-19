use std::collections::HashSet;

pub type CardId = u8;

#[derive(Eq, PartialEq, Debug)]
pub struct Card {
    card_id: CardId,
    winning_numbers: HashSet<u8>,
    candidate_numbers: HashSet<u8>,
}

impl Card {
    pub fn new(
        card_id: CardId,
        winning_numbers: HashSet<u8>,
        candidate_numbers: HashSet<u8>,
    ) -> Self {
        Self {
            card_id,
            winning_numbers,
            candidate_numbers,
        }
    }

    pub fn card_id(&self) -> CardId {
        self.card_id
    }

    pub fn calculate_points(&self) -> u32 {
        let winning_count = self.calculate_matching_numbers();
        if winning_count == 0 {
            return 0;
        }
        2_u32.pow(winning_count - 1)
    }

    pub fn calculate_matching_numbers(&self) -> u32 {
        self.winning_numbers
            .iter()
            .filter(|x| self.candidate_numbers.contains(x))
            .count() as u32
    }
}
