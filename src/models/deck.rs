use crate::schema::{decks};
use serde::{Serialize, Deserialize};
use validator::Validate;


#[derive(Queryable, Identifiable, Debug, Serialize)]
pub struct Deck {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Deserialize, Validate)]
#[table_name = "decks"]
pub struct NewDeck {
    #[validate(length(min = 3))]
    pub name: String,
}
