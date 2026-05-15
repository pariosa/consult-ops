use actix_web::{HttpResponse, Responder, web};

use crate::auth::permissions::{can_manage_transactions, can_process_transactions};
use crate::auth_context::AuthUser;
use crate::db::Db;
use crate::models::operational_transaction::OperationalTransaction;
use crate::services::event_service::EventService;

pub async fn list_engagement_transactions(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    if !can_process_transactions(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to view engagement transactions."
        }));
    }

    let engagement_id = path.into_inner();

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
    if !can_process_transactions(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to view organization transactions."
        }));
    }

    let organization_id = path.into_inner();

    let result = sqlx::query_as::<_, OperationalTransaction>(
        r#"
        SELECT *
        FROM operational_transactions
        WHERE organization_id = ?
        ORDER BY created_at DESC
        "#,
    )
    .bind(organization_id)
    .fetch_all(db.pool.as_ref())
    .await;

    match result {
        Ok(transactions) => HttpResponse::Ok().json(transactions),
        Err(err) => {
            eprintln!("list_organization_transactions error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

async fn apply_transaction_status(
    db: web::Data<Db>,
    transaction_id: i64,
    next_status: &str,
    event_type: &str,
) -> HttpResponse {
    let existing = match sqlx::query_as::<_, OperationalTransaction>(
        r#"
        SELECT *
        FROM operational_transactions
        WHERE id = ?
        "#,
    )
    .bind(transaction_id)
    .fetch_one(db.pool.as_ref())
    .await
    {
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
        None,
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
            None,
            "engagement",
            engagement_id,
            event_type,
            Some(&from_status),
            Some(next_status),
            metadata,
        )
        .await;
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

    apply_transaction_status(db, path.into_inner(), "paid", "OperationalTransactionPaid").await
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
        path.into_inner(),
        "cancelled",
        "OperationalTransactionCancelled",
    )
    .await
}
