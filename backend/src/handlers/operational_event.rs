use actix_web::{HttpResponse, Responder, ResponseError, web};
use serde::Serialize;
use sqlx::FromRow;

use crate::auth_context::AuthUser;
use crate::db::Db;
use crate::services::authz::require_org_member;

#[derive(Debug, Serialize, FromRow)]
struct OperationalEventResponse {
    id: i64,
    organization_id: i64,
    actor_user_id: Option<i64>,
    entity_type: String,
    entity_id: i64,
    event_type: String,
    from_status: Option<String>,
    to_status: Option<String>,
    metadata: String,
    created_at: String,
}

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

pub async fn list_engagement_events(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let engagement_id = path.into_inner();

    let organization_id = match organization_id_for_engagement(&db, engagement_id).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::NotFound().body("Engagement not found"),
    };

    if let Err(err) = require_org_member(db.pool.as_ref(), auth.id, organization_id).await {
        return err.error_response();
    }

    let events = sqlx::query_as::<_, OperationalEventResponse>(
        r#"
        SELECT
            id,
            organization_id,
            actor_user_id,
            entity_type,
            entity_id,
            event_type,
            from_status,
            to_status,
            metadata,
            created_at
        FROM operational_events
        WHERE entity_type = 'engagement'
          AND entity_id = $1
          AND organization_id = $2
        ORDER BY created_at DESC
        "#,
    )
    .bind(engagement_id)
    .bind(organization_id)
    .fetch_all(db.pool.as_ref())
    .await;

    match events {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(err) => {
            eprintln!("list_engagement_events error: {:?}", err);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": err.to_string()
            }))
        }
    }
}

pub async fn list_organization_events(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let organization_id = path.into_inner();

    if let Err(err) = require_org_member(db.pool.as_ref(), auth.id, organization_id).await {
        return err.error_response();
    }

    let events = sqlx::query_as::<_, OperationalEventResponse>(
        r#"
        SELECT
            id,
            organization_id,
            actor_user_id,
            entity_type,
            entity_id,
            event_type,
            from_status,
            to_status,
            metadata,
            created_at
        FROM operational_events
        WHERE organization_id = $1
        ORDER BY created_at DESC
        LIMIT 250
        "#,
    )
    .bind(organization_id)
    .fetch_all(db.pool.as_ref())
    .await;

    match events {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(err) => {
            eprintln!("list_organization_events error: {:?}", err);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": err.to_string()
            }))
        }
    }
}
