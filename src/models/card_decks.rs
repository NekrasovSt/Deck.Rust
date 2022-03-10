use crate::models::deck::Deck;
use crate::models::card::Card;
use crate::schema::{card_decks};


#[derive(Queryable, Associations, Insertable, PartialEq, Debug)]
#[belongs_to(Card, foreign_key = "card_id")]
#[belongs_to(Deck, foreign_key = "deck_id")]
#[table_name = "card_decks"]
pub struct CardDecks {
    pub card_id: i32,
    pub deck_id: i32,
    pub order: i32,
}