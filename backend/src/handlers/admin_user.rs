use crate::auth_context::AuthUser;
use crate::db::Db;
use crate::models::user::User;
use crate::services::authz::require_platform_admin;
use actix_web::{HttpResponse, Responder, ResponseError, web};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct UserMembershipResponse {
    pub organization_id: i64,
    pub organization_name: String,
    pub role: String,
    pub status: String,
}

pub async fn disable_user(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    if let Err(err) = require_platform_admin(&auth) {
        return err.error_response();
    }

    let user_id = path.into_inner();

    match sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET disabled_at = CURRENT_TIMESTAMP,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $1
        RETURNING id, email, password_hash, name, user_type, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .fetch_one(&*db.pool)
    .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body("Failed to disable user")
        }
    }
}

pub async fn enable_user(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    if let Err(err) = require_platform_admin(&auth) {
        return err.error_response();
    }

    let user_id = path.into_inner();

    match sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET disabled_at = NULL,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $1
        RETURNING id, email, password_hash, name, user_type, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .fetch_one(&*db.pool)
    .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body("Failed to enable user")
        }
    }
}

pub async fn get_user_memberships(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    if let Err(err) = require_platform_admin(&auth) {
        return err.error_response();
    }

    let user_id = path.into_inner();

    match sqlx::query_as::<_, UserMembershipResponse>(
        r#"
        SELECT
            om.organization_id,
            o.name AS organization_name,
            om.role,
            om.status
        FROM organization_members om
        JOIN organizations o ON o.id = om.organization_id
        WHERE om.user_id = $1
        ORDER BY o.name ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(&*db.pool)
    .await
    {
        Ok(memberships) => HttpResponse::Ok().json(memberships),
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch memberships")
        }
    }
}

pub async fn force_password_reset(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    if let Err(err) = require_platform_admin(&auth) {
        return err.error_response();
    }

    let user_id = path.into_inner();
    let token = Uuid::new_v4().to_string();
    let token_hash = token.clone(); // TODO: hash this before production.
    let expires_at = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(2))
        .unwrap()
        .to_rfc3339();

    match sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO password_reset_tokens (
            user_id,
            token_hash,
            expires_at,
            created_at
        )
        VALUES ($1, $2, $3, CURRENT_TIMESTAMP)
        RETURNING id
        "#,
    )
    .bind(user_id)
    .bind(&token_hash)
    .bind(expires_at)
    .fetch_one(&*db.pool)
    .await
    {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Password reset token created.",
            "reset_token": token
        })),
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body("Failed to create password reset token")
        }
    }
}

pub async fn revoke_user_sessions(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    if let Err(err) = require_platform_admin(&auth) {
        return err.error_response();
    }

    let user_id = path.into_inner();

    match sqlx::query(
        r#"
        UPDATE auth_sessions
        SET revoked_at = CURRENT_TIMESTAMP
        WHERE user_id = $1
          AND revoked_at IS NULL
        "#,
    )
    .bind(user_id)
    .execute(&*db.pool)
    .await
    {
        Ok(result) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Active sessions revoked.",
            "revoked_count": result.rows_affected()
        })),
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body("Failed to revoke sessions")
        }
    }
}
