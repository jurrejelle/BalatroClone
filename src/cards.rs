use std::fmt;
use crate::suit::Suit;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Card{
    pub rank : usize, // 1: Ace, 2-10: normal cards, 11: Jack, 12: Queen, 13: King
    pub suit : Suit,
}
impl Card {
    pub fn apply_mult(&self, current_mult: usize) -> usize {
        return current_mult;
    }
    pub fn apply_chips(&self, current_chips: usize) -> usize {
        match &self.rank{
            1 => return current_chips + 11,
            2..=10 => return current_chips + self.rank,
            11..=13 => return current_chips + 10,
            _ => panic!("Rank unexpected value, expected 1 <= rank <= 13, got {}", self.rank)
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.rank{
            1       => write!(f, "A{}", self.suit),
            2..=10  => write!(f, "{}{}", self.rank, self.suit),
            11      => write!(f, "J{}", self.suit),
            12      => write!(f, "Q{}", self.suit),
            13      => write!(f, "K{}", self.suit),
            _ => panic!("Rank unexpected value, expected 1 <= rank <= 13, got {}", self.rank)
        }

    }
}