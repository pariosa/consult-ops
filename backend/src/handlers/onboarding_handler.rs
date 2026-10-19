use crate::auth_context::AuthUser;
use crate::db::Db;
use crate::services::authz::require_org_role;
use actix_web::ResponseError;
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

#[derive(Debug, Serialize, FromRow)]
pub struct DiscoverableOrganization {
    pub id: i64,
    pub name: String,
    pub slug: Option<String>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct MyJoinRequest {
    pub id: i64,
    pub organization_id: i64,
    pub organization_name: String,
    pub status: String,
    pub created_at: chrono::DateTime<Utc>,
}

#[derive(Debug, FromRow)]
struct PendingJoinRequest {
    user_id: i64,
    status: String,
}

pub async fn discover_organizations(db: web::Data<Db>, _auth: AuthUser) -> impl Responder {
    let organizations = sqlx::query_as::<_, DiscoverableOrganization>(
        r#"
        SELECT id, name, slug
        FROM organizations
        WHERE visibility = 'public'
        ORDER BY name ASC, id ASC
        LIMIT 100
        "#,
    )
    .fetch_all(db.pool.as_ref())
    .await;

    match organizations {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(e) => {
            eprintln!("Organization discovery failed: {e}");
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to discover organizations"
            }))
        }
    }
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

pub async fn request_to_join_organization(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let organization_id = path.into_inner();

    let mut tx = match db.pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Lock the organization so concurrent requests see a consistent state.
    let visibility = sqlx::query_scalar::<_, String>(
        "SELECT visibility FROM organizations WHERE id = $1 FOR UPDATE",
    )
    .bind(organization_id)
    .fetch_optional(&mut *tx)
    .await;

    match visibility {
        Ok(Some(v)) if v == "public" => {}
        Ok(_) => {
            return HttpResponse::NotFound()
                .json(serde_json::json!({"error": "Organization not found"}));
        }
        Err(_) => return HttpResponse::InternalServerError().finish(),
    }

    let membership_exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM organization_members
            WHERE organization_id = $1
              AND user_id = $2
              AND status = 'active'
        )
        "#,
    )
    .bind(organization_id)
    .bind(auth.id)
    .fetch_one(&mut *tx)
    .await;

    match membership_exists {
        Ok(true) => {
            return HttpResponse::Conflict().json(serde_json::json!({"error": "Already a member"}));
        }
        Ok(false) => {}
        Err(_) => return HttpResponse::InternalServerError().finish(),
    }

    let inserted = sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO organization_join_requests
            (organization_id, user_id, requested_role, status)
        VALUES ($1, $2, 'member', 'pending')
        ON CONFLICT (organization_id, user_id)
            WHERE status = 'pending'
        DO NOTHING
        RETURNING id
        "#,
    )
    .bind(organization_id)
    .bind(auth.id)
    .fetch_optional(&mut *tx)
    .await;

    match inserted {
        Ok(Some(request_id)) => {
            if tx.commit().await.is_err() {
                return HttpResponse::InternalServerError().finish();
            }

            HttpResponse::Created().json(serde_json::json!({
                "id": request_id,
                "organization_id": organization_id,
                "status": "pending"
            }))
        }
        Ok(None) => HttpResponse::Conflict().json(serde_json::json!({
            "error": "You already have a pending request"
        })),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn list_my_join_requests(db: web::Data<Db>, auth: AuthUser) -> impl Responder {
    let rows = sqlx::query_as::<_, MyJoinRequest>(
        r#"
        SELECT
            jr.id,
            jr.organization_id,
            o.name AS organization_name,
            jr.status,
            jr.created_at
        FROM organization_join_requests jr
        JOIN organizations o ON o.id = jr.organization_id
        WHERE jr.user_id = $1
        ORDER BY jr.created_at DESC, jr.id DESC
        LIMIT 100
        "#,
    )
    .bind(auth.id)
    .fetch_all(db.pool.as_ref())
    .await;

    match rows {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(e) => {
            eprintln!("Failed to list join requests: {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(Debug, Serialize, FromRow)]
pub struct OrganizationJoinRequest {
    pub id: i64,
    pub organization_id: i64,
    pub user_id: i64,
    pub requested_role: String,
    pub status: String,
    pub created_at: chrono::DateTime<Utc>,
}

pub async fn list_organization_join_requests(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let organization_id = path.into_inner();

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

    let rows = sqlx::query_as::<_, OrganizationJoinRequest>(
        r#"
        SELECT id, organization_id, user_id,
               requested_role, status, created_at
        FROM organization_join_requests
        WHERE organization_id = $1
          AND status = 'pending'
        ORDER BY created_at ASC, id ASC
        LIMIT 100
        "#,
    )
    .bind(organization_id)
    .fetch_all(db.pool.as_ref())
    .await;

    match rows {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(e) => {
            eprintln!("Failed to list join requests: {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn decide_join_request(
    db: web::Data<Db>,
    auth: AuthUser,
    organization_id: i64,
    request_id: i64,
    approve: bool,
) -> HttpResponse {
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

    let mut tx = match db.pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let request = sqlx::query_as::<_, PendingJoinRequest>(
        r#"
        SELECT user_id, status
        FROM organization_join_requests
        WHERE id = $1 AND organization_id = $2
        FOR UPDATE
        "#,
    )
    .bind(request_id)
    .bind(organization_id)
    .fetch_optional(&mut *tx)
    .await;

    let request = match request {
        Ok(Some(request)) => request,
        Ok(None) => {
            return HttpResponse::NotFound()
                .json(serde_json::json!({"error": "Join request not found"}));
        }
        Err(e) => {
            eprintln!("Join request lookup failed: {e}");
            return HttpResponse::InternalServerError().finish();
        }
    };

    if request.status != "pending" {
        return HttpResponse::Conflict().json(serde_json::json!({
            "error": "This request has already been decided"
        }));
    }

    if approve {
        // Do not overwrite or reactivate any existing membership.
        let inserted = sqlx::query(
            r#"
            INSERT INTO organization_members (
                organization_id, user_id, role, status,
                created_at, updated_at
            )
            VALUES (
                $1, $2, 'member', 'active',
                $3, $3
            )
            ON CONFLICT (organization_id, user_id)
            DO NOTHING
            "#,
        )
        .bind(organization_id)
        .bind(request.user_id)
        .bind(Utc::now().to_rfc3339())
        .execute(&mut *tx)
        .await;

        match inserted {
            Ok(result) if result.rows_affected() == 1 => {}
            Ok(_) => {
                return HttpResponse::Conflict().json(serde_json::json!({
                    "error": "User already has a membership record; manual review required"
                }));
            }
            Err(e) => {
                eprintln!("Membership activation failed: {e}");
                return HttpResponse::InternalServerError().finish();
            }
        }
    }

    let new_status = if approve { "approved" } else { "rejected" };

    let result = sqlx::query(
        r#"
        UPDATE organization_join_requests
        SET status = $1,
            reviewed_by_user_id = $2,
            reviewed_at = NOW(),
            updated_at = NOW()
        WHERE id = $3
          AND organization_id = $4
          AND status = 'pending'
        "#,
    )
    .bind(new_status)
    .bind(auth.id)
    .bind(request_id)
    .bind(organization_id)
    .execute(&mut *tx)
    .await;

    match result {
        Ok(result) if result.rows_affected() == 1 => {}
        Ok(_) => return HttpResponse::Conflict().finish(),
        Err(e) => {
            eprintln!("Join request decision failed: {e}");
            return HttpResponse::InternalServerError().finish();
        }
    }

    if let Err(e) = tx.commit().await {
        eprintln!("Join request commit failed: {e}");
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().json(serde_json::json!({
        "id": request_id,
        "organization_id": organization_id,
        "user_id": request.user_id,
        "status": new_status
    }))
}

pub async fn approve_organization_join_request(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<(i64, i64)>,
) -> impl Responder {
    let (organization_id, request_id) = path.into_inner();
    decide_join_request(db, auth, organization_id, request_id, true).await
}

pub async fn reject_organization_join_request(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<(i64, i64)>,
) -> impl Responder {
    let (organization_id, request_id) = path.into_inner();
    decide_join_request(db, auth, organization_id, request_id, false).await
}
