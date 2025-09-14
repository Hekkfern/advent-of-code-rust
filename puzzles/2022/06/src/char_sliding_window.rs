use bitvec::prelude::*;
use std::collections::VecDeque;
use std::ops::BitXor;

const ALPHABET_SIZE: usize = 26;

/// Sliding window over lowercase ASCII letters 'a'..='z'.
pub struct CharSlidingWindow<const WINDOW_SIZE: usize> {
    chars: VecDeque<u8>,
    // 26-bit bitmap stored in a u32 word, LSB-first indexing (bit 0 == 'a').
    sliding: BitArr!(for ALPHABET_SIZE, in u32, Lsb0),
}

impl<const WINDOW_SIZE: usize> CharSlidingWindow<WINDOW_SIZE> {
    pub fn new() -> Self {
        Self {
            chars: VecDeque::with_capacity(WINDOW_SIZE),
            sliding: bitarr![u32, Lsb0; 0; ALPHABET_SIZE],
        }
    }

    pub fn add_char(&mut self, c: char) {
        let c8 = c as u8;
        let idx = (c8 - b'a') as usize;

        // add
        self.chars.push_back(c8);
        let mut new_bit = bitarr![u32, Lsb0; 0; ALPHABET_SIZE];
        new_bit.set(idx, true);
        self.sliding = self.sliding.bitxor(new_bit);

        // remove if exceeded window size
        if self.chars.len() > WINDOW_SIZE
            && let Some(old) = self.chars.pop_front()
        {
            let old_idx = (old - b'a') as usize;
            let mut old_bit = bitarr![u32, Lsb0; 0; ALPHABET_SIZE];
            old_bit.set(old_idx, true);
            self.sliding = self.sliding.bitxor(old_bit);
        }
    }

    /// True if window length == WINDOW_SIZE and all chars are unique.
    pub fn are_unique(&self) -> bool {
        if self.chars.len() < WINDOW_SIZE {
            return false;
        }
        // If any character repeats, its bit cancels back to 0, reducing the count.
        self.sliding.count_ones() == WINDOW_SIZE
    }
}

#[cfg(test)]
mod tests {
    use super::CharSlidingWindow;

    #[test]
    fn basics() {
        let mut w: CharSlidingWindow<4> = CharSlidingWindow::new();
        for c in ['a', 'b', 'c', 'd'] {
            w.add_char(c);
        }
        assert!(w.are_unique());

        w.add_char('a'); // window now: b c d a
        assert!(w.are_unique());

        w.add_char('c'); // window now: c d a c  => 'c' repeats
        assert!(!w.are_unique());

        // fill with unique again
        for c in ['e', 'f', 'g'] {
            w.add_char(c);
        }
        assert!(w.are_unique());
    }

    #[test]
    fn too_short_is_false() {
        let mut w: CharSlidingWindow<3> = CharSlidingWindow::new();
        w.add_char('a');
        w.add_char('b');
        assert!(!w.are_unique());
        w.add_char('c');
        assert!(w.are_unique());
    }
}
