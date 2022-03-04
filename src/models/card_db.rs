use crate::schema::{cards};

#[derive(Queryable, Identifiable, Insertable)]
pub struct Card {
    pub id: i32,
    pub card_type: String,
    pub suit: String,
    pub number: i32,
}

#[derive(Insertable)]
#[table_name = "cards"]
pub struct NewCard<'a> {
    pub card_type: &'a str,
    pub suit: &'a str,
    pub number: i32,
}