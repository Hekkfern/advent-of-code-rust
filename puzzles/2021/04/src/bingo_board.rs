pub const BOARD_WIDTH: usize = 5;
pub const BOARD_HEIGHT: usize = 5;

pub struct BingoBoard {
    pub numbers: ndarray::Array2<u8>,
}

impl BingoBoard {
    pub fn new(numbers: Vec<u8>) -> Self {
        assert_eq!(numbers.len(), BOARD_WIDTH * BOARD_HEIGHT);
        let array = ndarray::Array2::from_shape_vec(
            (BOARD_HEIGHT, BOARD_WIDTH),
            numbers,
        )
        .unwrap();
        BingoBoard { numbers: array }
    }

    pub fn mark_number(&mut self, number: u8) {
        for elem in self.numbers.iter_mut() {
            if *elem == number {
                *elem = 255; // Marked number
            }
        }
    }

    pub fn has_bingo(&self) -> bool {
        // Check rows
        for row in self.numbers.rows() {
            if row.iter().all(|&x| x == 255) {
                return true;
            }
        }
        // Check columns
        for col in self.numbers.columns() {
            if col.iter().all(|&x| x == 255) {
                return true;
            }
        }
        false
    }

    pub fn unmarked_sum(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|&&x| x != 255)
            .map(|&x| x as u32)
            .sum()
    }
}
