use crate::auth_context::AuthUser;
use crate::db::Db;
use crate::models::project::{CreateProject, CreateProjectRequest, Project};
use crate::services::authz::{require_org_member, require_org_role};
use crate::services::event_service::EventService;
use actix_web::{HttpResponse, Responder, ResponseError, web};
use chrono::Utc;
use sqlx::SqlitePool;

async fn organization_id_for_client(db: &Db, client_id: i64) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(
        r#"
        SELECT organization_id
        FROM clients
        WHERE id = $1
        "#,
    )
    .bind(client_id)
    .fetch_one(db.pool.as_ref())
    .await
}

/// Global project listing should be platform-admin only.
/// Normal app usage should prefer `/api/organizations/{id}/projects`.
pub async fn get_projects(db: web::Data<Db>, auth: AuthUser) -> impl Responder {
    if auth.user_type != "admin" && auth.user_type != "super_admin" {
        return HttpResponse::Forbidden().body("Platform admin access required");
    }

    match Project::all(&db).await {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch projects")
        }
    }
}

pub async fn get_project_by_id(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let id = path.into_inner();

    let result = sqlx::query_as::<_, crate::models::project::Project>(
        r#"
        SELECT
            id,
            organization_id,
            client_id,
            name,
            start_date,
            end_date,
            description,
            created_at
        FROM projects
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(db.pool.as_ref())
    .await;

    match result {
        Ok(Some(project)) => {
            if let Err(err) =
                require_org_member(db.pool.as_ref(), auth.id, project.organization_id).await
            {
                return err.error_response();
            }

            HttpResponse::Ok().json(project)
        }

        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "message": format!("Project #{} not found", id)
        })),

        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "message": err.to_string()
        })),
    }
}

pub async fn create_project(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
    info: web::Json<CreateProjectRequest>,
) -> impl Responder {
    let organization_id = path.into_inner();
    let input = info.into_inner();

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

    let client_organization_id = match organization_id_for_client(&db, input.client_id).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid client_id"),
    };

    if client_organization_id != organization_id {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Client does not belong to this organization."
        }));
    }

    let new_project = CreateProject {
        organization_id,
        client_id: input.client_id,
        name: input.name,
        start_date: input.start_date.or_else(|| Some(Utc::now().to_rfc3339())),
        description: input.description,
        end_date: input.end_date,
    };

    match Project::create(&db, new_project).await {
        Ok(project) => {
            let _ = EventService::record_event(
                db.pool.as_ref(),
                project.organization_id,
                Some(auth.id),
                "project",
                project.id,
                "ProjectCreated",
                None,
                Some("created"),
                serde_json::json!({
                    "client_id": project.client_id,
                    "name": project.name
                }),
            )
            .await;

            HttpResponse::Created().json(project)
        }
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body("Failed to create project")
        }
    }
}
