use actix_web::{HttpResponse, Responder, web};
use serde_json::json;
use stripe::Client;

use stripe_checkout::CheckoutSessionMode;

use stripe_checkout::checkout_session::{CreateCheckoutSession, CreateCheckoutSessionLineItems};

pub async fn create_activation_checkout(path: web::Path<i64>) -> impl Responder {
    let engagement_id = path.into_inner();

    let stripe_secret = std::env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set");

    let frontend_url =
        std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    let price_id = std::env::var("STRIPE_ACTIVATION_PRICE_ID")
        .expect("STRIPE_ACTIVATION_PRICE_ID must be set");

    let client = Client::new(stripe_secret);

    let success_url = format!(
        "{}/engagements/{}/billing?checkout=success",
        frontend_url, engagement_id
    );

    let cancel_url = format!(
        "{}/engagements/{}/billing?checkout=cancelled",
        frontend_url, engagement_id
    );

    let line_item = CreateCheckoutSessionLineItems {
        price: Some(price_id),
        quantity: Some(1),
        ..Default::default()
    };
    let session_result = CreateCheckoutSession::new()
        .mode(CheckoutSessionMode::Payment)
        .success_url(success_url)
        .cancel_url(cancel_url)
        .client_reference_id(engagement_id.to_string())
        .line_items(vec![line_item])
        .send(&client)
        .await;

    match session_result {
        Ok(session) => HttpResponse::Ok().json(json!({
            "url": session.url
        })),

        Err(err) => {
            eprintln!("Stripe checkout error: {:?}", err);
            HttpResponse::InternalServerError().body(format!("{:?}", err))
        }
    }
}
