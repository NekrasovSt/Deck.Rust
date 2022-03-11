#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;

mod shuffles;
mod schema;
mod models;
mod db;
mod services;
mod tests;

use actix_web::{get, post, delete, put, App, HttpServer, Responder, web, error};
use crate::db::{add_deck, delete_deck, find_by_name, find_deck, get_cards_by_deck, get_decks, init_db, save_card};
use crate::models::deck::{NewDeck};
use crate::services::card_builder;

#[post("/deck")]
async fn post(new_deck: web::Json<NewDeck>) -> actix_web::Result<impl Responder> {
    let deck = web::block(move || add_deck(&new_deck.name))
        .await
        .map_err(error::ErrorBadRequest)?;
    Ok(web::Json(deck))
}

#[get("/deck/{id}")]
async fn get_id(web::Path(id): web::Path<i32>) -> actix_web::Result<impl Responder> {
    let deck = web::block(move || find_deck(id))
        .await
        .map_err(error::ErrorBadRequest)?;
    Ok(web::Json(deck))
}

#[get("/deck")]
async fn get() -> actix_web::Result<impl Responder> {
    let result = web::block(move || get_decks())
        .await
        .map_err(error::ErrorBadRequest)?;
    Ok(web::Json(result))
}

#[get("/deck/getByName/{name}")]
async fn get_by_name(web::Path(name): web::Path<String>) -> actix_web::Result<impl Responder> {
    let result = web::block(move || find_by_name(&name))
        .await
        .map_err(error::ErrorBadRequest)?;
    Ok(web::Json(result))
}

#[delete("/deck/{id}")]
async fn delete(web::Path(id): web::Path<i32>) -> actix_web::Result<impl Responder> {
    let result = web::block(move || delete_deck(id))
        .await
        .map_err(error::ErrorBadRequest)?;
    Ok(web::Json(result))
}

#[get("/deck/{id}/getCards")]
async fn get_cards(web::Path(id): web::Path<i32>) -> actix_web::Result<impl Responder> {
    let result = web::block(move || get_cards_by_deck(id))
        .await
        .map_err(error::ErrorBadRequest)?;
    Ok(web::Json(result))
}

#[get("/deck/{id}/getHumanizeCards")]
async fn get_humanize_cards(web::Path(id): web::Path<i32>) -> actix_web::Result<impl Responder> {
    let cards = web::block(move || get_cards_by_deck(id))
        .await
        .map_err(error::ErrorBadRequest)?;
    Ok(web::Json(
        cards.iter()
            .map(|c| c.to_human())
            .collect::<Vec<String>>()))
}

#[put("/deck/{id}/shuffle")]
async fn shuffle(web::Path(id): web::Path<i32>) -> actix_web::Result<impl Responder> {
    let cards = web::block(move || get_cards_by_deck(id))
        .await
        .map_err(error::ErrorBadRequest)?;
    let mut ids = cards.iter().map(|x| x.id).collect::<Vec<i32>>();
    shuffles::hand_shuffle::shuffle(&mut ids);
    web::block(move || save_card(id, ids))
        .await
        .map_err(error::ErrorBadRequest)?;
    Ok(web::Json(cards))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_db();
    HttpServer::new(|| {
        App::new()
            .service(get)
            .service(get_id)
            .service(get_by_name)
            .service(get_cards)
            .service(post)
            .service(post)
            .service(delete)
            .service(get_humanize_cards)
            .service(shuffle)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
