// main.rs
mod db;
mod models;
mod seed_demo;
mod auth;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use db::Db;
use auth::{login, register};
use models::{
    get_clients, create_client,
    get_projects, create_project,
    get_contracts, create_contract,
    get_invoices, create_invoice,
    get_payments, create_payment,
};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize DB pool
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");
    let db = Db::new(&database_url)
        .await
        .expect("Failed to connect to DB");
    // Seed demo data if needed
    if let Err(e) = seed_demo::seed_demo_data(&db).await {
        eprintln!("Failed to seed demo data: {}", e);
    }

    println!("Server running at http://127.0.0.1:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            )
            .service(
                web::scope("/api")
                    // Auth routes
                    .route("/auth/register", web::post().to(register))
                    .route("/auth/login", web::post().to(login))
                    // Client routes
                    .route("/clients", web::get().to(get_clients))
                    .route("/clients", web::post().to(create_client))
                    // Project routes
                    .route("/projects", web::get().to(get_projects))
                    .route("/projects", web::post().to(create_project))
                    // Contract routes
                    .route("/contracts", web::get().to(get_contracts))
                    .route("/contracts", web::post().to(create_contract))
                    // Invoice routes
                    .route("/invoices", web::get().to(get_invoices))
                    .route("/invoices", web::post().to(create_invoice))
                    // Payment routes
                    .route("/payments", web::get().to(get_payments))
                    .route("/payments", web::post().to(create_payment))
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}