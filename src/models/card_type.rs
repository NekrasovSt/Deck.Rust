use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Copy, Clone, Debug, PartialEq)]
pub enum CardType {
    None,
    Jack,
    Queen,
    King,
    Ace
}