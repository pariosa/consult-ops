use crate::models::Client;
use crate::{db::Db, models::client::CreateClient};
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

pub async fn create_client(
    db: web::Data<Db>,
    path: web::Path<i64>,
    info: web::Json<CreateClient>,
) -> impl Responder {
    let organization_id = path.into_inner();
    let mut payload = info.into_inner();
    payload.organization_id = organization_id;

    match Client::create(&db, payload).await {
        Ok(client) => HttpResponse::Ok().json(client),
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body("Failed to create client")
        }
    }
}
