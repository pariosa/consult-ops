use actix_web::{HttpResponse, Responder, web};
use serde::Serialize;
use sqlx::SqlitePool;

#[derive(Debug, Serialize)]
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

pub async fn list_engagement_events(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
) -> impl Responder {
    let engagement_id = path.into_inner();

    let events = sqlx::query_as!(
        OperationalEventResponse,
        r#"
        SELECT
            id as "id!",
            organization_id as "organization_id!",
            actor_user_id,
            entity_type as "entity_type!",
            entity_id as "entity_id!",
            event_type as "event_type!",
            from_status,
            to_status,
            metadata as "metadata!",
            created_at as "created_at!"
        FROM operational_events
        WHERE entity_type = 'engagement'
          AND entity_id = $1
        ORDER BY created_at DESC
        "#,
        engagement_id
    )
    .fetch_all(pool.get_ref())
    .await;

    match events {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}

pub async fn list_organization_events(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
) -> impl Responder {
    let organization_id = path.into_inner();

    let events = sqlx::query_as!(
        OperationalEventResponse,
        r#"
        SELECT
            id as "id!",
            organization_id as "organization_id!",
            actor_user_id,
            entity_type as "entity_type!",
            entity_id as "entity_id!",
            event_type as "event_type!",
            from_status,
            to_status,
            metadata as "metadata!",
            created_at as "created_at!"
        FROM operational_events
        WHERE organization_id = $1
        ORDER BY created_at DESC
        LIMIT 250
        "#,
        organization_id
    )
    .fetch_all(pool.get_ref())
    .await;

    match events {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}
