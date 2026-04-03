use crate::db::Db;
use crate::models::payment::{CreatePayment, Payment};
use actix_web::{HttpResponse, Responder, web};

/// Get all payments
pub async fn get_payments(db: web::Data<Db>) -> impl Responder {
    match Payment::all(&db).await {
        Ok(payments) => HttpResponse::Ok().json(payments),
        Err(e) => {
            eprintln!("DB error fetching payments: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch payments")
        }
    }
}

/// Create a new payment
pub async fn create_payment(db: web::Data<Db>, info: web::Json<CreatePayment>) -> impl Responder {
    let payment = Payment::new(info.into_inner());

    match Payment::create(&db, payment).await {
        Ok(saved) => HttpResponse::Ok().json(saved),
        Err(e) => {
            eprintln!("DB error creating payment: {}", e);
            HttpResponse::InternalServerError().body("Failed to create payment")
        }
    }
}
