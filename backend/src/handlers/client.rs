use crate::db::Db;
use crate::models::Client;
use actix_web::{HttpResponse, Responder, web};

pub async fn get_clients(db: web::Data<Db>) -> impl Responder {
    match Client::all(&db).await {
        Ok(clients) => HttpResponse::Ok().json(clients),
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch clients")
        }
    }
}

pub async fn create_client(db: web::Data<Db>, info: web::Json<Client>) -> impl Responder {
    match Client::create(&db, info.into_inner()).await {
        Ok(client) => HttpResponse::Ok().json(client),
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body("Failed to create client")
        }
    }
}
