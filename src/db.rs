use std::env;
use std::error::Error;
use dotenv::dotenv;
use diesel::{Connection, PgConnection, RunQueryDsl};
use crate::models::deck::{Deck, NewDeck};
use crate::schema::decks;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn add_deck(deckName: &String) -> Result<Deck, String> {
    let connection = establish_connection();
    match diesel::insert_into(decks::table)
        .values(NewDeck {
            name: deckName.to_owned()
        })
        .get_result::<Deck>(&connection) {
        Ok(i) => Ok(i),
        Err(err) => Err(format!("Ошибка добавление новой колоды: {}", err.to_string()))
    }
}