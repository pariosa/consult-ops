use crate::auth_context::AuthUser;
use crate::db::Db;
use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize)]
pub struct CreateMyOrganizationRequest {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct SetCurrentOrganizationRequest {
    pub organization_id: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct MyOrganizationResponse {
    pub organization_id: i64,
    pub name: String,
    pub slug: Option<String>,
    pub role: String,
    pub status: String,
    pub is_current: bool,
}

fn slugify(name: &str) -> String {
    let slug = name
        .trim()
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    if slug.is_empty() {
        format!("organization-{}", Utc::now().timestamp())
    } else {
        slug
    }
}

pub async fn list_my_organizations(db: web::Data<Db>, auth: AuthUser) -> impl Responder {
    let rows = sqlx::query_as::<_, MyOrganizationResponse>(
        r#"
        SELECT
            o.id AS organization_id,
            o.name AS name,
            o.slug AS slug,
            om.role AS role,
            om.status AS status,
            (u.current_organization_id = o.id) AS is_current
        FROM organization_members om
        JOIN organizations o ON o.id = om.organization_id
        JOIN users u ON u.id = om.user_id
        WHERE om.user_id = $1
          AND om.status = 'active'
        ORDER BY is_current DESC, o.name ASC
        "#,
    )
    .bind(auth.id)
    .fetch_all(db.pool.as_ref())
    .await;

    match rows {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn set_current_organization(
    db: web::Data<Db>,
    auth: AuthUser,
    payload: web::Json<SetCurrentOrganizationRequest>,
) -> impl Responder {
    let count = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)::BIGINT
        FROM organization_members
        WHERE user_id = $1
          AND organization_id = $2
          AND status = 'active'
        "#,
    )
    .bind(auth.id)
    .bind(payload.organization_id)
    .fetch_one(db.pool.as_ref())
    .await;

    let Ok(count) = count else {
        return HttpResponse::InternalServerError().body("Failed to check membership");
    };

    if count == 0 {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You are not a member of this organization"
        }));
    }

    let now = Utc::now().to_rfc3339();

    let result = sqlx::query(
        r#"
        UPDATE users
        SET current_organization_id = $1,
            updated_at = $2
        WHERE id = $3
        "#,
    )
    .bind(payload.organization_id)
    .bind(&now)
    .bind(auth.id)
    .execute(db.pool.as_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Current organization updated",
            "organization_id": payload.organization_id
        })),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn create_my_organization(
    db: web::Data<Db>,
    auth: AuthUser,
    payload: web::Json<CreateMyOrganizationRequest>,
) -> impl Responder {
    let name = payload.name.trim();

    if name.len() < 2 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Organization name is required"
        }));
    }

    let now = Utc::now().to_rfc3339();
    let base_slug = slugify(name);

    let mut tx = match db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    let organization_id = match sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO organizations (
            name,
            slug,
            created_by_user_id,
            created_at,
            updated_at
        )
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
        "#,
    )
    .bind(name)
    .bind(&base_slug)
    .bind(auth.id)
    .bind(&now)
    .bind(&now)
    .fetch_one(&mut *tx)
    .await
    {
        Ok(id) => id,
        Err(e) => {
            let _ = tx.rollback().await;
            return HttpResponse::BadRequest().body(e.to_string());
        }
    };

    if let Err(e) = sqlx::query(
        r#"
        INSERT INTO organization_members (
            organization_id,
            user_id,
            role,
            status,
            created_at,
            updated_at
        )
        VALUES ($1, $2, 'owner', 'active', $3, $4)
        ON CONFLICT(organization_id, user_id)
        DO UPDATE SET
            role = 'owner',
            status = 'active',
            updated_at = excluded.updated_at
        "#,
    )
    .bind(organization_id)
    .bind(auth.id)
    .bind(&now)
    .bind(&now)
    .execute(&mut *tx)
    .await
    {
        let _ = tx.rollback().await;
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    if let Err(e) = sqlx::query(
        r#"
        UPDATE users
        SET current_organization_id = $1,
            updated_at = $2
        WHERE id = $3
        "#,
    )
    .bind(organization_id)
    .bind(&now)
    .bind(auth.id)
    .execute(&mut *tx)
    .await
    {
        let _ = tx.rollback().await;
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    if let Err(e) = tx.commit().await {
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    HttpResponse::Created().json(serde_json::json!({
        "organization_id": organization_id,
        "name": name,
        "slug": base_slug,
        "role": "owner",
        "status": "active",
        "is_current": true
    }))
}
