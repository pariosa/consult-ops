use actix_web::{HttpResponse, Responder, web};

use crate::auth::permissions::can_manage_platform;
use crate::auth_context::AuthUser;
use crate::db::Db;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PlatformOrganization {
    pub id: i64,
    pub name: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PlatformUser {
    pub id: i64,
    pub email: Option<String>,
    pub name: Option<String>,
    pub user_type: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PlatformOrganizationMember {
    pub id: i64,
    pub organization_id: Option<i64>,
    pub user_id: Option<i64>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub user_type: Option<String>,
    pub role: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePlatformOrganization {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatePlatformUser {
    pub email: String,
    pub name: Option<String>,
    pub user_type: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct AssignUserToOrganization {
    pub user_id: i64,
    pub role: String,
}

pub async fn list_platform_organizations(db: web::Data<Db>, auth: AuthUser) -> impl Responder {
    if !can_manage_platform(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Platform admin access required."
        }));
    }

    let result = sqlx::query_as!(
        PlatformOrganization,
        r#"
    SELECT id, name, created_at, updated_at
    FROM organizations
    ORDER BY created_at DESC
    "#
    )
    .fetch_all(db.pool.as_ref())
    .await;

    match result {
        Ok(orgs) => HttpResponse::Ok().json(orgs),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn create_platform_organization(
    db: web::Data<Db>,
    auth: AuthUser,
    payload: web::Json<CreatePlatformOrganization>,
) -> impl Responder {
    if !can_manage_platform(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Platform admin access required."
        }));
    }

    let result = sqlx::query_as!(
        PlatformOrganization,
        r#"
    INSERT INTO organizations (name, created_at, updated_at)
    VALUES (?, datetime('now'), datetime('now'))
    RETURNING id, name, created_at, updated_at
    "#,
        payload.name
    )
    .fetch_one(db.pool.as_ref())
    .await;

    match result {
        Ok(org) => HttpResponse::Created().json(org),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn list_platform_users(db: web::Data<Db>, auth: AuthUser) -> impl Responder {
    if !can_manage_platform(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Platform admin access required."
        }));
    }

    let result = sqlx::query_as!(
        PlatformUser,
        r#"
        SELECT id, email, name, user_type, created_at, updated_at
        FROM users
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(db.pool.as_ref())
    .await;

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn create_platform_user(
    db: web::Data<Db>,
    auth: AuthUser,
    payload: web::Json<CreatePlatformUser>,
) -> impl Responder {
    if !can_manage_platform(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Platform admin access required."
        }));
    }

    let password_hash = match crate::auth::hash_password(&payload.password) {
        Ok(hash) => hash,
        Err(err) => return HttpResponse::InternalServerError().body(err),
    };
    let result = sqlx::query_as!(
        PlatformUser,
        r#"
        INSERT INTO users (email, password_hash, name, user_type, created_at, updated_at)
        VALUES (?, ?, ?, ?, datetime('now'), datetime('now'))
        RETURNING id, email, name, user_type, created_at, updated_at
        "#,
        payload.email,
        password_hash,
        payload.name,
        payload.user_type
    )
    .fetch_one(db.pool.as_ref())
    .await;

    match result {
        Ok(user) => HttpResponse::Created().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn list_platform_organization_members(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    if !can_manage_platform(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Platform admin access required."
        }));
    }

    let organization_id = path.into_inner();

    let result = sqlx::query_as::<_, PlatformOrganizationMember>(
        r#"
        SELECT
            om.id,
            om.organization_id,
            om.user_id,
            u.email,
            u.name,
            u.user_type,
            om.role,
            om.status,
            om.created_at,
            om.updated_at
        FROM organization_members om
        JOIN users u ON u.id = om.user_id
        WHERE om.organization_id = ?
        ORDER BY om.created_at DESC
        "#,
    )
    .bind(organization_id)
    .fetch_all(db.pool.as_ref())
    .await;

    match result {
        Ok(members) => HttpResponse::Ok().json(members),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
pub async fn assign_platform_user_to_organization(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
    payload: web::Json<AssignUserToOrganization>,
) -> impl Responder {
    if !can_manage_platform(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Platform admin access required."
        }));
    }

    let organization_id = path.into_inner();

    let result = sqlx::query(
        r#"
    INSERT INTO organization_members (
        organization_id,
        user_id,
        role,
        status,
        created_at,
        updated_at
    )
    VALUES (?, ?, ?, 'active', datetime('now'), datetime('now'))
    ON CONFLICT(organization_id, user_id)
    DO UPDATE SET
        role = excluded.role,
        status = 'active',
        updated_at = datetime('now')
    "#,
    )
    .bind(organization_id)
    .bind(payload.user_id)
    .bind(&payload.role)
    .execute(db.pool.as_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "success": true
        })),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
