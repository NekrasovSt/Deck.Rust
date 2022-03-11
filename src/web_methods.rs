use actix_web::{error, Responder, web, delete, get, post, put};
use crate::models::deck::NewDeck;
use crate::shuffles;
use crate::db::{add_deck, delete_deck, find_by_name, find_deck, get_cards_by_deck, get_decks, save_card};

#[post("/deck")]
pub async fn post(new_deck: web::Json<NewDeck>) -> actix_web::Result<impl Responder> {
    let deck = web::block(move || add_deck(&new_deck.name))
        .await
        .map_err(error::ErrorBadRequest)?;
    Ok(web::Json(deck))
}

#[get("/deck/{id}")]
pub async fn get_id(web::Path(id): web::Path<i32>) -> actix_web::Result<impl Responder> {
    let deck = web::block(move || find_deck(id))
        .await
        .map_err(error::ErrorBadRequest)?;
    Ok(web::Json(deck))
}

#[get("/deck")]
pub async fn get() -> actix_web::Result<impl Responder> {
    let result = web::block(move || get_decks())
        .await
        .map_err(error::ErrorBadRequest)?;
    Ok(web::Json(result))
}

#[get("/deck/getByName/{name}")]
pub async fn get_by_name(web::Path(name): web::Path<String>) -> actix_web::Result<impl Responder> {
    let result = web::block(move || find_by_name(&name))
        .await
        .map_err(error::ErrorBadRequest)?;
    Ok(web::Json(result))
}

#[delete("/deck/{id}")]
pub async fn delete(web::Path(id): web::Path<i32>) -> actix_web::Result<impl Responder> {
    let result = web::block(move || delete_deck(id))
        .await
        .map_err(error::ErrorBadRequest)?;
    Ok(web::Json(result))
}

#[get("/deck/{id}/getCards")]
pub async fn get_cards(web::Path(id): web::Path<i32>) -> actix_web::Result<impl Responder> {
    let result = web::block(move || get_cards_by_deck(id))
        .await
        .map_err(error::ErrorBadRequest)?;
    Ok(web::Json(result))
}

#[get("/deck/{id}/getHumanizeCards")]
pub async fn get_humanize_cards(web::Path(id): web::Path<i32>) -> actix_web::Result<impl Responder> {
    let cards = web::block(move || get_cards_by_deck(id))
        .await
        .map_err(error::ErrorBadRequest)?;
    Ok(web::Json(
        cards.iter()
            .map(|c| c.to_human())
            .collect::<Vec<String>>()))
}

#[put("/deck/{id}/shuffle")]
pub async fn shuffle(web::Path(id): web::Path<i32>) -> actix_web::Result<impl Responder> {
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
