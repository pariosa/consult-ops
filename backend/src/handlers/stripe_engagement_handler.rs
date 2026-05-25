use actix_web::{HttpResponse, Responder, ResponseError, web};
use serde_json::json;
use stripe::Client;
use stripe_checkout::CheckoutSessionMode;
use stripe_checkout::checkout_session::{CreateCheckoutSession, CreateCheckoutSessionLineItems};

use crate::auth_context::AuthUser;
use crate::db::Db;
use crate::services::authz::require_org_role;

// this might need to be deleted soon

async fn organization_id_for_engagement(db: &Db, engagement_id: i64) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(
        r#"
        SELECT organization_id
        FROM engagements
        WHERE id = $1
        "#,
    )
    .bind(engagement_id)
    .fetch_one(db.pool.as_ref())
    .await
}

pub async fn create_activation_checkout(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let engagement_id = path.into_inner();

    let organization_id = match organization_id_for_engagement(&db, engagement_id).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::NotFound().body("Engagement not found"),
    };

    if let Err(err) = require_org_role(
        db.pool.as_ref(),
        auth.id,
        organization_id,
        &["owner", "admin"],
    )
    .await
    {
        return err.error_response();
    }

    let stripe_secret = match std::env::var("STRIPE_SECRET_KEY") {
        Ok(value) => value,
        Err(_) => return HttpResponse::InternalServerError().body("STRIPE_SECRET_KEY must be set"),
    };

    let price_id = match std::env::var("STRIPE_ACTIVATION_PRICE_ID") {
        Ok(value) => value,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .body("STRIPE_ACTIVATION_PRICE_ID must be set");
        }
    };

    let frontend_url =
        std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

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
        .metadata(std::collections::HashMap::from([
            ("engagement_id".to_string(), engagement_id.to_string()),
            ("organization_id".to_string(), organization_id.to_string()),
        ]))
        .send(&client)
        .await;

    match session_result {
        Ok(session) => HttpResponse::Ok().json(json!({
            "url": session.url,
            "checkout_session_id": session.id,
        })),
        Err(err) => {
            eprintln!("Stripe checkout error: {:?}", err);
            HttpResponse::InternalServerError().body(format!("{:?}", err))
        }
    }
}
