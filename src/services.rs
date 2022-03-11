use crate::models::card::{NewCard};

pub fn card_builder() -> Vec<NewCard> {
    let numbers: [i32; 9] = [2, 3, 4, 5, 6, 7, 8, 9, 10];
    let suits: [&str; 4] = ["Clubs", "Hearts", "Spades", "Diamonds"];
    let types: [&str; 4] = [
        "Jack",
        "Queen",
        "King",
        "Ace",
    ];
    let mut cards = Vec::with_capacity(52);
    for suit in suits {
        for number in numbers {
            cards.push(NewCard {
                card_type: String::from("None"),
                suit: String::from(suit),
                number,
            })
        }
        for typ in types {
            cards.push(NewCard {
                card_type: String::from(typ),
                suit: String::from(suit),
                number: 0,
            })
        }
    }
    cards
}
