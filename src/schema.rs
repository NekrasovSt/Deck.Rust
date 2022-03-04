table! {
    card_decks (card_id, deck_id) {
        card_id -> Int4,
        deck_id -> Int4,
        order -> Int4,
    }
}

table! {
    cards (id) {
        id -> Int4,
        card_type -> Varchar,
        suit -> Varchar,
        number -> Int4,
    }
}

table! {
    decks (id) {
        id -> Int4,
        name -> Varchar,
    }
}

joinable!(card_decks -> cards (card_id));
joinable!(card_decks -> decks (deck_id));

allow_tables_to_appear_in_same_query!(
    card_decks,
    cards,
    decks,
);
