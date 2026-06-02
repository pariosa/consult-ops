use actix_web::{HttpRequest, HttpResponse, Responder, web};
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::db::Db;
use crate::handlers::engagement_billing::{
    activate_engagement_from_payment, record_billing_and_engagement_event,
};
use crate::models::EngagementBilling;
use crate::services::engagement_activation_service::maybe_activate_engagement;

type HmacSha256 = Hmac<Sha256>;

fn verify_stripe_signature(payload: &str, signature_header: &str, webhook_secret: &str) -> bool {
    let mut timestamp: Option<&str> = None;
    let mut signatures: Vec<&str> = Vec::new();

    for part in signature_header.split(',') {
        let mut pieces = part.splitn(2, '=');
        let key = pieces.next().unwrap_or("");
        let value = pieces.next().unwrap_or("");

        match key {
            "t" => timestamp = Some(value),
            "v1" => signatures.push(value),
            _ => {}
        }
    }

    let Some(timestamp) = timestamp else {
        return false;
    };

    if signatures.is_empty() {
        return false;
    }

    let signed_payload = format!("{}.{}", timestamp, payload);

    let mut mac = match HmacSha256::new_from_slice(webhook_secret.as_bytes()) {
        Ok(mac) => mac,
        Err(_) => return false,
    };

    mac.update(signed_payload.as_bytes());

    let expected_signature = hex::encode(mac.finalize().into_bytes());

    signatures.iter().any(|sig| *sig == expected_signature)
}

pub async fn stripe_webhook(
    db: web::Data<Db>,
    body: web::Bytes,
    req: HttpRequest,
) -> impl Responder {
    eprintln!("STRIPE WEBHOOK HIT");
    let webhook_secret = match std::env::var("STRIPE_WEBHOOK_SECRET") {
        Ok(secret) => secret,
        Err(_) => {
            return HttpResponse::InternalServerError().body("STRIPE_WEBHOOK_SECRET missing");
        }
    };

    let signature = match req.headers().get("stripe-signature") {
        Some(value) => match value.to_str() {
            Ok(value) => value,
            Err(_) => {
                return HttpResponse::BadRequest().body("Invalid Stripe signature header");
            }
        },
        None => {
            return HttpResponse::BadRequest().body("Missing Stripe signature header");
        }
    };

    let payload = match std::str::from_utf8(&body) {
        Ok(payload) => payload,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UTF-8 payload"),
    };

    if !verify_stripe_signature(payload, signature, &webhook_secret) {
        eprintln!("Stripe webhook signature verification failed");
        return HttpResponse::BadRequest().body("Invalid webhook signature");
    }

    let event: serde_json::Value = match serde_json::from_str(payload) {
        Ok(event) => event,
        Err(err) => {
            eprintln!("Stripe webhook JSON parse error: {:?}", err);
            return HttpResponse::BadRequest().body("Invalid JSON payload");
        }
    };

    let event_type = event
        .get("type")
        .and_then(|value| value.as_str())
        .unwrap_or("");
    eprintln!("STRIPE EVENT TYPE: {}", event_type);
    if event_type != "checkout.session.completed" {
        return HttpResponse::Ok().json(serde_json::json!({
            "received": true,
            "ignored": event_type
        }));
    }

    let session_id = event
        .get("data")
        .and_then(|data| data.get("object"))
        .and_then(|object| object.get("id"))
        .and_then(|id| id.as_str());

    let Some(session_id) = session_id else {
        return HttpResponse::BadRequest().body("Missing checkout session id");
    };

    match EngagementBilling::mark_paid_by_session(db.pool.as_ref(), session_id).await {
        Ok(billing) => {
            eprintln!("MARKING BILLING PAID BY SESSION: {}", session_id);
            record_billing_and_engagement_event(
                &db,
                &billing,
                "ActivationFeePaid",
                Some("pending"),
                Some("paid"),
            )
            .await;

            let update_engagement_fee_result = sqlx::query(
                r#"
                    UPDATE engagements
                    SET platform_fee_status = 'paid',
                        updated_at = $1
                    WHERE id = $2
                    AND organization_id = $3
                    "#,
            )
            .bind(chrono::Utc::now().to_rfc3339())
            .bind(billing.engagement_id)
            .bind(billing.organization_id)
            .execute(db.pool.as_ref())
            .await;

            if let Err(err) = update_engagement_fee_result {
                eprintln!("Failed to update engagement platform fee status: {}", err);

                return HttpResponse::InternalServerError()
                    .body("Failed to update engagement platform fee status");
            }
            let _ = maybe_activate_engagement(
                db.get_ref(),
                billing.engagement_id,
                billing.organization_id,
                None,
            )
            .await;

            if let Err(err) = activate_engagement_from_payment(
                &db,
                billing.engagement_id,
                billing.organization_id,
            )
            .await
            {
                eprintln!("activate_engagement_from_payment webhook error: {:?}", err);
            }

            HttpResponse::Ok().json(serde_json::json!({
                "received": true,
                "session_id": session_id,
                "billing_id": billing.id,
                "engagement_id": billing.engagement_id
            }))
        }
        Err(err) => {
            eprintln!("mark_paid_by_session error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}
