use actix_web::{HttpResponse, Responder, web};
use sqlx::SqlitePool;

use crate::models::engagement::{CreateEngagement, Engagement};

pub async fn create_for_project(
    db: web::Data<SqlitePool>,
    path: web::Path<i64>,
    payload: web::Json<CreateEngagement>,
) -> impl Responder {
    let project_id = path.into_inner();
    let mut input = payload.into_inner();
    input.project_id = project_id;

    match Engagement::create(&db, input).await {
        Ok(engagement) => HttpResponse::Ok().json(engagement),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn list_for_project(db: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
    let project_id = path.into_inner();

    match Engagement::for_project(&db, project_id).await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn show(db: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
    match Engagement::find(&db, path.into_inner()).await {
        Ok(engagement) => HttpResponse::Ok().json(engagement),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

pub async fn mark_contract_sent(db: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
    match Engagement::update_status(&db, path.into_inner(), "contract_sent").await {
        Ok(engagement) => HttpResponse::Ok().json(engagement),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn mark_signed(db: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
    match Engagement::update_status(&db, path.into_inner(), "contract_signed").await {
        Ok(engagement) => HttpResponse::Ok().json(engagement),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
