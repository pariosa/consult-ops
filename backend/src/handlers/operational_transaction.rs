use actix_web::{HttpResponse, Responder, ResponseError, web};

use crate::auth::permissions::{can_manage_transactions, can_process_transactions};
use crate::auth_context::AuthUser;
use crate::db::Db;
use crate::models::operational_transaction::OperationalTransaction;
use crate::services::authz::{require_org_member, require_org_role};
use crate::services::event_service::EventService;
use crate::services::notification_email_recipient_service::NotificationRecipientService;
use crate::services::notification_service::NotificationService;

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

async fn organization_id_for_transaction(db: &Db, transaction_id: i64) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(
        r#"
        SELECT organization_id
        FROM operational_transactions
        WHERE id = $1
        "#,
    )
    .bind(transaction_id)
    .fetch_one(db.pool.as_ref())
    .await
}

pub async fn list_engagement_transactions(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let engagement_id = path.into_inner();

    if !can_process_transactions(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to view engagement transactions."
        }));
    }

    let organization_id = match organization_id_for_engagement(&db, engagement_id).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::NotFound().body("Engagement not found"),
    };

    if let Err(err) = require_org_member(db.pool.as_ref(), auth.id, organization_id).await {
        return err.error_response();
    }

    match OperationalTransaction::for_engagement(db.pool.as_ref(), engagement_id).await {
        Ok(transactions) => HttpResponse::Ok().json(transactions),
        Err(err) => {
            eprintln!("list_engagement_transactions error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn list_organization_transactions(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let organization_id = path.into_inner();

    if !can_process_transactions(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to view organization transactions."
        }));
    }

    if let Err(err) = require_org_member(db.pool.as_ref(), auth.id, organization_id).await {
        return err.error_response();
    }

    match OperationalTransaction::for_organization(db.pool.as_ref(), organization_id).await {
        Ok(transactions) => HttpResponse::Ok().json(transactions),
        Err(err) => {
            eprintln!("list_organization_transactions error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

async fn apply_transaction_status(
    db: web::Data<Db>,
    auth: AuthUser,
    transaction_id: i64,
    next_status: &str,
    event_type: &str,
) -> HttpResponse {
    let organization_id = match organization_id_for_transaction(&db, transaction_id).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::NotFound().body("Transaction not found"),
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

    let existing = match OperationalTransaction::find(db.pool.as_ref(), transaction_id).await {
        Ok(transaction) => transaction,
        Err(err) => {
            eprintln!("find operational transaction error: {:?}", err);
            return HttpResponse::NotFound().body(err.to_string());
        }
    };

    let from_status = existing.status.clone();

    let updated =
        match OperationalTransaction::update_status(db.pool.as_ref(), transaction_id, next_status)
            .await
        {
            Ok(transaction) => transaction,
            Err(err) => {
                eprintln!("update transaction status error: {:?}", err);
                return HttpResponse::InternalServerError().body(err.to_string());
            }
        };

    let metadata = serde_json::json!({
        "transaction_id": updated.id,
        "agreement_id": updated.agreement_id,
        "engagement_id": updated.engagement_id,
        "milestone_id": updated.milestone_id,
        "from_party_id": updated.from_party_id,
        "to_party_id": updated.to_party_id,
        "transaction_type": updated.transaction_type,
        "amount_cents": updated.amount_cents,
        "currency": updated.currency,
        "from_status": from_status,
        "to_status": next_status
    });

    let _ = EventService::record_event(
        db.pool.as_ref(),
        updated.organization_id,
        Some(auth.id),
        "operational_transaction",
        updated.id,
        event_type,
        Some(&from_status),
        Some(next_status),
        metadata.clone(),
    )
    .await;

    if let Some(engagement_id) = updated.engagement_id {
        let _ = EventService::record_event(
            db.pool.as_ref(),
            updated.organization_id,
            Some(auth.id),
            "engagement",
            engagement_id,
            event_type,
            Some(&from_status),
            Some(next_status),
            metadata,
        )
        .await;
    }

    if next_status == "paid" || next_status == "failed" {
        let notification_type = if next_status == "paid" {
            "transaction_paid"
        } else {
            "transaction_failed"
        };

        let title = if next_status == "paid" {
            "Transaction marked paid"
        } else {
            "Transaction failed"
        };

        let body = if next_status == "paid" {
            format!(
                "A transaction for ${:.2} was marked paid.",
                updated.amount_cents as f64 / 100.0
            )
        } else {
            format!(
                "A transaction for ${:.2} failed.",
                updated.amount_cents as f64 / 100.0
            )
        };

        match NotificationRecipientService::transaction_party_emails(db.pool.as_ref(), updated.id)
            .await
        {
            Ok(emails) => {
                for email in emails {
                    if let Err(err) = NotificationService::notify_email(
                        db.pool.as_ref(),
                        updated.organization_id,
                        email,
                        notification_type,
                        title,
                        &body,
                        Some("operational_transaction"),
                        Some(updated.id),
                    )
                    .await
                    {
                        eprintln!("transaction notification error: {:?}", err);
                    }
                }
            }
            Err(err) => eprintln!("transaction recipient lookup error: {:?}", err),
        }
    }

    HttpResponse::Ok().json(updated)
}

pub async fn mark_transaction_processing(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    if !can_process_transactions(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to process transactions."
        }));
    }

    apply_transaction_status(
        db,
        auth,
        path.into_inner(),
        "processing",
        "OperationalTransactionProcessing",
    )
    .await
}

pub async fn mark_transaction_paid(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    if !can_manage_transactions(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to mark transactions paid."
        }));
    }

    apply_transaction_status(
        db,
        auth,
        path.into_inner(),
        "paid",
        "OperationalTransactionPaid",
    )
    .await
}

pub async fn mark_transaction_failed(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    if !can_manage_transactions(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to mark transactions failed."
        }));
    }

    apply_transaction_status(
        db,
        auth,
        path.into_inner(),
        "failed",
        "OperationalTransactionFailed",
    )
    .await
}

pub async fn cancel_transaction(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    if !can_manage_transactions(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to cancel transactions."
        }));
    }

    apply_transaction_status(
        db,
        auth,
        path.into_inner(),
        "cancelled",
        "OperationalTransactionCancelled",
    )
    .await
}
