use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub enum Suit
{
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}