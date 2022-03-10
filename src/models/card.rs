use serde::{Deserialize, Serialize};
use crate::schema::{cards};

#[derive(Queryable, Identifiable, Insertable, Serialize)]
pub struct Card {
    pub id: i32,
    pub card_type: String,
    pub suit: String,
    pub number: i32,
}


impl Card {
    fn symbol(&self) -> &str {
        match self.suit.as_str() {
            "Clubs" => "♣",
            "Hearts" => "♥",
            "Spades" => "♠",
            "Diamonds" => "♦",
            _ => ""
        }
    }
    pub fn to_human(&self) -> String {
        if self.card_type == "None" {
            format!("{} {}", self.number, self.symbol())
        } else {
            format!("{:?} {}", self.card_type, self.symbol())
        }
    }
}

#[derive(Insertable, Serialize)]
#[table_name = "cards"]
pub struct NewCard {
    pub card_type: String,
    pub suit: String,
    pub number: i32,
}

impl NewCard {
    pub fn from(card: &crate::models::card::Card) -> NewCard {
        NewCard {
            number: card.number as i32,
            card_type: format!("{:?}", card.card_type),
            suit: format!("{:?}", card.suit),
        }
    }
    fn symbol(&self) -> char {
        match self.suit.as_str() {
            "Clubs" => '♣',
            "Hearts" => '♥',
            "Spades" => '♠',
            "Diamonds" => '♦',
            _ => ' '
        }
    }
    pub fn to_human(&self) -> String {
        if self.card_type == "None" {
            format!("{} {}", self.number, self.symbol())
        } else {
            format!("{:?} {}", self.card_type, self.symbol())
        }
    }
}