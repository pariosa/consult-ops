use actix_web::{HttpResponse, Responder, web};
use sqlx::SqlitePool;

use crate::models::engagement_milestone::{CreateEngagementMilestone, EngagementMilestone};

pub async fn create(
    db: web::Data<SqlitePool>,
    path: web::Path<i64>,
    payload: web::Json<CreateEngagementMilestone>,
) -> impl Responder {
    let engagement_id = path.into_inner();
    let mut input = payload.into_inner();
    input.engagement_id = engagement_id;

    match EngagementMilestone::create(&db, input).await {
        Ok(milestone) => HttpResponse::Ok().json(milestone),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn list(db: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
    match EngagementMilestone::for_engagement(&db, path.into_inner()).await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn submit(db: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
    match EngagementMilestone::update_status(&db, path.into_inner(), "submitted").await {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn approve(db: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
    match EngagementMilestone::update_status(&db, path.into_inner(), "approved").await {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn mark_paid(db: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
    match EngagementMilestone::update_status(&db, path.into_inner(), "paid").await {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
