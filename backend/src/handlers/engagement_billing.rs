use crate::db::Db;
use crate::domain::engagement_state::{EngagementEvent, EngagementStatus};
use crate::models::engagement_billing::{
    CreateEngagementBillingRequest, EngagementBilling, UpdateCheckoutSessionRequest,
};
use crate::services::email_notification_service::EmailNotificationService;
use crate::services::event_service::EventService;
use crate::services::notification_email_recipient_service::NotificationRecipientService;
use crate::services::operations_kernel_service::OperationsKernelService;
use actix_web::{HttpResponse, Responder, web};
use sqlx::{Result, SqlitePool};
use stripe_checkout::CheckoutSession;
use stripe_checkout::checkout_session::CreateCheckoutSession;
use stripe_checkout::checkout_session::CreateCheckoutSessionLineItems;
use stripe_shared::CheckoutSessionMode;

pub async fn activate_engagement_from_payment(
    db: &Db,
    engagement_id: i64,
    organization_id: i64,
) -> Result<(), String> {
    let current_status: String = sqlx::query_scalar!(
        r#"
        SELECT status as "status!"
        FROM engagements
        WHERE id = $1
        "#,
        engagement_id
    )
    .fetch_one(db.pool.as_ref())
    .await
    .map_err(|err| err.to_string())?;

    let current_status: EngagementStatus =
        serde_json::from_value(serde_json::Value::String(current_status.clone()))
            .map_err(|_| format!("Invalid engagement status: {}", current_status))?;

    let next_status = OperationsKernelService::apply_engagement_event(
        db.pool.as_ref(),
        organization_id,
        engagement_id,
        None,
        current_status,
        EngagementEvent::PaymentReceived,
    )
    .await?;

    let next_status_string = serde_json::to_value(next_status)
        .ok()
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| format!("{:?}", next_status).to_lowercase());

    sqlx::query!(
        r#"
        UPDATE engagements
        SET status = $1
        WHERE id = $2
        "#,
        next_status_string,
        engagement_id
    )
    .execute(db.pool.as_ref())
    .await
    .map_err(|err| err.to_string())?;

    Ok(())
}
async fn organization_id_for_engagement(db: &Db, engagement_id: i64) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar!(
        r#"
        SELECT organization_id as "organization_id!"
        FROM engagements
        WHERE id = $1
        "#,
        engagement_id
    )
    .fetch_one(db.pool.as_ref())
    .await
}

pub async fn record_billing_and_engagement_event(
    db: &Db,
    billing: &EngagementBilling,
    event_type: &str,
    from_status: Option<&str>,
    to_status: Option<&str>,
) {
    let metadata = serde_json::json!({
        "billing_id": billing.id,
        "engagement_id": billing.engagement_id,
        "billing_type": billing.billing_type,
        "amount_cents": billing.amount_cents,
        "currency": billing.currency,
        "stripe_checkout_session_id": billing.stripe_checkout_session_id,
        "stripe_payment_intent_id": billing.stripe_payment_intent_id,
    });

    let _ = EventService::record_event(
        db.pool.as_ref(),
        billing.organization_id,
        None,
        "engagement_billing",
        billing.id,
        event_type,
        from_status,
        to_status,
        metadata.clone(),
    )
    .await;

    let _ = EventService::record_event(
        db.pool.as_ref(),
        billing.organization_id,
        None,
        "engagement",
        billing.engagement_id,
        event_type,
        from_status,
        to_status,
        metadata,
    )
    .await;
}

pub async fn list_engagement_billing(db: web::Data<Db>, path: web::Path<i64>) -> impl Responder {
    let engagement_id = path.into_inner();

    match EngagementBilling::for_engagement(db.pool.as_ref(), engagement_id).await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(err) => {
            eprintln!("list_engagement_billing error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn create_engagement_billing(
    db: web::Data<Db>,
    path: web::Path<i64>,
    payload: web::Json<CreateEngagementBillingRequest>,
) -> impl Responder {
    let engagement_id = path.into_inner();
    let input = payload.into_inner();

    let organization_id = match organization_id_for_engagement(&db, engagement_id).await {
        Ok(id) => id,
        Err(err) => {
            eprintln!("organization_id_for_engagement error: {:?}", err);
            return HttpResponse::BadRequest().body("Invalid engagement_id");
        }
    };

    let billing_type = input
        .billing_type
        .unwrap_or_else(|| "activation_fee".to_string());

    let amount_cents = input.amount_cents.unwrap_or(1000);
    let currency = input.currency.unwrap_or_else(|| "usd".to_string());

    match EngagementBilling::create(
        db.pool.as_ref(),
        engagement_id,
        organization_id,
        billing_type,
        amount_cents,
        currency,
    )
    .await
    {
        Ok(billing) => {
            record_billing_and_engagement_event(
                &db,
                &billing,
                "EngagementBillingCreated",
                None,
                Some(&billing.status),
            )
            .await;

            HttpResponse::Created().json(billing)
        }
        Err(err) => {
            eprintln!("create_engagement_billing error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn create_activation_fee(db: web::Data<Db>, path: web::Path<i64>) -> impl Responder {
    let engagement_id = path.into_inner();

    let organization_id = match organization_id_for_engagement(&db, engagement_id).await {
        Ok(id) => id,
        Err(err) => {
            eprintln!("organization_id_for_engagement error: {:?}", err);
            return HttpResponse::BadRequest().body("Invalid engagement_id");
        }
    };
    if let Ok(Some(existing)) =
        EngagementBilling::find_activation_fee(db.pool.as_ref(), engagement_id).await
    {
        if existing.status == "paid" {
            return HttpResponse::Conflict().json(serde_json::json!({
                "error": "Activation fee has already been paid for this engagement.",
                "billing": existing
            }));
        }

        return HttpResponse::Ok().json(existing);
    }
    match EngagementBilling::create_activation_fee(db.pool.as_ref(), engagement_id, organization_id)
        .await
    {
        Ok(billing) => {
            record_billing_and_engagement_event(
                &db,
                &billing,
                "ActivationFeeCreated",
                None,
                Some(&billing.status),
            )
            .await;

            HttpResponse::Created().json(billing)
        }
        Err(err) => {
            eprintln!("create_activation_fee error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn attach_checkout_session(
    db: web::Data<Db>,
    path: web::Path<i64>,
    payload: web::Json<UpdateCheckoutSessionRequest>,
) -> impl Responder {
    let billing_id = path.into_inner();
    let input = payload.into_inner();

    match EngagementBilling::attach_checkout_session(
        db.pool.as_ref(),
        billing_id,
        &input.stripe_checkout_session_id,
    )
    .await
    {
        Ok(billing) => {
            record_billing_and_engagement_event(
                &db,
                &billing,
                "ActivationCheckoutStarted",
                Some("pending"),
                Some(&billing.status),
            )
            .await;

            HttpResponse::Ok().json(billing)
        }
        Err(err) => {
            eprintln!("attach_checkout_session error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn mark_billing_paid(db: web::Data<Db>, path: web::Path<i64>) -> impl Responder {
    let billing_id = path.into_inner();

    match EngagementBilling::mark_paid(db.pool.as_ref(), billing_id).await {
        Ok(billing) => {
            record_billing_and_engagement_event(
                &db,
                &billing,
                "ActivationFeePaid",
                Some("pending"),
                Some("paid"),
            )
            .await;

            match activate_engagement_from_payment(
                &db,
                billing.engagement_id,
                billing.organization_id,
            )
            .await
            {
                Ok(_) => HttpResponse::Ok().json(billing),
                Err(err) => {
                    eprintln!("activate_engagement_from_payment error: {:?}", err);

                    HttpResponse::Ok().json(serde_json::json!({
                        "billing": billing,
                        "activation_warning": err
                    }))
                }
            }
        }
        Err(err) => {
            eprintln!("mark_billing_paid error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}
pub async fn create_activation_checkout(db: web::Data<Db>, path: web::Path<i64>) -> impl Responder {
    let engagement_id = path.into_inner();

    let organization_id = match organization_id_for_engagement(&db, engagement_id).await {
        Ok(id) => id,
        Err(err) => {
            eprintln!("organization_id_for_engagement error: {:?}", err);
            return HttpResponse::BadRequest().body("Invalid engagement_id");
        }
    };

    let billing =
        match EngagementBilling::find_activation_fee(db.pool.as_ref(), engagement_id).await {
            Ok(Some(existing)) => existing,
            Ok(None) => match EngagementBilling::create_activation_fee(
                db.pool.as_ref(),
                engagement_id,
                organization_id,
            )
            .await
            {
                Ok(created) => {
                    record_billing_and_engagement_event(
                        &db,
                        &created,
                        "ActivationFeeCreated",
                        None,
                        Some(&created.status),
                    )
                    .await;

                    created
                }
                Err(err) => {
                    eprintln!("create_activation_fee error: {:?}", err);
                    return HttpResponse::InternalServerError().body(err.to_string());
                }
            },
            Err(err) => {
                eprintln!("find_activation_fee error: {:?}", err);
                return HttpResponse::InternalServerError().body(err.to_string());
            }
        };

    if billing.status == "paid" {
        return HttpResponse::Conflict().json(serde_json::json!({
            "error": "Activation fee is already paid.",
            "billing": billing
        }));
    }

    let stripe_secret = match std::env::var("STRIPE_SECRET_KEY") {
        Ok(key) => key,
        Err(_) => return HttpResponse::InternalServerError().body("STRIPE_SECRET_KEY missing"),
    };

    let frontend_url =
        std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://127.0.0.1:3000".to_string());
    let http = reqwest::Client::new();

    let success_url = format!(
        "{}/engagements/{}?activation=success",
        frontend_url, engagement_id
    );

    let cancel_url = format!(
        "{}/engagements/{}?activation=cancelled",
        frontend_url, engagement_id
    );

    let amount_cents = billing.amount_cents.to_string();

    let form = vec![
        ("mode", "payment".to_string()),
        ("success_url", success_url),
        ("cancel_url", cancel_url),
        ("line_items[0][quantity]", "1".to_string()),
        (
            "line_items[0][price_data][currency]",
            billing.currency.clone(),
        ),
        ("line_items[0][price_data][unit_amount]", amount_cents),
        (
            "line_items[0][price_data][product_data][name]",
            "Engagement activation fee".to_string(),
        ),
        ("metadata[billing_id]", billing.id.to_string()),
        ("metadata[engagement_id]", billing.engagement_id.to_string()),
        (
            "metadata[organization_id]",
            billing.organization_id.to_string(),
        ),
    ];

    let session: serde_json::Value = match http
        .post("https://api.stripe.com/v1/checkout/sessions")
        .bearer_auth(&stripe_secret)
        .form(&form)
        .send()
        .await
    {
        Ok(res) => match res.json().await {
            Ok(json) => json,
            Err(err) => {
                eprintln!("Stripe checkout JSON parse error: {:?}", err);
                return HttpResponse::InternalServerError().body(err.to_string());
            }
        },
        Err(err) => {
            eprintln!("Stripe checkout request error: {:?}", err);
            return HttpResponse::InternalServerError().body(err.to_string());
        }
    };
    let checkout_url = session
        .get("url")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let session_id = match session.get("id").and_then(|v| v.as_str()) {
        Some(id) => id.to_string(),
        None => {
            eprintln!("Stripe session missing id: {:?}", session);
            return HttpResponse::InternalServerError().body("Stripe session missing id");
        }
    };

    let updated_billing =
        match EngagementBilling::attach_checkout_session(db.pool.as_ref(), billing.id, &session_id)
            .await
        {
            Ok(updated) => updated,
            Err(err) => {
                eprintln!("attach_checkout_session error: {:?}", err);
                return HttpResponse::InternalServerError().body(err.to_string());
            }
        };

    record_billing_and_engagement_event(
        &db,
        &updated_billing,
        "ActivationCheckoutStarted",
        Some("pending"),
        Some(&updated_billing.status),
    )
    .await;
    if let Some(url) = checkout_url.clone() {
        match NotificationRecipientService::engagement_client_email(db.pool.as_ref(), engagement_id)
            .await
        {
            Ok(Some(client_email)) => {
                if let Err(err) =
                    EmailNotificationService::activation_checkout(client_email, url).await
                {
                    eprintln!("activation checkout email error: {:?}", err);
                }
            }
            Ok(None) => {
                eprintln!("No client email found for engagement {}", engagement_id);
            }
            Err(err) => {
                eprintln!("engagement_client_email lookup error: {:?}", err);
            }
        }
    }
    HttpResponse::Ok().json(serde_json::json!({
        "billing": updated_billing,
        "checkout_session_id": session_id,
        "url": checkout_url
    }))
}
