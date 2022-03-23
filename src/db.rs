use std::env;
use std::error::Error;
use dotenv::dotenv;
use diesel::{Connection, EqAll, insert_into, PgConnection, QueryDsl, r2d2, RunQueryDsl};
use diesel::r2d2::{ConnectionManager};
use crate::services::card_builder;
use crate::models::card::{Card};
use crate::models::card_decks::CardDecks;
use crate::models::deck::{Deck, NewDeck};

use crate::schema::card_decks;
use crate::schema::cards;
use crate::schema::decks;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
embed_migrations!();

pub fn create_pool() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = diesel::r2d2::ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn link_cards_to_deck(linked_deck_id: i32, connection: &PgConnection) -> Result<(), diesel::result::Error> {
    let result = crate::schema::cards::dsl::cards.load::<Card>(connection)?;

    let val = result.iter().enumerate().map(|(index, c)| CardDecks {
        deck_id: linked_deck_id,
        order: index as i32,
        card_id: c.id,
    }).collect::<Vec<CardDecks>>();
    diesel::insert_into(crate::schema::card_decks::table)
        .values(val).execute(connection)?;
    Ok(())
}

pub fn add_deck(deck_name: &String, connection: &PgConnection) -> Result<Deck, diesel::result::Error> {
    let new_deck = diesel::insert_into(decks::table)
        .values(NewDeck {
            name: deck_name.to_owned()
        })
        .get_result::<Deck>(connection)?;
    link_cards_to_deck(new_deck.id, connection)?;
    Ok(new_deck)
}

pub fn find_deck(id: i32, connection: &PgConnection) -> Result<Deck, diesel::result::Error> {
    let deck = crate::schema::decks::dsl::decks.find(id).first(connection)?;
    Ok(deck)
}

pub fn get_decks(connection: &PgConnection) -> Result<Vec<Deck>, diesel::result::Error> {
    let decks = crate::schema::decks::dsl::decks.load::<Deck>(connection)?;
    Ok(decks)
}

pub fn find_by_name(name: &String, connection: &PgConnection) -> Result<Vec<Deck>, diesel::result::Error> {
    let decks = crate::schema::decks::dsl::decks.filter(crate::schema::decks::name.eq_all(name)).load::<Deck>(connection)?;
    Ok(decks)
}

pub fn delete_deck(id: i32, connection: &PgConnection) -> Result<String, diesel::result::Error> {
    let result = diesel::delete(crate::schema::decks::dsl::decks.find(id)).execute(connection)?;
    Ok(format!("Удалено строк: {}", result))
}

pub fn get_cards_by_deck(deck_id: i32, connection: &PgConnection) -> Result<Vec<Card>, diesel::result::Error> {
    let tuples = card_decks::table.inner_join(cards::table)
        .select((cards::suit, cards::number, cards::card_type, cards::id))
        .filter(crate::schema::card_decks::deck_id.eq_all(deck_id))
        .order(crate::db::card_decks::order)
        .load::<(String, i32, String, i32)>(connection)?;
    Ok(tuples.into_iter().map(|t| Card { card_type: t.2, suit: t.0, number: t.1, id: t.3 }).collect())
}

pub fn save_card(save_deck_id: i32, cards: Vec<i32>, connection: &PgConnection) -> Result<Vec<CardDecks>, diesel::result::Error> {
    use crate::db::card_decks::dsl::card_decks;

    let links = cards.iter().enumerate().map(|(index, id)| {
        CardDecks {
            card_id: *id,
            deck_id: save_deck_id,
            order: index as i32,
        }
    }).collect::<Vec<CardDecks>>();
    connection.transaction(|| {
        diesel::delete(card_decks.filter(crate::schema::card_decks::deck_id.eq_all(save_deck_id)))
            .execute(connection)?;
        diesel::insert_into(crate::schema::card_decks::table)
            .values(&links)
            .execute(connection)?;
        Ok(())
    }).map_err(|err: diesel::result::Error| err)?;
    Ok(links)
}

pub fn init_db() -> Result<(), Box<dyn Error>> {
    let connection = establish_connection();
    embedded_migrations::run(&connection)?;
    let cards = crate::schema::cards::dsl::cards.load::<Card>(&connection)?;
    if cards.len() == 0 {
        let new_cards = card_builder();
        insert_into(cards::table)
            .values(new_cards)
            .execute(&connection)?;
    }
    Ok(())
}
