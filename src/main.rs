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
mod web_methods;

use actix_web::{App, HttpServer};
use crate::db::init_db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_db().expect("Неудалось создать БД");
    HttpServer::new(|| {
        App::new()
            .service(web_methods::get)
            .service(web_methods::get_id)
            .service(web_methods::get_by_name)
            .service(web_methods::get_cards)
            .service(web_methods::post)
            .service(web_methods::post)
            .service(web_methods::delete)
            .service(web_methods::get_humanize_cards)
            .service(web_methods::shuffle)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
