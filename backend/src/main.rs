mod auth;
mod db;
mod handlers;
mod models;
mod routes;
mod seed_demo;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use db::Db;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize DB pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
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
                    .allow_any_header(),
            )
            .configure(routes::config) // <- attach routes here
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
