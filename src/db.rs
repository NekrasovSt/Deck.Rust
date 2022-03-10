use std::env;
use dotenv::dotenv;
use diesel::{Connection, EqAll, insert_into, PgConnection, PgTextExpressionMethods, QueryDsl, RunQueryDsl};
use crate::card_builder;
use crate::models::card::{Card, NewCard};
use crate::models::card_decks::CardDecks;
use crate::models::deck::{Deck, NewDeck};

use crate::schema::card_decks;
use crate::schema::cards;
use crate::schema::decks;

embed_migrations!();

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn link(linked_deck_id: i32, connection: &PgConnection) -> Result<(), diesel::result::Error> {
    let result = crate::schema::cards::dsl::cards.load::<Card>(connection)?;
    for (index, c) in result.iter().enumerate() {
        diesel::insert_into(crate::schema::card_decks::table)
            .values(CardDecks {
                deck_id: linked_deck_id,
                order: index as i32,
                card_id: c.id,
            }).execute(connection);
    }
    Ok(())
}

pub fn add_deck(deck_name: &String) -> Result<Deck, String> {
    let connection = establish_connection();
    match diesel::insert_into(decks::table)
        .values(NewDeck {
            name: deck_name.to_owned()
        })
        .get_result::<Deck>(&connection) {
        Ok(new_deck) => match link(new_deck.id, &connection) {
            Ok(()) => Ok(new_deck),
            Err(err_link) => Err(format!("Ошибка добавление новой колоды: {}", err_link.to_string()))
        },
        Err(err) => Err(format!("Ошибка добавление новой колоды: {}", err.to_string()))
    }
}

pub fn find_deck(id: i32) -> Result<Deck, String> {
    let connection = establish_connection();
    match crate::schema::decks::dsl::decks.find(id).first(&connection) {
        Ok(deck) => Ok(deck),
        Err(_) => Err(String::from("Колода не найдена"))
    }
}

pub fn get_decks() -> Result<Vec<Deck>, String> {
    let connection = establish_connection();
    match crate::schema::decks::dsl::decks.load::<Deck>(&connection) {
        Ok(decks) => Ok(decks),
        Err(err) => Err(String::from("Ошибка получения колод"))
    }
}

pub fn find_by_name(name: &String) -> Result<Vec<Deck>, String> {
    let connection = establish_connection();
    match crate::schema::decks::dsl::decks.filter(crate::schema::decks::name.eq_all(name)).load::<Deck>(&connection) {
        Ok(decks) => Ok(decks),
        Err(err) => Err(String::from("Ошибка получения колод"))
    }
}

pub fn delete_deck(id: i32) -> Result<String, String> {
    let connection = establish_connection();
    match diesel::delete(crate::schema::decks::dsl::decks.find(id)).execute(&connection) {
        Ok(r) => Ok(format!("Удалено строк: {}", r)),
        Err(_) => Err(String::from("Не удалось удалить колоду"))
    }
}

pub fn get_cards_by_deck(deck_id: i32) -> Result<Vec<NewCard>, String> {
    use crate::db::cards::{id, suit, number, card_type};
    let connection = establish_connection();
    let result = card_decks::table.inner_join(cards::table)
        .select((cards::suit, cards::number, cards::card_type))
        .order(crate::db::card_decks::order)
        .load::<(String, i32, String)>(&connection);
    match result {
        Ok(tuples) => Ok(tuples.into_iter().map(|t| NewCard { card_type: t.2, suit: t.0, number: t.1 }).collect()),
        Err(_) => Err(String::from("Ошибка получения карт"))
    }
}


pub fn init_db() {
    let connection = establish_connection();
    embedded_migrations::run(&connection);
    if crate::schema::cards::dsl::cards.load::<Card>(&connection).unwrap().len() == 0 {
        let new_cards = card_builder();
        insert_into(cards::table)
            .values(new_cards)
            .execute(&connection);
    }
}