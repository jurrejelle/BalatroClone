use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Suit::Spades    => write!(f, "S"),
            Suit::Hearts    => write!(f, "H"),
            Suit::Diamonds  => write!(f, "D"),
            Suit::Clubs     => write!(f, "C")
        }
    }
}