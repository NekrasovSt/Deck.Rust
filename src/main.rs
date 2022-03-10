#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;

mod schema;
mod models;
mod db;
mod services;
mod response;

use actix_web::{get, post, delete, App, HttpResponse, HttpServer, Responder, web, Result, error};
use crate::db::{add_deck, delete_deck, find_by_name, find_deck, get_cards_by_deck, get_decks, init_db};
use crate::models::card::NewCard;
use crate::models::deck::{NewDeck};
use crate::services::card_builder;

#[post("/deck")]
async fn post(new_deck: web::Json<NewDeck>) -> impl Responder {
    let result = add_deck(&new_deck.name);
    match result {
        Ok(deck) => Ok(web::Json(deck)),
        Err(err) => Err(error::ErrorBadRequest(err))
    }
}

#[get("/deck/{id}")]
async fn get_id(web::Path(id): web::Path<i32>) -> impl Responder {
    let result = find_deck(id);
    match result {
        Ok(deck) => Ok(web::Json(deck)),
        Err(err) => Err(error::ErrorNotFound(err))
    }
}

#[get("/deck")]
async fn get() -> impl Responder {
    let result = get_decks();
    match result {
        Ok(deck) => Ok(web::Json(deck)),
        Err(err) => Err(error::ErrorBadRequest(err))
    }
}

#[get("/deck/getByName/{name}")]
async fn get_by_name(web::Path(name): web::Path<String>) -> impl Responder {
    let result = find_by_name(&name);
    match result {
        Ok(deck) => Ok(web::Json(deck)),
        Err(err) => Err(error::ErrorBadRequest(err))
    }
}

#[delete("/deck/{id}")]
async fn delete(web::Path(id): web::Path<i32>) -> impl Responder {
    let result = delete_deck(id);
    match result {
        Ok(message) => Ok(web::Json(message)),
        Err(err) => Err(error::ErrorBadRequest(err))
    }
}

#[get("/deck/{id}/getCards")]
async fn get_cards(web::Path(id): web::Path<i32>) -> impl Responder {
    match get_cards_by_deck(id) {
        Ok(cards) => Ok(web::Json(cards)),
        Err(err) => Err(error::ErrorBadRequest(err))
    }
}

#[get("/deck/{id}/getHumanizeCards")]
async fn get_humanize_cards(web::Path(id): web::Path<i32>) -> impl Responder {
    match get_cards_by_deck(id) {
        Ok(cards) => Ok(web::Json(
            cards.iter()
                .map(|c| c.to_human())
                .collect::<Vec<String>>())),
        Err(err) => Err(error::ErrorBadRequest(err))
    }
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
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}