#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
extern crate validator;

mod shuffles;
mod schema;
mod models;
mod db;
mod services;
mod tests;
mod web_methods;
mod config;
mod errors;


use actix_web::{App, HttpServer, middleware, web};
use slog::info;
use crate::db::{create_pool, init_db};
use crate::models::app_state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = create_pool();

    let log = config::configure_log();
    info!(log, "Пытаюсь подключится к БД");

    init_db().expect("Неудалось создать БД");

    let address = "127.0.0.1:8080";
    info!(
        log,
        "Starting server at http://{}", address);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                pool: pool.clone(),
                log: log.clone(),
            }))
            .wrap(middleware::Logger::default())
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
        .bind(address)?
        .run()
        .await
}
