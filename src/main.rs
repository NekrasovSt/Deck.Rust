#[macro_use]
extern crate diesel;
extern crate dotenv;

mod schema;
mod models;
mod db;

use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, web};
use crate::db::add_deck;
use crate::models::deck::{Deck, NewDeck};

#[post("/deck")]
async fn post(newDeck: web::Json<NewDeck>) -> impl Responder {
    let result = add_deck(&newDeck.name);
    web::Json(result.unwrap())
    // HttpResponse::Ok().body(newDeck.name.to_string())
}

#[get("/deck/{id}")]
async fn get_id(web::Path(id): web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(id.to_string())
}

#[get("/deck")]
async fn get() -> impl Responder {
    HttpResponse::Ok().body("Get!")
}

#[get("/deck/getByName/{name}")]
async fn get_by_name(web::Path(name): web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(name)
}

#[get("/deck/{id}/getCards")]
async fn get_cards(web::Path(id): web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(id.to_string())
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get)
            .service(get_id)
            .service(get_by_name)
            .service(get_cards)
            .service(post)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}