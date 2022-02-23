use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Deck {
    pub name: String,
    pub id: u32
}