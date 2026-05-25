use crate::auth_context::AuthUser;
use crate::db::Db;
use crate::models::client::CreateClient;
use crate::models::project::{CreateProject, CreateProjectRequest};
use crate::models::user::User;
use crate::models::{
    Client, Contract, Invoice, Organization, Payment, Project, UpdateOrganization,
};
use crate::services::authz::{require_org_member, require_org_role};
use actix_web::{HttpResponse, Responder, ResponseError, web};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize)]
pub struct CreateOrganizationMemberPayload {
    pub user_id: i64,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrganizationMemberPayload {
    pub role: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct OrganizationMemberRow {
    pub id: i64,
    pub organization_id: i64,
    pub user_id: i64,
    pub role: String,
    pub email: String,
    pub name: Option<String>,
    pub user_type: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AdminSummary {
    pub users: i64,
    pub organizations: i64,
    pub active_projects: i64,
    pub invoices_pending: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct ProjectPortalSummary {
    pub assigned_projects: i64,
    pub active_clients: i64,
    pub pending_invoices: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct ClientPortalSummary {
    pub open_projects: i64,
    pub contracts: i64,
    pub invoices_due: i64,
}

async fn organization_id_for_member(db: &Db, member_id: i64) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(
        r#"
        SELECT organization_id
        FROM organization_members
        WHERE id = $1
        "#,
    )
    .bind(member_id)
    .fetch_one(db.pool.as_ref())
    .await
}

pub async fn get_organization(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let organization_id = path.into_inner();

    if let Err(err) = require_org_member(db.pool.as_ref(), auth.id, organization_id).await {
        return err.error_response();
    }

    match Organization::find(&db, organization_id).await {
        Ok(Some(org)) => HttpResponse::Ok().json(org),
        Ok(None) => HttpResponse::NotFound().body("Organization not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn update_organization(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
    payload: web::Json<UpdateOrganization>,
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

    match Organization::update(&db, organization_id, payload.into_inner()).await {
        Ok(Some(org)) => HttpResponse::Ok().json(org),
        Ok(None) => HttpResponse::NotFound().body("Organization not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_organization_members(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let organization_id = path.into_inner();

    if let Err(err) = require_org_member(db.pool.as_ref(), auth.id, organization_id).await {
        return err.error_response();
    }

    let result = sqlx::query_as::<_, OrganizationMemberRow>(
        r#"
        SELECT
            om.id,
            om.organization_id,
            om.user_id,
            om.role,
            u.email,
            u.name,
            u.user_type,
            om.created_at,
            om.updated_at
        FROM organization_members om
        JOIN users u ON u.id = om.user_id
        WHERE om.organization_id = $1
        ORDER BY u.name ASC, u.email ASC
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

pub async fn create_organization_member(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
    payload: web::Json<CreateOrganizationMemberPayload>,
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

    let result = sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO organization_members (
            organization_id,
            user_id,
            role,
            status,
            created_at,
            updated_at
        )
        VALUES ($1, $2, $3, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        ON CONFLICT(organization_id, user_id)
        DO UPDATE SET
            role = excluded.role,
            status = 'active',
            updated_at = CURRENT_TIMESTAMP
        RETURNING id
        "#,
    )
    .bind(organization_id)
    .bind(payload.user_id)
    .bind(&payload.role)
    .fetch_one(db.pool.as_ref())
    .await;

    match result {
        Ok(id) => HttpResponse::Created().json(serde_json::json!({ "id": id })),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn update_organization_member(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
    payload: web::Json<UpdateOrganizationMemberPayload>,
) -> impl Responder {
    let member_id = path.into_inner();

    let organization_id = match organization_id_for_member(&db, member_id).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::NotFound().body("Organization member not found"),
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

    let result = sqlx::query(
        r#"
        UPDATE organization_members
        SET role = $1,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $2
        "#,
    )
    .bind(&payload.role)
    .bind(member_id)
    .execute(db.pool.as_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Organization member updated"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn delete_organization_member(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let member_id = path.into_inner();

    let organization_id = match organization_id_for_member(&db, member_id).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::NotFound().body("Organization member not found"),
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

    let result = sqlx::query(
        r#"
        UPDATE organization_members
        SET status = 'removed',
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $1
        "#,
    )
    .bind(member_id)
    .execute(db.pool.as_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Organization member removed"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_organization_clients(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let organization_id = path.into_inner();

    if let Err(err) = require_org_member(db.pool.as_ref(), auth.id, organization_id).await {
        return err.error_response();
    }

    match Client::for_organization(&db, organization_id).await {
        Ok(clients) => HttpResponse::Ok().json(clients),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn create_organization_client(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
    payload: web::Json<CreateClient>,
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

    let mut client = payload.into_inner();
    client.organization_id = organization_id;

    match Client::create(&db, client).await {
        Ok(client) => HttpResponse::Created().json(client),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_organization_projects(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let organization_id = path.into_inner();

    if let Err(err) = require_org_member(db.pool.as_ref(), auth.id, organization_id).await {
        return err.error_response();
    }

    match Project::for_organization(&db, organization_id).await {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn create_organization_project(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
    payload: web::Json<CreateProjectRequest>,
) -> impl Responder {
    let organization_id = path.into_inner();
    let input = payload.into_inner();

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

    let project = CreateProject {
        organization_id,
        client_id: input.client_id,
        name: input.name,
        start_date: input.start_date,
        end_date: input.end_date,
        description: input.description,
    };

    match Project::create(&db, project).await {
        Ok(project) => HttpResponse::Created().json(project),
        Err(err) => {
            eprintln!("DB error creating project: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn get_organization_contracts(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let organization_id = path.into_inner();

    if let Err(err) = require_org_member(db.pool.as_ref(), auth.id, organization_id).await {
        return err.error_response();
    }

    match Contract::for_organization(&db, organization_id).await {
        Ok(contracts) => HttpResponse::Ok().json(contracts),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_organization_invoices(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let organization_id = path.into_inner();

    if let Err(err) = require_org_member(db.pool.as_ref(), auth.id, organization_id).await {
        return err.error_response();
    }

    match Invoice::for_organization(db.pool.as_ref(), organization_id).await {
        Ok(invoices) => HttpResponse::Ok().json(invoices),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_organization_payments(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let organization_id = path.into_inner();

    if let Err(err) = require_org_member(db.pool.as_ref(), auth.id, organization_id).await {
        return err.error_response();
    }

    match Payment::for_organization(&db, organization_id).await {
        Ok(payments) => HttpResponse::Ok().json(payments),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_admin_summary(db: web::Data<Db>, auth: AuthUser) -> impl Responder {
    if auth.user_type != "admin" && auth.user_type != "super_admin" {
        return HttpResponse::Forbidden().body("Platform admin access required");
    }

    let result: sqlx::Result<AdminSummary> = sqlx::query_as::<_, AdminSummary>(
        r#"
        SELECT
            (SELECT COUNT(*) FROM users)::BIGINT AS users,
            (SELECT COUNT(*) FROM organizations)::BIGINT AS organizations,
            (SELECT COUNT(*) FROM projects)::BIGINT AS active_projects,
            (SELECT COUNT(*) FROM invoices WHERE lower(status) = 'pending')::BIGINT AS invoices_pending
        "#,
    )
    .fetch_one(db.pool.as_ref())
    .await;

    match result {
        Ok(summary) => HttpResponse::Ok().json(summary),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_project_portal_summary(db: web::Data<Db>, auth: AuthUser) -> impl Responder {
    let result: sqlx::Result<ProjectPortalSummary> = sqlx::query_as::<_, ProjectPortalSummary>(
        r#"
        SELECT
            (SELECT COUNT(*)
             FROM projects p
             JOIN organization_members om ON om.organization_id = p.organization_id
             WHERE om.user_id = $1 AND om.status = 'active')::BIGINT AS assigned_projects,

            (SELECT COUNT(DISTINCT p.client_id)
             FROM projects p
             JOIN organization_members om ON om.organization_id = p.organization_id
             WHERE om.user_id = $1 AND om.status = 'active')::BIGINT AS active_clients,

            (SELECT COUNT(*)
             FROM invoices i
             JOIN organization_members om ON om.organization_id = i.organization_id
             WHERE om.user_id = $1 AND om.status = 'active' AND lower(i.status) = 'pending')::BIGINT AS pending_invoices
        "#,
    )
    .bind(auth.id)
    .fetch_one(db.pool.as_ref())
    .await;

    match result {
        Ok(summary) => HttpResponse::Ok().json(summary),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_client_portal_summary(db: web::Data<Db>, auth: AuthUser) -> impl Responder {
    let result: sqlx::Result<ClientPortalSummary> = sqlx::query_as::<_, ClientPortalSummary>(
        r#"
        SELECT
            (SELECT COUNT(*)
             FROM projects p
             JOIN organization_members om ON om.organization_id = p.organization_id
             WHERE om.user_id = $1 AND om.status = 'active')::BIGINT AS open_projects,

            (SELECT COUNT(*)
             FROM contracts c
             JOIN organization_members om ON om.organization_id = c.organization_id
             WHERE om.user_id = $1 AND om.status = 'active')::BIGINT AS contracts,

            (SELECT COUNT(*)
             FROM invoices i
             JOIN organization_members om ON om.organization_id = i.organization_id
             WHERE om.user_id = $1 AND om.status = 'active' AND lower(i.status) = 'pending')::BIGINT AS invoices_due
        "#,
    )
    .bind(auth.id)
    .fetch_one(db.pool.as_ref())
    .await;

    match result {
        Ok(summary) => HttpResponse::Ok().json(summary),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_me(db: web::Data<Db>, auth_user: AuthUser) -> impl Responder {
    match User::find_by_id(&db, auth_user.id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_my_organization(db: web::Data<Db>, auth_user: AuthUser) -> impl Responder {
    let result = sqlx::query_as::<_, Organization>(
        r#"
        SELECT
            o.id,
            o.name,
            o.slug,
            o.created_by_user_id,
            o.created_at,
            o.updated_at
        FROM organizations o
        JOIN organization_members om ON om.organization_id = o.id
        JOIN users u ON u.id = om.user_id
        WHERE om.user_id = $1
          AND om.status = 'active'
          AND (
              u.current_organization_id = o.id
              OR u.current_organization_id IS NULL
          )
        ORDER BY
          CASE WHEN u.current_organization_id = o.id THEN 0 ELSE 1 END,
          o.id ASC
        LIMIT 1
        "#,
    )
    .bind(auth_user.id)
    .fetch_optional(db.pool.as_ref())
    .await;

    match result {
        Ok(Some(org)) => HttpResponse::Ok().json(org),
        Ok(None) => HttpResponse::NotFound().body("No organization found for user"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
