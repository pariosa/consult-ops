use actix_web::{web, get};
use crate::controllers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api")
        .route("/users", web::get().to(controllers::get_users)));
}