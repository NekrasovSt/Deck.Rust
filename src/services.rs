use crate::models::card::Card;
use crate::models::card_type::CardType;
use crate::models::suit::Suit;

pub fn card_builder() -> Vec<Card> {
    let numbers: [u8; 9] = [2, 3, 4, 5, 6, 7, 8, 9, 10];
    let suits: [Suit; 4] = [Suit::Clubs, Suit::Hearts, Suit::Spades, Suit::Diamonds];
    let types: [CardType; 4] = [CardType::Jack, CardType::Queen, CardType::King, CardType::Ace];
    let mut cards = Vec::with_capacity(52);
    for suit in suits
    {
        for number in numbers
        {
            cards.push(Card {
                card_type: CardType::None,
                suit,
                number,
            })
        }
        for typ in types
        {
            cards.push(Card {
                card_type: typ,
                suit,
                number: 0,
            })
        }
    }
    cards
}