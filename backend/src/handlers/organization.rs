// backend/src/handlers/organization.rs

use crate::models::client::CreateClient;
use crate::models::project::{CreateProject, CreateProjectRequest};
use crate::models::user::User;
use crate::models::{
    Client, Contract, Invoice, Organization, Payment, Project, UpdateOrganization,
};
use crate::{auth_context::AuthUser, db::Db};
use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize)]
pub struct UpdateOrganizationPayload {
    pub name: Option<String>,
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

pub async fn get_organization(db: web::Data<Db>, path: web::Path<i64>) -> impl Responder {
    let organization_id = path.into_inner();

    match Organization::find(&db, organization_id).await {
        Ok(Some(org)) => HttpResponse::Ok().json(org),
        Ok(None) => HttpResponse::NotFound().body("Organization not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn update_organization(
    db: web::Data<Db>,
    path: web::Path<i64>,
    payload: web::Json<UpdateOrganization>,
) -> impl Responder {
    let organization_id = path.into_inner();

    match Organization::update(&db, organization_id, payload.into_inner()).await {
        Ok(Some(org)) => HttpResponse::Ok().json(org),
        Ok(None) => HttpResponse::NotFound().body("Organization not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_organization_members(db: web::Data<Db>, path: web::Path<i64>) -> impl Responder {
    let organization_id = path.into_inner();

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
        WHERE om.organization_id = ?
        ORDER BY u.name ASC, u.email ASC
        "#,
    )
    .bind(organization_id)
    .fetch_all(&*db.pool)
    .await;

    match result {
        Ok(members) => HttpResponse::Ok().json(members),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn create_organization_member(
    db: web::Data<Db>,
    path: web::Path<i64>,
    payload: web::Json<CreateOrganizationMemberPayload>,
) -> impl Responder {
    let organization_id = path.into_inner();

    let result = sqlx::query(
        r#"
        INSERT INTO organization_members
        (organization_id, user_id, role, created_at, updated_at)
        VALUES (?, ?, ?, datetime('now'), datetime('now'))
        "#,
    )
    .bind(organization_id)
    .bind(payload.user_id)
    .bind(&payload.role)
    .execute(&*db.pool)
    .await;

    match result {
        Ok(record) => HttpResponse::Created().json(record.last_insert_rowid()),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn update_organization_member(
    db: web::Data<Db>,
    path: web::Path<i64>,
    payload: web::Json<UpdateOrganizationMemberPayload>,
) -> impl Responder {
    let member_id = path.into_inner();

    let result = sqlx::query(
        r#"
        UPDATE organization_members
        SET role = ?, updated_at = datetime('now')
        WHERE id = ?
        "#,
    )
    .bind(&payload.role)
    .bind(member_id)
    .execute(&*db.pool)
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Organization member updated"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn delete_organization_member(db: web::Data<Db>, path: web::Path<i64>) -> impl Responder {
    let member_id = path.into_inner();

    let result = sqlx::query(
        r#"
        DELETE FROM organization_members
        WHERE id = ?
        "#,
    )
    .bind(member_id)
    .execute(&*db.pool)
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Organization member deleted"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_organization_clients(db: web::Data<Db>, path: web::Path<i64>) -> impl Responder {
    let organization_id = path.into_inner();

    let result = sqlx::query_as::<_, Client>(
        r#"
        SELECT
            id,
            organization_id,
            name,
            email,
            tax_id,
            phone,
            company_name,
            address,
            city,
            state,
            zip,
            country,
            created_at,
            updated_at
        FROM clients
        WHERE organization_id = ?
        ORDER BY name ASC
        "#,
    )
    .bind(organization_id)
    .fetch_all(&*db.pool)
    .await;

    match result {
        Ok(clients) => HttpResponse::Ok().json(clients),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_organization_projects(db: web::Data<Db>, path: web::Path<i64>) -> impl Responder {
    let organization_id = path.into_inner();

    let result = sqlx::query_as::<_, Project>(
        r#"
        SELECT
            id,
            organization_id,
            client_id,
            name,
            start_date,
            description,
            end_date,
            created_at,
            updated_at
        FROM projects
        WHERE organization_id = ?
        ORDER BY created_at DESC
        "#,
    )
    .bind(organization_id)
    .fetch_all(&*db.pool)
    .await;

    match result {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_admin_summary(db: web::Data<Db>) -> impl Responder {
    let result: sqlx::Result<AdminSummary> = sqlx::query_as::<_, AdminSummary>(
        r#"
        SELECT
            (SELECT COUNT(*) FROM users) as users,
            (SELECT COUNT(*) FROM organizations) as organizations,
            (SELECT COUNT(*) FROM projects) as active_projects,
            (SELECT COUNT(*) FROM invoices WHERE status = 'Pending') as invoices_pending
        "#,
    )
    .fetch_one(&*db.pool)
    .await;

    match result {
        Ok(summary) => HttpResponse::Ok().json(summary),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
pub async fn create_organization_project(
    db: web::Data<Db>,
    path: web::Path<i64>,
    payload: web::Json<CreateProjectRequest>,
) -> impl Responder {
    let organization_id = path.into_inner();
    let input = payload.into_inner();

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
pub async fn create_organization_client(
    db: web::Data<Db>,
    path: web::Path<i64>,
    payload: web::Json<CreateClient>,
) -> impl Responder {
    let organization_id = path.into_inner();
    let mut client = payload.into_inner();

    client.organization_id = organization_id;

    match Client::create(&db, client).await {
        Ok(client) => HttpResponse::Created().json(client),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_project_portal_summary(db: web::Data<Db>) -> impl Responder {
    let result: sqlx::Result<ProjectPortalSummary> = sqlx::query_as::<_, ProjectPortalSummary>(
        r#"
            SELECT
                (SELECT COUNT(*) FROM projects) as assigned_projects,
                (SELECT COUNT(DISTINCT client_id) FROM projects) as active_clients,
                (SELECT COUNT(*) FROM invoices WHERE status = 'Pending') as pending_invoices
            "#,
    )
    .fetch_one(&*db.pool)
    .await;

    match result {
        Ok(summary) => HttpResponse::Ok().json(summary),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_client_portal_summary(db: web::Data<Db>) -> impl Responder {
    let result: sqlx::Result<ClientPortalSummary> = sqlx::query_as::<_, ClientPortalSummary>(
        r#"
            SELECT
                (SELECT COUNT(*) FROM projects) as open_projects,
                (SELECT COUNT(*) FROM contracts) as contracts,
                (SELECT COUNT(*) FROM invoices WHERE status = 'Pending') as invoices_due
            "#,
    )
    .fetch_one(&*db.pool)
    .await;

    match result {
        Ok(summary) => HttpResponse::Ok().json(summary),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
pub async fn get_me(db: web::Data<Db>, auth_user: AuthUser) -> impl Responder {
    let result: sqlx::Result<Option<User>> = sqlx::query_as::<_, User>(
        r#"
        SELECT id, email, password_hash, name, user_type, created_at, updated_at
        FROM users
        WHERE id = ?
        "#,
    )
    .bind(auth_user.id)
    .fetch_optional(&*db.pool)
    .await;

    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_my_organization(db: web::Data<Db>, auth_user: AuthUser) -> impl Responder {
    let result = sqlx::query_as::<_, Organization>(
        r#"
        SELECT
            o.id,
            o.name,
            o.created_at,
            o.updated_at
        FROM organizations o
        JOIN organization_members om ON om.organization_id = o.id
        WHERE om.user_id = ?
        ORDER BY o.id ASC
        LIMIT 1
        "#,
    )
    .bind(auth_user.id)
    .fetch_optional(&*db.pool)
    .await;

    match result {
        Ok(Some(org)) => HttpResponse::Ok().json(org),
        Ok(None) => HttpResponse::NotFound().body("No organization found for user"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_organization_contracts(db: web::Data<Db>, path: web::Path<i64>) -> impl Responder {
    let organization_id = path.into_inner();

    let result: sqlx::Result<Vec<Contract>> = sqlx::query_as::<_, Contract>(
        r#"
        SELECT
            id,
            organization_id,
            project_id,
            title,
            status,
            signed_at,
            start_date,
            end_date,
            value,
            currency,
            terms,
            notes,
            external_id,
            created_at,
            updated_at
        FROM contracts
        WHERE organization_id = ?
        ORDER BY created_at DESC
        "#,
    )
    .bind(organization_id)
    .fetch_all(&*db.pool)
    .await;

    match result {
        Ok(contracts) => HttpResponse::Ok().json(contracts),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_organization_invoices(db: web::Data<Db>, path: web::Path<i64>) -> impl Responder {
    let organization_id = path.into_inner();

    let result: sqlx::Result<Vec<Invoice>> = sqlx::query_as::<_, Invoice>(
        r#"
        SELECT
            id,
            organization_id,
            contract_id,
            invoice_number,
            status,
            issued_at,
            due_date,
            subtotal,
            tax,
            total,
            currency,
            notes,
            created_at,
            updated_at
        FROM invoices
        WHERE organization_id = ?
        ORDER BY created_at DESC
        "#,
    )
    .bind(organization_id)
    .fetch_all(&*db.pool)
    .await;

    match result {
        Ok(invoices) => HttpResponse::Ok().json(invoices),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_organization_payments(db: web::Data<Db>, path: web::Path<i64>) -> impl Responder {
    let organization_id = path.into_inner();

    let result: sqlx::Result<Vec<Payment>> = sqlx::query_as::<_, Payment>(
        r#"
        SELECT
            id,
            organization_id,
            invoice_id,
            paid_at,
            amount,
            currency,
            method,
            reference,
            notes,
            created_at
        FROM payments
        WHERE organization_id = ?
        ORDER BY created_at DESC
        "#,
    )
    .bind(organization_id)
    .fetch_all(&*db.pool)
    .await;

    match result {
        Ok(payments) => HttpResponse::Ok().json(payments),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
