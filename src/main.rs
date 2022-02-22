mod models;
use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};

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
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}