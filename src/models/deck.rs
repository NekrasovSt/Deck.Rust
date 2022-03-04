use crate::schema::{decks};
use serde::{Serialize, Deserialize};

#[derive(Queryable, Identifiable, Debug, Serialize)]
pub struct Deck {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "decks"]
pub struct NewDeck {
    pub name: String,
}