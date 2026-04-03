use crate::auth::{login, register};
use crate::handlers::{
    create_client, create_contract, create_invoice, create_payment, create_project, get_clients,
    get_contracts, get_invoices, get_payments, get_projects,
};
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // Auth
            .route("/auth/register", web::post().to(register))
            .route("/auth/login", web::post().to(login))
            // Clients
            .route("/clients", web::get().to(get_clients))
            .route("/clients", web::post().to(create_client))
            // Projects
            .route("/projects", web::get().to(get_projects))
            .route("/projects", web::post().to(create_project))
            // Contracts
            .route("/contracts", web::get().to(get_contracts))
            .route("/contracts", web::post().to(create_contract))
            // Invoices
            .route("/invoices", web::get().to(get_invoices))
            .route("/invoices", web::post().to(create_invoice))
            // Payments
            .route("/payments", web::get().to(get_payments))
            .route("/payments", web::post().to(create_payment)),
    );
}
