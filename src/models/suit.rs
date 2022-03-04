use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Copy, Clone, Debug, PartialEq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}
