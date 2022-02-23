mod models;
mod services;
mod tests;

use actix_web::{get, post, delete, App, HttpResponse, HttpServer, Responder, web};
use crate::models::deck::Deck;
use crate::services::card_builder;

#[get("/deck/{id}")]
async fn get_id(web::Path(id): web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(id.to_string())
}

#[get("/deck")]
async fn get() -> impl Responder {
    HttpResponse::Ok().body("Get!")
}

#[post("/deck")]
async fn post(mut deck: web::Json<Deck>) -> impl Responder {
    deck.id = 14;
    HttpResponse::Created().body(format!("{:?}", deck))
}

#[get("/deck/getByName/{name}")]
async fn get_by_name(web::Path(name): web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(name)
}

#[get("/deck/{id}/getCards")]
async fn get_cards(web::Path(id): web::Path<u32>) -> impl Responder {
    web::Json(card_builder())
    
}#[get("/deck/{id}/getHumanizeCards")]
async fn get_humanize_cards(web::Path(id): web::Path<u32>) -> impl Responder {
    web::Json(card_builder().iter().map(|card|{card.to_human()}).collect::<Vec<String>>().join(", "))
}

#[delete("/deck/{id}")]
async fn delete(web::Path(id): web::Path<u32>) -> impl Responder {
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
            .service(delete)
            .service(get_humanize_cards)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}