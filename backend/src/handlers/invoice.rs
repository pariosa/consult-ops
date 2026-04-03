use crate::db::Db;
use crate::models::Invoice;
use actix_web::{HttpResponse, Responder, web};

/// Get all invoices
pub async fn get_invoices(db: web::Data<Db>) -> impl Responder {
    match Invoice::all(&db).await {
        Ok(invoices) => HttpResponse::Ok().json(invoices),
        Err(e) => {
            eprintln!("DB error fetching invoices: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch invoices")
        }
    }
}

/// Create a new invoice
pub async fn create_invoice(db: web::Data<Db>, info: web::Json<Invoice>) -> impl Responder {
    let mut invoice = info.into_inner();

    // Default status
    if invoice.status.is_empty() {
        invoice.status = "draft".to_string();
    }

    // Default created_at
    if invoice.created_at.is_empty() {
        invoice.created_at = chrono::Utc::now().to_rfc3339();
    }

    // Optional numeric fields: subtotal, tax, total can default to 0 if not provided
    if invoice.subtotal.is_none() {
        invoice.subtotal = Some(0.0);
    }
    if invoice.tax.is_none() {
        invoice.tax = Some(0.0);
    }
    if invoice.total.is_none() {
        invoice.total = Some(invoice.subtotal.unwrap_or(0.0) + invoice.tax.unwrap_or(0.0));
    }

    match Invoice::create(&db, invoice).await {
        Ok(invoice) => HttpResponse::Ok().json(invoice),
        Err(e) => {
            eprintln!("DB error creating invoice: {}", e);
            HttpResponse::InternalServerError().body("Failed to create invoice")
        }
    }
}
