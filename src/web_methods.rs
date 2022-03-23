use actix_web::{error, Responder, web, delete, get, post, put, Error, HttpResponse};
use actix_web::error::BlockingError;
use actix_web::web::Data;
use diesel::{PgConnection, r2d2};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use actix_web_validator::{Json};
use slog::{error, Logger, o};
use crate::models::deck::NewDeck;
use crate::{AppState, shuffles};
use crate::db::{add_deck, delete_deck, find_by_name, find_deck, get_cards_by_deck, get_decks, save_card};
use crate::errors::AppError;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn log_error(log: Logger) -> impl Fn(BlockingError<diesel::result::Error>) -> AppError {
    move |e| {
        let err = AppError::from(e);
        let log = log.new(o!(
            "cause" => err.cause.clone()
        ));
        error!(log, "{}", err.message());
        err
    }
}

#[post("/deck")]
pub async fn post(new_deck: Json<NewDeck>, state: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let connection = get_connection(state.pool.clone())?;
    let deck = web::block(move || add_deck(&new_deck.name, &connection))
        .await
        .map_err(error::ErrorBadRequest)?;
    Ok(web::Json(deck))
}

fn get_connection(pool: DbPool) -> Result<PooledConnection<ConnectionManager<PgConnection>>, AppError> {
    pool.get().map_err(|err| AppError::from(err))
}

#[get("/deck/{id}")]
pub async fn get_id(web::Path(id): web::Path<i32>, state: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let connection = get_connection(state.pool.clone())?;
    let deck = web::block(move || find_deck(id, &connection))
        .await
        .map_err(error::ErrorBadRequest)?;
    Ok(web::Json(deck))
}

#[get("/deck")]
pub async fn get(state: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let connection = get_connection(state.pool.clone())?;
    let result = web::block(move || get_decks(&connection))
        .await
        .map_err(log_error(state.log.clone()))?;
    Ok(web::Json(result))
}

#[get("/deck/getByName/{name}")]
pub async fn get_by_name(web::Path(name): web::Path<String>, state: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let connection = get_connection(state.pool.clone())?;
    let result = web::block(move || find_by_name(&name, &connection))
        .await
        .map_err(error::ErrorBadRequest)?;
    Ok(web::Json(result))
}

#[delete("/deck/{id}")]
pub async fn delete(web::Path(id): web::Path<i32>, state: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let connection = get_connection(state.pool.clone())?;
    let result = web::block(move || delete_deck(id, &connection))
        .await
        .map_err(error::ErrorBadRequest)?;
    Ok(web::Json(result))
}

#[get("/deck/{id}/getCards")]
pub async fn get_cards(web::Path(id): web::Path<i32>, state: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let connection = get_connection(state.pool.clone())?;
    let result = web::block(move || get_cards_by_deck(id, &connection))
        .await
        .map_err(error::ErrorBadRequest)?;
    Ok(web::Json(result))
}

#[get("/deck/{id}/getHumanizeCards")]
pub async fn get_humanize_cards(web::Path(id): web::Path<i32>, state: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let connection = get_connection(state.pool.clone())?;
    let cards = web::block(move || get_cards_by_deck(id, &connection))
        .await
        .map_err(error::ErrorBadRequest)?;
    Ok(web::Json(
        cards.iter()
            .map(|c| c.to_human())
            .collect::<Vec<String>>()))
}

#[put("/deck/{id}/shuffle")]
pub async fn shuffle(web::Path(id): web::Path<i32>, state: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let connection = get_connection(state.pool.clone())?;
    let cards = web::block(move || get_cards_by_deck(id, &connection))
        .await
        .map_err(error::ErrorNotFound)?;
    let mut ids = cards.iter().map(|x| x.id).collect::<Vec<i32>>();
    shuffles::hand_shuffle::shuffle(&mut ids);
    let connection = get_connection(state.pool.clone())?;
    web::block(move || save_card(id, ids, &connection))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(web::Json(cards))
}
