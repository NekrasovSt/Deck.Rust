use crate::models::card_type::CardType;
use crate::models::suit::Suit;

pub struct Card {
    card_type: CardType,
    suit: Suit,
    number: u8
}