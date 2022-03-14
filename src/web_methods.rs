use actix_web::{error, Responder, web, delete, get, post, put, Error};
use actix_web::web::Data;
use diesel::{PgConnection, r2d2};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use actix_web_validator::{Json};
use crate::models::deck::NewDeck;
use crate::shuffles;
use crate::db::{add_deck, delete_deck, find_by_name, find_deck, get_cards_by_deck, get_decks, save_card};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[post("/deck")]
pub async fn post(new_deck: Json<NewDeck>, pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let connection = get_connection(pool)?;
    let deck = web::block(move || add_deck(&new_deck.name, &connection))
        .await
        .map_err(error::ErrorBadRequest)?;
    Ok(web::Json(deck))
}

fn get_connection(pool: Data<DbPool>) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Error> {
    pool.get().map_err(|_| error::ErrorBadRequest(String::from("Ошибка подключения к БД.")))
}

#[get("/deck/{id}")]
pub async fn get_id(web::Path(id): web::Path<i32>, pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let connection = get_connection(pool)?;
    let deck = web::block(move || find_deck(id, &connection))
        .await
        .map_err(error::ErrorBadRequest)?;
    Ok(web::Json(deck))
}

#[get("/deck")]
pub async fn get(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let connection = get_connection(pool)?;
    let result = web::block(move || get_decks(&connection))
        .await
        .map_err(error::ErrorBadRequest)?;
    Ok(web::Json(result))
}

#[get("/deck/getByName/{name}")]
pub async fn get_by_name(web::Path(name): web::Path<String>, pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let connection = get_connection(pool)?;
    let result = web::block(move || find_by_name(&name, &connection))
        .await
        .map_err(error::ErrorBadRequest)?;
    Ok(web::Json(result))
}

#[delete("/deck/{id}")]
pub async fn delete(web::Path(id): web::Path<i32>, pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let connection = get_connection(pool)?;
    let result = web::block(move || delete_deck(id, &connection))
        .await
        .map_err(error::ErrorBadRequest)?;
    Ok(web::Json(result))
}

#[get("/deck/{id}/getCards")]
pub async fn get_cards(web::Path(id): web::Path<i32>, pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let connection = get_connection(pool)?;
    let result = web::block(move || get_cards_by_deck(id, &connection))
        .await
        .map_err(error::ErrorBadRequest)?;
    Ok(web::Json(result))
}

#[get("/deck/{id}/getHumanizeCards")]
pub async fn get_humanize_cards(web::Path(id): web::Path<i32>, pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let connection = get_connection(pool)?;
    let cards = web::block(move || get_cards_by_deck(id, &connection))
        .await
        .map_err(error::ErrorBadRequest)?;
    Ok(web::Json(
        cards.iter()
            .map(|c| c.to_human())
            .collect::<Vec<String>>()))
}

#[put("/deck/{id}/shuffle")]
pub async fn shuffle(web::Path(id): web::Path<i32>, pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let connection = get_connection(pool.clone())?;
    let cards = web::block(move || get_cards_by_deck(id, &connection))
        .await
        .map_err(error::ErrorBadRequest)?;
    let mut ids = cards.iter().map(|x| x.id).collect::<Vec<i32>>();
    shuffles::hand_shuffle::shuffle(&mut ids);
    let connection = get_connection(pool.clone())?;
    web::block(move || save_card(id, ids, &connection))
        .await
        .map_err(error::ErrorBadRequest)?;
    Ok(web::Json(cards))
}
