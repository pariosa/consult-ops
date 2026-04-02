use actix_web::{web, HttpResponse};
use crate::models::{Project, Client, Contract, Invoice, Payment};
use rusqlite::Connection;

pub async fn get_projects(db: web::Data<Conn>) -> impl Responder {
    let projects = Project::all(&db).unwrap_or(vec![]);
    HttpResponse::Ok().json(projects)
}

pub async fn get_invoices(db: web::Data<Connection>) -> HttpResponse {
    let invoices = Invoice::all(&db).unwrap_or(vec![]);
    HttpResponse::Ok().json(invoices)
}

pub async fn get_contracts(db: web::Data<Connection>) -> HttpResponse {
    let contracts = Contract::all(&db).unwrap_or(vec![]);
    HttpResponse::Ok().json(contracts)
}

pub async fn get_users() -> HttpResponse {
    HttpResponse::Ok().body("users list")
}