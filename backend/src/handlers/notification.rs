use actix_web::{HttpResponse, Responder, web};

use crate::auth_context::AuthUser;
use crate::db::Db;

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct NotificationResponse {
    pub id: i64,
    pub organization_id: i64,
    pub user_id: Option<i64>,
    pub recipient_email: Option<String>,
    pub notification_type: String,
    pub title: String,
    pub body: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<i64>,
    pub read_at: Option<String>,
    pub created_at: String,
}

pub async fn list_my_notifications(db: web::Data<Db>, auth: AuthUser) -> impl Responder {
    let result = sqlx::query_as::<_, NotificationResponse>(
        r#"
        SELECT
            id,
            organization_id,
            user_id,
            recipient_email,
            notification_type,
            title,
            body,
            entity_type,
            entity_id,
            read_at,
            created_at
        FROM notifications
        WHERE user_id = $1
           OR lower(recipient_email) = lower($2)
        ORDER BY created_at DESC
        LIMIT 50
        "#,
    )
    .bind(auth.id)
    .bind(&auth.email)
    .fetch_all(db.pool.as_ref())
    .await;

    match result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn mark_notification_read(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let notification_id = path.into_inner();

    let result = sqlx::query(
        r#"
        UPDATE notifications
        SET read_at = CURRENT_TIMESTAMP
        WHERE id = $1
          AND (
            user_id = $2
            OR lower(recipient_email) = lower($3)
          )
        "#,
    )
    .bind(notification_id)
    .bind(auth.id)
    .bind(&auth.email)
    .execute(db.pool.as_ref())
    .await;

    match result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                return HttpResponse::NotFound().json(serde_json::json!({
                    "error": "Notification not found"
                }));
            }

            HttpResponse::Ok().json(serde_json::json!({ "success": true }))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn mark_all_notifications_read(db: web::Data<Db>, auth: AuthUser) -> impl Responder {
    let result = sqlx::query(
        r#"
        UPDATE notifications
        SET read_at = CURRENT_TIMESTAMP
        WHERE read_at IS NULL
          AND (
            user_id = $1
            OR lower(recipient_email) = lower($2)
          )
        "#,
    )
    .bind(auth.id)
    .bind(&auth.email)
    .execute(db.pool.as_ref())
    .await;

    match result {
        Ok(result) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "updated_count": result.rows_affected()
        })),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
