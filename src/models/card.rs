use crate::models::card_type::CardType;
use crate::models::suit::Suit;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Card {
    pub card_type: CardType,
    pub suit: Suit,
    pub number: u8,
}

impl Card {
    fn symbol(&self) -> &str {
        match self.suit
        {
            Suit::Clubs => "♣",
            Suit::Hearts => "♥",
            Suit::Spades => "♠",
            Suit::Diamonds => "♦"
        }
    }
    pub fn to_human(&self) -> String {
        if self.card_type == CardType::None {
            format!("{} {}", self.number, self.symbol())
        } else {
            format!("{:?} {}", self.card_type, self.symbol())
        }
    }
}